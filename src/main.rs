mod cell;
mod coord;
mod drawable;
mod game;
mod goals;
mod input;
mod maze;
mod player;

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
            println!("usage:\n\t./mazehem client host_addr:port\n\t./mazehem host")
        }
        _ => (),
    }
}
