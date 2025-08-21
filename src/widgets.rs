use std::collections::HashMap;

use eyre::OptionExt;
use vello::{Renderer, RendererOptions, util::RenderContext};
use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::WindowId};

use crate::*;

pub struct Widgets<Message> {
    pub(crate) renderers: HashMap<usize, Renderer>,

    pub(crate) ctx: RenderContext,

    pub(crate) widgets: HashMap<WindowId, SurfaceWidget<Message>>,
    pub(crate) devices: HashMap<WindowId, usize>,

    pub(crate) lut: HashMap<String, WindowId>,
}

impl<Message: 'static> Widgets<Message> {
    pub(crate) fn new() -> Self {
        Self {
            ctx: RenderContext::new(),
            renderers: HashMap::new(),
            widgets: HashMap::new(),
            devices: HashMap::new(),
            lut: HashMap::new(),
        }
    }

    pub(crate) fn reconciliate(
        &mut self,
        roots: Vec<RootWidget<Message>>,
        event_loop: &dyn ActiveEventLoop,
    ) -> Result<()> {
        let lut = self.lut.drain().collect::<HashMap<_, _>>();
        let mut widgets = self.widgets.drain().collect::<HashMap<_, _>>();

        for root in roots {
            let label = root.label.clone();

            if !lut.contains_key(&label) {
                let widget = SurfaceWidget::new(root, event_loop, &mut self.ctx)?;
                let id = widget.id;
                let dev_id = widget.dev_id();

                self.widgets.insert(id, widget);
                self.lut.insert(label.clone(), id);
                self.devices.insert(id, dev_id);

                self.renderers.entry(dev_id).or_insert(Renderer::new(
                    &self.ctx.devices[dev_id].device,
                    RendererOptions::default(),
                )?);
            } else {
                let id = *lut.get(&label).unwrap();

                if let Some(mut widget) = widgets.remove(&id) {
                    widget.child = match widget.child {
                        Some(child) => match root.child {
                            Some(root_child) => Some(child.merge(root_child)),
                            None => None,
                        },
                        None => root.child,
                    };

                    // TODO reconfigure surface
                    self.widgets.insert(id, widget);
                    self.lut.insert(label, id);
                }
            }
        }

        Ok(())
    }

    pub(crate) fn redraw(&mut self, id: WindowId) -> Result<()> {
        if let Some(widget) = self.widgets.get_mut(&id) {
            let renderer = self
                .renderers
                .get_mut(&widget.dev_id())
                .ok_or_eyre(format!(
                    "No renderer found for device id {}",
                    widget.dev_id()
                ))?;

            let device = self
                .ctx
                .devices
                .get(widget.dev_id())
                .ok_or_eyre(format!("No device found for id {}", widget.dev_id()))?;

            widget.draw(renderer, device)?;
        }

        Ok(())
    }

    pub(crate) fn handle_event(
        &mut self,
        msg: &mut Vec<Message>,
        id: WindowId,
        event: WindowEvent,
    ) -> Result<()> {
        if let Some(widget) = self.widgets.get_mut(&id) {
            match event {
                WindowEvent::SurfaceResized(size) => {
                    if let Some(widget) = self.widgets.get_mut(&id) {
                        widget.surface.config.alpha_mode =
                            vello::wgpu::CompositeAlphaMode::PreMultiplied;

                        self.ctx
                            .resize_surface(&mut widget.surface, size.width, size.height);
                    }
                }
                WindowEvent::CloseRequested => {
                    if let Some(widget) = self.widgets.remove(&id) {
                        self.lut.remove(&widget.label);
                        self.devices.remove(&id);
                    }
                }
                WindowEvent::KeyboardInput {
                    device_id: _,
                    event,
                    is_synthetic: _,
                } => match event.state {
                    winit::event::ElementState::Pressed => {
                        widget.handle_event(msg, Event::KeyPressed(event.logical_key))?;
                    }
                    winit::event::ElementState::Released => {
                        widget.handle_event(msg, Event::KeyReleased(event.logical_key))?;
                    }
                },
                WindowEvent::ModifiersChanged(modifiers) => {
                    widget.handle_event(msg, Event::KeyModifiersChanged(modifiers.state()))?;
                }
                WindowEvent::PointerMoved {
                    device_id: _,
                    position,
                    primary: _,
                    source: _,
                } => {
                    widget.handle_event(
                        msg,
                        Event::PointerMoved(Point::new(position.x, position.y)),
                    )?;
                }
                WindowEvent::PointerButton {
                    device_id: _,
                    state,
                    position,
                    primary: _,
                    button,
                } => match state {
                    winit::event::ElementState::Pressed => {
                        widget.handle_event(
                            msg,
                            Event::PointerPressed {
                                position: Point::new(position.x, position.y),
                                button,
                            },
                        )?;
                    }
                    winit::event::ElementState::Released => {
                        widget.handle_event(
                            msg,
                            Event::PointerReleased {
                                position: Point::new(position.x, position.y),
                                button,
                            },
                        )?;
                    }
                },
                WindowEvent::MouseWheel {
                    device_id: _,
                    delta,
                    phase: _,
                } => {
                    widget.handle_event(
                        msg,
                        Event::PointerScrolled(match delta {
                            winit::event::MouseScrollDelta::LineDelta(x, y) => {
                                MouseScrollDelta::LineDelta(x, y)
                            }
                            winit::event::MouseScrollDelta::PixelDelta(delta) => {
                                MouseScrollDelta::PixelDelta(Vec2::new(delta.x, delta.y))
                            }
                        }),
                    )?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub(crate) fn request_redraw(&self) {
        for widget in self.widgets.values() {
            widget.window.request_redraw();
        }
    }
}
