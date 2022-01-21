mod game;

use coffee::graphics::WindowSettings;
use coffee::Game;
use game::Mazehem;

fn main() {
    match Mazehem::run(WindowSettings {
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
