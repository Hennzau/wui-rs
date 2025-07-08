use std::collections::{HashMap, HashSet, hash_map::ValuesMut};

use eyre::OptionExt;
use wayland_backend::client::ObjectId;

use crate::prelude::*;

pub(crate) struct Views {
    pub(crate) registering: bool,

    pub(crate) accessed: HashSet<ObjectId>,

    pub(crate) lut: HashMap<String, ObjectId>,
    pub(crate) namespaces: HashMap<ObjectId, String>,

    pub(crate) container: HashMap<ObjectId, View>,
}

impl Views {
    pub(crate) fn new() -> Self {
        Self {
            registering: false,
            accessed: HashSet::new(),

            lut: HashMap::new(),
            namespaces: HashMap::new(),
            container: HashMap::new(),
        }
    }

    pub(crate) fn activate_cache(&mut self) {
        self.registering = true;
    }

    pub(crate) fn deactivate(&mut self) {
        self.registering = false;
    }

    pub(crate) fn cache(&mut self, id: Option<ObjectId>) -> Option<ObjectId> {
        if self.registering {
            if let Some(id) = &id {
                self.accessed.insert(id.clone());
            }
        }

        id
    }

    pub(crate) fn get_mut(&mut self, id: &ObjectId) -> Option<&mut View> {
        self.container.get_mut(id)
    }

    pub(crate) fn get_id(&mut self, namespace: &String) -> Option<ObjectId> {
        let id = self.cache(self.lut.get(namespace).cloned());

        id
    }

    pub(crate) fn insert(&mut self, view: View) {
        let namespace = view.namespace();
        let id = view.id();

        self.cache(Some(id.clone()));

        self.lut.insert(namespace.clone(), id.clone());
        self.namespaces.insert(id.clone(), namespace.clone());

        self.container.insert(id, view);
    }

    pub(crate) fn remove(&mut self, id: &ObjectId) -> Result<()> {
        self.accessed.remove(id);

        let namespace = self
            .namespaces
            .remove(id)
            .ok_or_eyre(format!("View {} should have had a namespace", id))?;

        self.lut.remove(&namespace);

        let view = self
            .container
            .remove(id)
            .ok_or_eyre(format!("View {} should have had a view handle", id))?;

        view.close();

        Ok(())
    }

    pub(crate) fn values_mut(&mut self) -> ValuesMut<'_, ObjectId, View> {
        self.container.values_mut()
    }

    pub(crate) fn garbage(&mut self) -> Result<()> {
        self.deactivate();

        let keys: Vec<ObjectId> = self.container.keys().cloned().collect();

        for view in &keys {
            if !self.accessed.contains(view) {
                self.remove(view)?;
            }
        }

        self.accessed.clear();

        Ok(())
    }
}
