mod maze;
mod server;

use server::Server;

fn main() {
    let mut server = Server::new();
    loop {
        server.handle_received_packets();
        server.send();
    }
}
