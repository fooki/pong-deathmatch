use bincode::{serialize};
use crate::net_messages::{ClientMsg};
use laminar::{Packet, Socket};
use std::time::{Instant};
use std::net::SocketAddr;
use crate::server_state::{StateUpdate};
use crate::server_network::ServerNet;

pub fn send_client_msg(src: Option<SocketAddr>, dst: SocketAddr, msg: ClientMsg) {
    let mut socket = match src {
        Some(addr) => Socket::bind(addr),
        None => Socket::bind_any(),
    }.unwrap();

    let msg = serialize(&msg).unwrap();
    let packet = Packet::unreliable(dst, msg);
    socket.send(packet).unwrap();
    socket.manual_poll(Instant::now());
}

pub fn assert_state_update(state: StateUpdate, new_state: &str) {
    let name = format!{"{:?}", state};
    assert!(name.contains(new_state));
}

// This might not be the best thing
// from https://github.com/rust-lang-nursery/rust-cookbook/issues/500
pub fn working_server_net() -> ServerNet {
    for port in 20000..65535 {
        let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
        if let Ok(server_net) = ServerNet::bind(addr) {
            return server_net;
        }
    }
    panic!("Can't open a scket");
}
