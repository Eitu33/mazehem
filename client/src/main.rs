mod client;

use client::Client;
use coffee::graphics::WindowSettings;
use coffee::Game;

fn main() {
    if let Err(coffee::Error::IO(_)) = Client::run(WindowSettings {
        title: String::from("Mazehem"),
        size: (590, 590),
        resizable: false,
        fullscreen: false,
        maximized: false,
    }) {
        println!("usage:\n\t./mazehem ${{host_addr}}:${{port}}");
    }
}
