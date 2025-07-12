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

delegate_keyboard!(@<Message: 'static + Send + Sync> State<Message>);

impl<Message: 'static + Send + Sync> KeyboardHandler for State<Message> {
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
        self.client.send_no_result(Request::Distribute {
            event: Event::KeyboardEnter,
            id: Some(surface.id()),
        });
    }

    fn leave(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &WlKeyboard,
        surface: &WlSurface,
        _: u32,
    ) {
        self.client.send_no_result(Request::Distribute {
            event: Event::KeyboardLeave,
            id: Some(surface.id()),
        });
    }

    fn press_key(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _: &WlKeyboard,
        _: u32,
        event: KeyEvent,
    ) {
        self.client.send_no_result(Request::Distribute {
            event: Event::KeyPressed { key: event.keysym },
            id: None,
        });
    }

    fn release_key(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &WlKeyboard,
        _: u32,
        event: KeyEvent,
    ) {
        self.client.send_no_result(Request::Distribute {
            event: Event::KeyReleased { key: event.keysym },
            id: None,
        });
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
        self.client.send_no_result(Request::Distribute {
            event: Event::KeyModifiersChanged { modifiers },
            id: None,
        });
    }
}
