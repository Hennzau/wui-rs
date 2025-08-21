use crate::*;

pub fn transform_event(event: Event, transform: Affine) -> Event {
    match event {
        Event::PointerMoved(position) => Event::PointerMoved(
            transform
                .pre_translate(position.to_vec2())
                .translation()
                .to_point(),
        ),
        Event::PointerPressed { position, button } => Event::PointerPressed {
            position: transform
                .pre_translate(position.to_vec2())
                .translation()
                .to_point(),
            button,
        },
        Event::PointerReleased { position, button } => Event::PointerReleased {
            position: transform
                .pre_translate(position.to_vec2())
                .translation()
                .to_point(),
            button,
        },
        _ => event,
    }
}

pub fn transform_handle_event<Message: 'static>(
    child: &mut Element<Message>,
    msg: &mut Vec<Message>,
    event: Event,
    transform: Affine,
) -> Result<()> {
    match transform_event(event, transform) {
        Event::PointerMoved(position) => {
            if child.size().to_rect().contains(position) {
                child.handle_event(msg, Event::PointerMoved(position))?;
                if !child.hovered() {
                    child.handle_event(msg, Event::PointerEntered)?;
                }
            } else {
                if child.hovered() {
                    child.handle_event(msg, Event::PointerLeft)?;
                }
            }

            Ok(())
        }
        Event::PointerPressed { position, button } => {
            if child.size().to_rect().contains(position) {
                child.handle_event(msg, Event::PointerPressed { position, button })?;
            }

            Ok(())
        }
        Event::PointerReleased { position, button } => {
            if child.size().to_rect().contains(position) {
                child.handle_event(msg, Event::PointerReleased { position, button })?;
            }

            Ok(())
        }
        Event::PointerEntered => Ok(()), // Don't propagate PointerEntered
        Event::PointerLeft => Ok(()),    // Don't propagate PointerLeft
        event => child.handle_event(msg, event),
    }
}
