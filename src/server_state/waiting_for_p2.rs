use core::fmt::Debug;
use crate::net_messages::{ClientMsg, ServerMsg};
use crate::server_network::ServerNet;
use std::net::SocketAddr;
use super::running::Running;
use super::waiting_for_p1::WaitingForP1;
use super::{ServerState, new_state, StateUpdate};

#[derive(Debug)]
pub struct WaitingForP2 {
    pub p1: SocketAddr,
}

impl WaitingForP2 {
    pub fn new(p1: SocketAddr) -> Self {
        Self { p1 }
    }
}

impl ServerState for WaitingForP2 {
    fn update(&mut self, net: &mut ServerNet) -> StateUpdate {
        loop {
            net.send(self.p1, ServerMsg::Ping)?;
            let msg = net.poll();

            if let Some((p2, ClientMsg::Hi)) = msg {

                // Avoid double connects
                if self.p1 != p2 {
                    return new_state(Box::new(Running::new(self.p1, p2)));
                }
            }

            if let Some((_, ClientMsg::Disconnect)) = msg {
                return new_state(Box::new(WaitingForP1::new()));
            }

            self.sleep();
        }
    }

    fn sleep_time_ms(&self) -> u64 {
        500
    }
}

#[cfg(test)]
mod waiting_for_p2_tests {
    use std::net::SocketAddr;
    use crate::test_helper::*;
    use super::*;

    #[test]
    fn test_waiting_for_p2_transitions_to_running_on_new_connect() {
        let p1_addr: SocketAddr = "127.0.0.1:45456".parse().unwrap();
        let mut net = working_server_net();
        let mut state = WaitingForP2::new(p1_addr);

        send_client_msg(None, net.addr, ClientMsg::Hi);

        assert_state_update(state.update(&mut net), "Running");
    }

    #[test]
    fn test_waiting_for_p2_does_not_transition_if_same_client_connects_twice() {
        let p1_addr: SocketAddr = "127.0.0.1:45456".parse().unwrap();
        let mut net = working_server_net();
        let mut state = WaitingForP2::new(p1_addr);

        // Same user says hi again, shouldn't trigger Running state.
        send_client_msg(Some(p1_addr), net.addr, ClientMsg::Hi);

        // Need this to avoid looping forever
        send_client_msg(Some(p1_addr), net.addr, ClientMsg::Disconnect);

        assert_state_update(state.update(&mut net), "WaitingForP1");
    }

    #[test]
    fn test_waiting_for_p2_transitions_to_waiting_to_p1_on_disconnect() {
        let p1_addr: SocketAddr = "127.0.0.1:45456".parse().unwrap();
        let mut net = working_server_net();
        let mut state = WaitingForP2::new(p1_addr);

        send_client_msg(None,net.addr, ClientMsg::Disconnect);
        let res = state.update(&mut net);

        assert_state_update(res, "WaitingForP1");

    }
}
