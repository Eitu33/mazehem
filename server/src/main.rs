mod maze;
mod server;

use server::Server;

fn main() {
    let mut server = Server::new();
    loop {
        server.receive_and_compute();
        server.send_players();
    }
}
