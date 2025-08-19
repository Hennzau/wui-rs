use std::collections::HashMap;

use crate::*;

mod widget;
pub use widget::*;

mod root;
pub use root::*;

mod wlwgpu;
pub(crate) use wlwgpu::*;

pub struct Widgets<Message> {
    pub(crate) widgets: HashMap<SurfaceId, WlWgpuWidget<Message>>,
    pub(crate) lut: HashMap<String, SurfaceId>,
}

impl<Message: 'static> Widgets<Message> {
    pub(crate) fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            lut: HashMap::new(),
        }
    }

    pub(crate) fn reconciliate(
        &mut self,
        elements: Vec<RootWidget<Message>>,
        shell: &mut Shell<Message>,
    ) -> Result<()> {
        let lut = self.lut.drain().collect::<HashMap<_, _>>();
        let mut widgets = self.widgets.drain().collect::<HashMap<_, _>>();

        for element in elements {
            let label = element.label.clone();

            if !lut.contains_key(&label) {
                let widget = WlWgpuWidget::new(element, shell)?;
                let id = widget.id.clone();

                self.widgets.insert(id.clone(), widget);
                self.lut.insert(label, id);
            } else {
                let id = lut.get(&label).unwrap().clone();
                if let Some(mut widget) = widgets.remove(&id) {
                    widget.child = element.child;

                    // TODO reconfigure surface
                    self.widgets.insert(id.clone(), widget);
                    self.lut.insert(label, id);
                }
            }
        }

        Ok(())
    }

    pub(crate) fn handle_event(
        &mut self,
        msg: &mut Vec<Message>,
        shell: &mut Shell<Message>,
        event: Event,
    ) -> Result<()> {
        if let Some(id) = &event.id {
            if event.kind == EventKind::Close {
                if let Some(mut widget) = self.widgets.remove(id) {
                    widget.handle_event(msg, shell, event.kind)?;
                    self.lut.remove(&widget.label);

                    shell.destroy_surface(id);
                }
            } else {
                if let Some(widget) = self.widgets.get_mut(id) {
                    widget.handle_event(msg, shell, event.kind)?;
                }
            }
        } else {
            if event.kind == EventKind::Close {
                for (id, mut widget) in self.widgets.drain() {
                    widget.handle_event(msg, shell, event.kind.clone())?;
                    self.lut.remove(&widget.label);

                    shell.destroy_surface(&id);
                }
            } else {
                for widget in self.widgets.values_mut() {
                    widget.handle_event(msg, shell, event.kind.clone())?;
                }
            }
        }

        Ok(())
    }

    pub(crate) fn draw(&mut self, shell: &mut Shell<Message>) -> Result<()> {
        for widget in self.widgets.values_mut() {
            widget.draw(shell)?;
        }

        Ok(())
    }
}
