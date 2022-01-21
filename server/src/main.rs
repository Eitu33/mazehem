mod maze;
mod server;

use local_ip_address::local_ip;
use server::Server;

fn main() {
    println!("server address: {}:9090", local_ip().unwrap());
    let mut server = Server::new();
    server.run();
}
