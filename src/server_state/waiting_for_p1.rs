use core::fmt::Debug;
use crate::net_messages::{ClientMsg};
use crate::server_network::ServerNet;
use super::waiting_for_p2::WaitingForP2;
use super::{ServerState, new_state, StateUpdate};

#[derive(Debug)]
pub struct WaitingForP1 { }

impl WaitingForP1 {
    pub fn new() -> Self {
        Self { }
    }
}

impl ServerState for WaitingForP1 {
    fn update(&mut self, net: &mut ServerNet) -> StateUpdate {
        loop {
            if let Some((p1_addr, ClientMsg::Hi)) = net.poll() {
                return new_state(Box::new(WaitingForP2::new(p1_addr)));
            }
            self.sleep();
        }
    }

    fn sleep_time_ms(&self) -> u64 {
        500
    }
}


#[cfg(test)]
mod waiting_for_p1_tests {
    use crate::test_helper::*;
    use super::*;

    #[test]
    fn test_waiting_for_p1_transitions_to_waiting_to_p2() {
        let mut net = working_server_net();
        let mut state = WaitingForP1::new();

        send_client_msg(None, net.addr, ClientMsg::Hi);

        assert_state_update(state.update(&mut net), "WaitingForP2");
    }
}
