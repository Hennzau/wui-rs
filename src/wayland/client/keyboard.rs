use crate::prelude::*;

use smithay_client_toolkit::{
    delegate_keyboard,
    reexports::client::{
        Connection, QueueHandle,
        protocol::{wl_keyboard::WlKeyboard, wl_surface::WlSurface},
    },
    seat::keyboard::{KeyEvent, KeyboardHandler, Keysym, Modifiers, RawModifiers},
};
use wayland_client::Proxy;

delegate_keyboard!(@<Message: 'static + Send + Sync> Client<Message>);

impl<Message: 'static + Send + Sync> KeyboardHandler for Client<Message> {
    fn enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &wayland_client::protocol::wl_keyboard::WlKeyboard,
        surface: &WlSurface,
        _serial: u32,
        _raw: &[u32],
        _keysyms: &[Keysym],
    ) {
        self.handle_event(
            WidgetId::Widget(surface.id()),
            WaylandWidgetEvent::WidgetEvent(Event::KeyboardEntered),
        );
    }

    fn leave(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &WlKeyboard,
        surface: &WlSurface,
        _: u32,
    ) {
        self.handle_event(
            WidgetId::Widget(surface.id()),
            WaylandWidgetEvent::WidgetEvent(Event::KeyboardLeaved),
        );
    }

    fn press_key(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _: &WlKeyboard,
        _: u32,
        event: KeyEvent,
    ) {
        self.handle_event(
            WidgetId::AllWidgets,
            WaylandWidgetEvent::WidgetEvent(Event::KeyPressed {
                key: event.raw_code,
            }),
        );
    }

    fn repeat_key(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &WlKeyboard,
        _serial: u32,
        event: KeyEvent,
    ) {
        self.handle_event(
            WidgetId::AllWidgets,
            WaylandWidgetEvent::WidgetEvent(Event::KeyPressed {
                key: event.raw_code,
            }),
        );
    }

    fn release_key(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &WlKeyboard,
        _: u32,
        event: KeyEvent,
    ) {
        self.handle_event(
            WidgetId::AllWidgets,
            WaylandWidgetEvent::WidgetEvent(Event::KeyReleased {
                key: event.raw_code,
            }),
        );
    }

    fn update_modifiers(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &WlKeyboard,
        _serial: u32,
        modifiers: Modifiers,
        _raw_modifiers: RawModifiers,
        _layout: u32,
    ) {
        self.handle_event(
            WidgetId::AllWidgets,
            WaylandWidgetEvent::WidgetEvent(Event::KeyModifiersChanged {
                ctrl: modifiers.ctrl,
                alt: modifiers.alt,
                shift: modifiers.shift,
                caps_lock: modifiers.caps_lock,
                logo: modifiers.logo,
                num_lock: modifiers.num_lock,
            }),
        );
    }
}
