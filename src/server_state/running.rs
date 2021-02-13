use core::fmt::Debug;
use crate::net_messages::{ClientMsg, ServerMsg, PlayerOrder};
use crate::pong_state::{PongState, PlayerMovement};
use crate::server_network::ServerNet;
use laminar::ErrorKind;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use super::waiting_for_p1::WaitingForP1;
use super::{ServerState, new_state, StateUpdate};

use crate::game_constants::{
    MS_PER_UPDATE,
    MS_PER_PING,
    MS_PER_STATE_BROADCAST,
};


#[derive(Debug)]
pub struct Running {
    // used to communicating with the different players
    p1: SocketAddr,
    p2: SocketAddr,

    // Keeps track of what move a player is doing during one game update. They
    // will reset between updates so the play will have to keep sending movement
    // updates.
    p1_move: Option<PlayerMovement>,
    p2_move: Option<PlayerMovement>,

    pong_state: PongState,

    // Keeps track of when its time to ping the clients
    last_ping: Instant,

    // Keeps track of when its time to send pong state to clients
    last_state_broadcast: Instant,
}

impl Running {
    pub fn new(p1: SocketAddr, p2: SocketAddr) -> Self {
        let pong_state = PongState::new();
        let last_ping = Instant::now();
        let last_state_broadcast = Instant::now();
        let p1_move = None;
        let p2_move = None;

        Self { p1, p2, p1_move, p2_move, pong_state, last_ping, last_state_broadcast }
    }

    fn maybe_ping_clients(&mut self, net: &mut ServerNet) -> Result<(), ErrorKind> {
        // Is it time for another ping?
        if Instant::now() - self.last_ping >= Duration::from_millis(MS_PER_PING) {
            net.send(self.p1, ServerMsg::Ping)?;
            net.send(self.p2, ServerMsg::Ping)?;

            self.last_ping = Instant::now();
        }
        Ok(())
    }

    fn maybe_send_pong_state(&mut self, net: &mut ServerNet) -> Result<(), ErrorKind> {
        // Is it time for state broadcast?
        let duration_since_broadcast = Instant::now() - self.last_state_broadcast;
        if duration_since_broadcast >= Duration::from_millis(MS_PER_STATE_BROADCAST) {
            net.send(self.p1, ServerMsg::State(self.pong_state))?;
            net.send(self.p2, ServerMsg::State(self.pong_state))?;

            self.last_state_broadcast = Instant::now();
        }
        Ok(())
    }
}

impl ServerState for Running {
    fn update(&mut self, net: &mut ServerNet) -> StateUpdate {
        // Tell the clients its time to start. Technically not necessary since
        // clients will show whatever pong state they have.
        net.send(self.p1, ServerMsg::Start(PlayerOrder::P1))?;
        net.send(self.p2, ServerMsg::Start(PlayerOrder::P2))?;

        loop {
            while let Some((addr, event)) = net.poll() {
                match event {
                    ClientMsg::MoveUp => {
                        if addr == self.p1 {
                            self.p1_move = Some(PlayerMovement::Up);
                        } else {
                            self.p2_move = Some(PlayerMovement::Up);
                        }
                    }

                    ClientMsg::MoveDown => {
                        if addr == self.p1 {
                            self.p1_move = Some(PlayerMovement::Down);
                        } else {
                            self.p2_move = Some(PlayerMovement::Down);
                        }
                    }

                    ClientMsg::Timeout | ClientMsg::Disconnect => {
                        // Any timeout will kill the game and the server will be
                        // in its initial state again.

                        net.send(self.p1, ServerMsg::Abort)?;
                        net.send(self.p2, ServerMsg::Abort)?;
                        return new_state(Box::new(WaitingForP1::new()));
                    }
                    _ => {}
                }
            }

            self.pong_state.tick(self.p1_move, self.p2_move);
            self.p1_move = None;
            self.p2_move = None;

            self.maybe_ping_clients(net)?;
            self.maybe_send_pong_state(net)?;
            self.sleep();
        }
    }

    fn sleep_time_ms(&self) -> u64 {
        MS_PER_UPDATE
    }
}
