mod client;

use client::Client;
use coffee::graphics::WindowSettings;
use coffee::Game;

fn main() {
    match Client::run(WindowSettings {
        title: String::from("Mazehem"),
        size: (590, 590),
        resizable: false,
        fullscreen: false,
        maximized: false,
    }) {
        Err(coffee::Error::IO(_)) => {
            println!("/mazehem ${{host_addr}}:${{port}}")
        }
        _ => (),
    }
}
