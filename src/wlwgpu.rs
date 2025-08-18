use smithay_client_toolkit::reexports::client::EventQueue;

pub use eyre::{Report, Result};

mod event;
pub use event::*;

mod surface;
pub use surface::*;

mod event_loop;
pub(crate) use event_loop::*;

mod client;
pub(crate) use client::*;

mod shell;
pub use shell::*;

mod wgpu;
pub use wgpu::*;

mod wl;
pub use wl::*;

mod scene;
pub use scene::*;

pub struct WlWgpu {
    pub(crate) event_queue: EventQueue<Client>,
    pub(crate) shell: Shell,
}

impl WlWgpu {
    pub fn run(
        mut self,
        event_loop: impl FnMut(&mut Shell, Event) -> Result<()> + 'static,
    ) -> Result<()> {
        let mut client = Client::new(self.shell, event_loop.event_loop());

        loop {
            if !client.shell.running {
                break;
            }

            if let Err(e) = self.event_queue.blocking_dispatch(&mut client) {
                println!("Error {}", e);
            }
        }

        Ok(())
    }
}

pub fn wlwgpu() -> Result<WlWgpu> {
    let wgpu = Wgpu::new();
    let (wl, event_queue) = Wl::new()?;

    let shell = Shell::new(wl, wgpu, true);

    let wlwgpu = WlWgpu { event_queue, shell };

    Ok(wlwgpu)
}
