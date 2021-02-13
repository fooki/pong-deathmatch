mod running;
mod waiting_for_p1;
mod waiting_for_p2;

pub use running::Running;
pub use waiting_for_p1::WaitingForP1;
pub use waiting_for_p2::WaitingForP2;
use crate::server_network::ServerNet;
use laminar::ErrorKind;
use std::thread;
use std::time::{Duration};

pub type StateUpdate = Result<Option<Box<dyn ServerState>>, ErrorKind>;

pub fn new_state(state: Box<dyn ServerState>) -> StateUpdate {
    Ok(Some(state))
}

pub trait ServerState: std::fmt::Debug {

    // Runs until there is a new state to return
    fn update(&mut self, net: &mut ServerNet) -> StateUpdate;

    // How long can we wait between loop iterations in a state? For example,
    // while waiting for player 2, we don't need to spam player 1 with info.
    fn sleep_time_ms(&self) -> u64;

    fn sleep(&mut self) {
        let duration = Duration::from_millis(self.sleep_time_ms());
        thread::sleep(duration);
    }

}
