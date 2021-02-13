mod client;
mod client_connection;
mod game_constants;
mod net_messages;
mod pong_state;
mod server;
mod server_network;
mod server_state;

#[cfg(test)]
mod test_helper;

use client::run as run_client;
use server::run as run_server;

use clap::Clap;

/// Pong!
#[derive(Clap)]
struct Opts {
    /// Run as a server or as one of the two clients?
    #[clap(short, long)]
    server: bool,

    /// The server address to send to/receive on
    #[clap(short, long, default_value = "127.0.0.1:6666")]
    addr: String,

    /// Bad-mannered computer player
    #[clap(long)]
    cpu: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    let cpu = opts.cpu;
    let addr = opts.addr;
    println!("Server address: {}", &addr);

    if opts.server {
        if let Err(e) = run_server(&addr) {
            println!("error: {}", e);

            std::process::exit(1);
        }
    } else {
        run_client(&addr, cpu);
    }
}
