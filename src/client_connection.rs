use bincode::{deserialize, serialize};
use crate::net_messages::{ClientMsg, ServerMsg};
use crossbeam_channel::{Sender, Receiver};
use laminar::{Packet, Socket, SocketEvent};
use std::net::SocketAddr;
use std::thread;

#[derive(Debug)]
pub enum ConnectionError {
    FailedToSend(String)
}

pub struct ClientConnection {
    server_addr: SocketAddr,

    // channel ends for sending/receiving data to/from a socket
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
}

impl ClientConnection {
    pub fn connect(server_addr: &str) -> Result<Self, ConnectionError> {
        let server_addr: SocketAddr = server_addr.parse().unwrap();

        let socket = Socket::bind_any().unwrap();
        let sender = socket.get_packet_sender();
        let receiver = socket.get_event_receiver();

        // Network communication is run in another thread to not mess up client
        // updating/drawing rates.
        Self::poll_in_separate_thread(socket);

        let mut connection = Self { server_addr, sender, receiver };
        connection.greet_server()?;
        Ok(connection)
    }

    pub fn poll_in_separate_thread(mut socket: Socket) {
        thread::spawn(move || socket.start_polling());
    }

    pub fn greet_server(&mut self) -> Result<(), ConnectionError> {
        self.send(ClientMsg::Hi)
    }

    pub fn send_move_up(&mut self) -> Result<(), ConnectionError> {
        self.send(ClientMsg::MoveUp)
    }

    pub fn send_move_down(&mut self) -> Result<(), ConnectionError> {
        self.send(ClientMsg::MoveDown)
    }

    pub fn pong(&mut self) -> Result<(), ConnectionError> {
        self.send(ClientMsg::Pong)
    }

    fn send(&mut self, msg: ClientMsg) -> Result<(), ConnectionError> {
        let msg = serialize(&msg).unwrap();
        let packet = Packet::reliable_ordered(self.server_addr, msg, None);
        self.sender.
            send(packet).
            map_err(|_| ConnectionError::FailedToSend(String::from("Could not send")))
    }

    pub fn receive(&mut self) -> Option<ServerMsg> {
        // This is cheating, error is treated as a nonexistant message.
        let pkt = self.receiver.try_recv().ok()?;

        match pkt {
            SocketEvent::Packet(pkt) => {
                deserialize::<ServerMsg>(pkt.payload()).ok()
            }

            SocketEvent::Timeout(_) => {
                println!("Timeout");
                Some(ServerMsg::Timeout)
            }

            SocketEvent::Connect(_) => {
                println!("Connected");
                Some(ServerMsg::Connect)
            }

            SocketEvent::Disconnect(_) => {
                println!("Disconnected");
                Some(ServerMsg::Disconnect)
            }
        }
    }
}

#[cfg(test)]
mod client_connection_tests {
    use super::*;

    #[test]
    fn test_connect_attempts_to_greet_server() {
        ClientConnection::connect("127.0.0.1:64646").unwrap();
    }
}
