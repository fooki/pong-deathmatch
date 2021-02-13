use crate::server_network::ServerNet;
use crate::server_state::{ServerState, WaitingForP1};
use laminar::{ErrorKind};
use std::net::SocketAddr;

pub fn run(my_addr: &str) ->Result<(), ErrorKind> {

    let addr: SocketAddr = my_addr.parse().unwrap();
    let net = ServerNet::bind(addr)?;
    let initial_state = Box::new(WaitingForP1::new());

    let mut server = Server::new(net, initial_state);

    println!("State: {:?}", server.state);

    loop {
        server.update()?;
    }
}

struct Server {
    net: ServerNet,
    state: Box<dyn ServerState>,
}

impl Server {
    fn new(net: ServerNet, initial_state: Box<dyn ServerState>) -> Self {
        Self { net, state: initial_state }
    }

    // Runs the server within one state until the server changes state.
    fn update(&mut self) -> Result<(), ErrorKind> {
        if let Some(new_state) = self.state.update(&mut self.net)? {
            println!("State: {:?}", &new_state);
            self.state = new_state;
        }
        Ok(())
    }
}
