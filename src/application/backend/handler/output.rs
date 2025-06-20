use crate::application::Backend;

use smithay_client_toolkit::{
    delegate_output,
    output::{OutputHandler, OutputState},
    reexports::client::{Connection, QueueHandle, protocol::wl_output::WlOutput},
};

delegate_output!(Backend);

impl OutputHandler for Backend {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _output: WlOutput) {}

    fn update_output(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _output: WlOutput) {}

    fn output_destroyed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _output: WlOutput) {
    }
}
