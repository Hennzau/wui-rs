use vello::Scene;
use winit::event::MouseButton;

use crate::*;

pub struct Keyboard<Message> {
    pub(crate) child: Element<Message>,

    pub(crate) selected: bool,

    pub(crate) on_press: Option<Box<dyn Fn(Key) -> Message + 'static + Send + Sync>>,
    pub(crate) on_release: Option<Box<dyn Fn(Key) -> Message + 'static + Send + Sync>>,
    pub(crate) on_modifier_change:
        Option<Box<dyn Fn(ModifiersState) -> Message + 'static + Send + Sync>>,
}

impl<Message: 'static + Send + Sync> Widget<Message> for Keyboard<Message> {
    fn size(&self) -> Size {
        self.child.size()
    }

    fn handle_event(&mut self, msg: &mut Vec<Message>, event: Event) -> Result<()> {
        match &event {
            Event::KeyPressed(key) => {
                if let Some(on_press) = &self.on_press {
                    msg.push(on_press(key.clone()));
                }
            }
            Event::KeyReleased(key) => {
                if let Some(on_release) = &self.on_release {
                    msg.push(on_release(key.clone()));
                }
            }
            Event::KeyModifiersChanged(modifiers) => {
                if let Some(on_modifier_change) = &self.on_modifier_change {
                    msg.push(on_modifier_change(modifiers.clone()));
                }
            }
            Event::PointerPressed {
                position: _,
                button,
            } => {
                if button.mouse_button() == MouseButton::Left {
                    self.selected = true;
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
                element.child = self.child.merge(element.child);

                Element { widget: element }
            }
            Err(element) => element,
        }
    }
}

impl<Message> Keyboard<Message> {}

// pub fn keyboard<Message>(child: impl IntoElement<Message>) -> Keyboard<Message> {
//     Keyboard {
//         child: child.into_element(),
//         on_press: None,
//         on_release: None,
//         on_modifier_change: None,
//     }
// }
