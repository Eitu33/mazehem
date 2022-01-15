mod cell;
mod coord;
mod drawable;
mod game;
mod goals;
mod maze;
mod player;

use coffee::graphics::WindowSettings;
use coffee::Game;
use game::Mazehem;

fn main() -> coffee::Result<()> {
    Mazehem::run(WindowSettings {
        title: String::from("Mazehem"),
        size: (590, 590),
        resizable: false,
        fullscreen: false,
        maximized: false,
    })
}
