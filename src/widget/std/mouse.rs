use vello::Scene;

use crate::*;

pub struct Mouse<Message> {
    pub(crate) child: Element<Message>,

    pub(crate) label: String,

    pub(crate) hovered: bool,

    pub(crate) on_enter: Option<Box<dyn Fn() -> Message + 'static + Send + Sync>>,
    pub(crate) on_leave: Option<Box<dyn Fn() -> Message + 'static + Send + Sync>>,

    pub(crate) on_move: Option<Box<dyn Fn(Point) -> Option<Message> + 'static + Send + Sync>>,
    pub(crate) on_press:
        Option<Box<dyn Fn(ButtonSource) -> Option<Message> + 'static + Send + Sync>>,
    pub(crate) on_release:
        Option<Box<dyn Fn(ButtonSource) -> Option<Message> + 'static + Send + Sync>>,
    pub(crate) on_scroll:
        Option<Box<dyn Fn(MouseScrollDelta) -> Option<Message> + 'static + Send + Sync>>,
}

impl<Message: 'static + Send + Sync> Widget<Message> for Mouse<Message> {
    fn size(&self) -> Size {
        self.child.size()
    }

    fn active(&self) -> bool {
        self.hovered
    }

    fn handle_event(&mut self, msg: &mut Vec<Message>, event: Event) -> Result<()> {
        match event {
            Event::PointerEntered => {
                self.hovered = true;
                if let Some(on_enter) = &self.on_enter {
                    msg.push(on_enter());
                }
            }
            Event::PointerLeft => {
                println!("Pointer left");
                self.hovered = false;
                if let Some(on_leave) = &self.on_leave {
                    msg.push(on_leave());
                }
            }
            Event::PointerMoved(position) => {
                if self.hovered {
                    if let Some(on_move) = &self.on_move {
                        if let Some(message) = on_move(position) {
                            msg.push(message);
                        }
                    }
                }
            }
            Event::PointerPressed {
                position: _,
                button,
            } => {
                if self.hovered {
                    if let Some(on_press) = &self.on_press {
                        if let Some(message) = on_press(button) {
                            msg.push(message);
                        }
                    }
                }
            }
            Event::PointerReleased {
                position: _,
                button,
            } => {
                if self.hovered {
                    if let Some(on_release) = &self.on_release {
                        if let Some(message) = on_release(button) {
                            msg.push(message);
                        }
                    }
                }
            }
            Event::PointerScrolled(delta) => {
                if self.hovered {
                    if let Some(on_scroll) = &self.on_scroll {
                        if let Some(message) = on_scroll(delta) {
                            msg.push(message);
                        }
                    }
                }
            }
            _ => {}
        }

        self.child.handle_event(msg, event)
    }

    fn draw(&self, scene: &mut Scene, transform: Affine) -> Result<()> {
        self.child.widget.draw(scene, transform)
    }

    fn merge(self: Box<Self>, element: Element<Message>) -> Element<Message> {
        match element.downcast::<Self>() {
            Ok(mut element) => {
                println!(
                    "Mouse {} {} {} <- Mouse {} {} {}",
                    element.label,
                    element.size(),
                    element.hovered,
                    self.label,
                    self.size(),
                    self.hovered
                );

                element.hovered = self.hovered;

                element.child = self.child.merge(element.child);

                Element { widget: element }
            }
            Err(element) => {
                println!("oiazjd");
                element
            }
        }
    }
}

impl<Message> Mouse<Message> {
    pub fn on_enter(mut self, on_enter: impl Fn() -> Message + 'static + Send + Sync) -> Self {
        self.on_enter = Some(Box::new(on_enter));
        self
    }

    pub fn on_leave(mut self, on_leave: impl Fn() -> Message + 'static + Send + Sync) -> Self {
        self.on_leave = Some(Box::new(on_leave));
        self
    }

    pub fn on_move(
        mut self,
        on_move: impl Fn(Point) -> Option<Message> + 'static + Send + Sync,
    ) -> Self {
        self.on_move = Some(Box::new(on_move));
        self
    }

    pub fn on_press(
        mut self,
        on_press: impl Fn(ButtonSource) -> Option<Message> + 'static + Send + Sync,
    ) -> Self {
        self.on_press = Some(Box::new(on_press));
        self
    }

    pub fn on_release(
        mut self,
        on_release: impl Fn(ButtonSource) -> Option<Message> + 'static + Send + Sync,
    ) -> Self {
        self.on_release = Some(Box::new(on_release));
        self
    }

    pub fn on_scroll(
        mut self,
        on_scroll: impl Fn(MouseScrollDelta) -> Option<Message> + 'static + Send + Sync,
    ) -> Self {
        self.on_scroll = Some(Box::new(on_scroll));
        self
    }
}

pub fn mouse<Message>(
    label: impl Into<String>,
    child: impl IntoElement<Message>,
) -> Mouse<Message> {
    Mouse {
        child: child.into_element(),
        label: label.into(),
        hovered: false,

        on_enter: None,
        on_leave: None,
        on_move: None,
        on_press: None,
        on_release: None,
        on_scroll: None,
    }
}
