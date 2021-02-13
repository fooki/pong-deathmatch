use bincode::{deserialize, serialize};
use crate::net_messages::{ClientMsg, ServerMsg};
use laminar::{Packet, Socket, SocketEvent, ErrorKind};
use std::net::SocketAddr;
use std::time::{Instant};

pub struct ServerNet {
    pub addr: SocketAddr,
    socket: Socket,
}

impl ServerNet {
    pub fn bind(addr: SocketAddr) -> Result<ServerNet, ErrorKind> {
        let socket = Socket::bind(addr)?;
        Ok(Self { addr, socket })
    }

    pub fn poll(&mut self) -> Option<(SocketAddr, ClientMsg)> {
        // The server doesn't run network communication in another thread. It
        // probably should though.

        self.socket.manual_poll(Instant::now());

        let pkt = self.socket.recv()?;
        let (addr, msg) = match pkt {
            SocketEvent::Packet(pkt) => {
                let msg = deserialize::<ClientMsg>(pkt.payload()).unwrap();
                (pkt.addr(), msg)
            }
            SocketEvent::Timeout(addr) => (addr, ClientMsg::Timeout),
            SocketEvent::Connect(addr) => (addr, ClientMsg::Connect),
            SocketEvent::Disconnect(addr) => (addr, ClientMsg::Disconnect),
        };
        Some((addr, msg))
    }

    pub fn send(&mut self, dst: SocketAddr, msg: ServerMsg) -> Result<(), ErrorKind> {
        let msg = serialize(&msg).unwrap();
        let packet = Packet::reliable_ordered(dst, msg, None);

        self.socket.send(packet)?;
        self.socket.manual_poll(Instant::now());

        Ok(())
    }
}
