mod cell;
mod coord;
mod maze;

use cell::Cell;
use coffee::graphics::{Color, Frame, Mesh, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Timer};
use maze::Maze;

fn main() -> coffee::Result<()> {
    Mazehem::run(WindowSettings {
        title: String::from("Mazehem"),
        size: (590, 590),
        resizable: false,
        fullscreen: false,
        maximized: false,
    })
}

struct Mazehem {
    cells: Vec<Cell>,
}

impl Game for Mazehem {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Mazehem> {
        let mut maze = Maze::new(30, 30);
        let cells = maze.generate();
        Task::succeed(|| Mazehem { cells })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        let mut mesh = Mesh::new();
        frame.clear(Color::BLACK);
        for cell in &self.cells {
            cell.draw(&mut mesh);
        }
        mesh.draw(&mut frame.as_target());
    }
}
