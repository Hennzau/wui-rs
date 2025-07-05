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

delegate_keyboard!(State);

impl KeyboardHandler for State {
    fn enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &wayland_client::protocol::wl_keyboard::WlKeyboard,
        surface: &WlSurface,
        _serial: u32,
        _raw: &[u32],
        keysyms: &[Keysym],
    ) {
        if let Err(e) = self.sender.send(StateEvent {
            event: ViewEvent::KeyboardEnter(keysyms.to_vec()),
            view_id: Some(surface.id()),
        }) {
            eprintln!("Failed to send keyboard enter event: {}", e);
        }
    }

    fn leave(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &WlKeyboard,
        surface: &WlSurface,
        _: u32,
    ) {
        if let Err(e) = self.sender.send(StateEvent {
            event: ViewEvent::KeyboardLeave,
            view_id: Some(surface.id()),
        }) {
            eprintln!("Failed to send keyboard leave event: {}", e);
        }
    }

    fn press_key(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _: &WlKeyboard,
        _: u32,
        event: KeyEvent,
    ) {
        if let Err(e) = self.sender.send(StateEvent {
            event: ViewEvent::KeyPressed(event),
            view_id: None,
        }) {
            eprintln!("Failed to send key press event: {}", e);
        }
    }

    fn release_key(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &WlKeyboard,
        _: u32,
        event: KeyEvent,
    ) {
        if let Err(e) = self.sender.send(StateEvent {
            event: ViewEvent::KeyReleased(event),
            view_id: None,
        }) {
            eprintln!("Failed to send key release event: {}", e);
        }
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
        if let Err(e) = self.sender.send(StateEvent {
            event: ViewEvent::KeyModifiersChanged(modifiers),
            view_id: None,
        }) {
            eprintln!("Failed to send key modifiers change event: {}", e);
        }
    }
}
