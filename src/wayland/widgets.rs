use std::collections::HashMap;

use wayland_backend::client::ObjectId;

use crate::prelude::*;

pub(crate) struct WaylandElement<Message> {
    pub(crate) element: Element<Message>,
    pub(crate) surface: Surface,
}

impl<Message> WaylandElement<Message>
where
    Message: 'static + Send + Sync,
{
    pub(crate) fn id(&self) -> ObjectId {
        self.surface.id()
    }

    pub(crate) fn label(&self) -> Option<Label> {
        self.element.label()
    }

    pub(crate) fn destroy(&self) {
        self.surface.destroy();
    }
}

impl<Message: 'static + Send + Sync> WaylandElement<Message> {
    pub(crate) fn new(protocol: &Protocol<Message>, element: Element<Message>) -> Self {
        let surface = Surface::new(protocol, &element);

        Self { element, surface }
    }

    pub(crate) fn with_surface(surface: Surface, element: Element<Message>) -> Self {
        Self { element, surface }
    }

    pub(crate) fn on_event(
        &mut self,
        event: WaylandWidgetEvent,
        msg: Sender<Message>,

        renderer: &mut Renderer,
    ) -> Result<()> {
        match event {
            WaylandWidgetEvent::WidgetEvent(event) => {
                self.element.on_event(event, msg)?;
            }
            WaylandWidgetEvent::Draw => {
                self.element.draw(renderer)?;
            }
            WaylandWidgetEvent::Configure { width, height } => {
                renderer.configure(&self.surface, width, height);
            }
            WaylandWidgetEvent::Close => unreachable!(),
        }

        Ok(())
    }
}

pub(crate) struct WaylandElements<Message> {
    pub(crate) elements: HashMap<ObjectId, WaylandElement<Message>>,

    pub(crate) lookup_id: HashMap<Label, ObjectId>,
    pub(crate) lookup_label: HashMap<ObjectId, Label>,
}

impl<Message: 'static + Send + Sync> WaylandElements<Message> {
    pub(crate) fn new() -> Self {
        Self {
            elements: HashMap::new(),
            lookup_id: HashMap::new(),
            lookup_label: HashMap::new(),
        }
    }

    pub(crate) fn extract(&mut self, label: &Label) -> Option<WaylandElement<Message>> {
        if let Some(id) = self.lookup_id.remove(label) {
            self.lookup_label.remove(&id);
            self.elements.remove(&id)
        } else {
            None
        }
    }

    pub(crate) fn add(&mut self, element: WaylandElement<Message>) {
        let id = element.id();
        let label = element.label().unwrap_or_else(|| {
            tracing::warn!("Element without label added: {:?}", id);

            Label::default()
        });

        self.lookup_id.insert(label.clone(), id.clone());
        self.lookup_label.insert(id.clone(), label);
        self.elements.insert(id, element);
    }

    pub(crate) fn destroy(&mut self, label: &Label) {
        if let Some(id) = self.lookup_id.remove(label) {
            self.lookup_label.remove(&id);

            if let Some(element) = self.elements.remove(&id) {
                element.destroy();
            }
        }
    }

    pub(crate) fn remove_by_id(&mut self, id: &ObjectId) {
        if let Some(label) = self.lookup_label.remove(id) {
            self.destroy(&label);
        }
    }

    pub(crate) fn remove_all(&mut self) {
        for id in self.lookup_id.values() {
            if let Some(element) = self.elements.remove(id) {
                element.destroy();
            }
        }
        self.lookup_id.clear();
        self.lookup_label.clear();
    }

    pub(crate) fn on_event(
        &mut self,
        id: WidgetId,
        event: WaylandWidgetEvent,
        msg: Sender<Message>,

        renderer: &mut Renderer,
    ) -> Result<()> {
        match id {
            WidgetId::AllWidgets => {
                self.remove_all();
            }
            WidgetId::Widget(id) => {
                if let Some(element) = self.elements.get_mut(&id) {
                    match event {
                        WaylandWidgetEvent::Close => {
                            self.remove_by_id(&id);
                        }
                        event => {
                            element.on_event(event, msg, renderer)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
