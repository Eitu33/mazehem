mod cell;
mod coord;
mod drawable;
mod maze;
mod player;

use cell::Cell;
use coffee::graphics::{Color, Font, Frame, Mesh, Rectangle, Shape, Text, Window, WindowSettings};
use coffee::input::keyboard::KeyCode;
use coffee::input::{self, keyboard, Input};
use coffee::load::Task;
use coffee::{Game, Timer};
use coord::Coord;
use drawable::Drawable;
use indexmap::IndexMap;
use maze::Maze;
use player::Player;

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
    cells: IndexMap<Coord, Cell>,
    last_key: Option<KeyCode>,
    player: Player,
    goal_coord: Coord,
}

impl Mazehem {
    fn move_allowed(&self, to: &Coord) -> bool {
        self.cells.get(&self.player.c).unwrap().n.contains(to)
            || self.cells.get(to).unwrap().n.contains(&self.player.c)
    }

    fn move_player(&mut self) {
        match self.last_key {
            Some(KeyCode::Right)
                if self.move_allowed(&Coord::new(
                    self.player.c.x.saturating_add(1),
                    self.player.c.y,
                )) =>
            {
                self.player.c.x += 1;
                self.last_key = None;
            }
            Some(KeyCode::Down)
                if self.move_allowed(&Coord::new(
                    self.player.c.x,
                    self.player.c.y.saturating_add(1),
                )) =>
            {
                self.player.c.y += 1;
                self.last_key = None;
            }
            Some(KeyCode::Left)
                if self.move_allowed(&Coord::new(
                    self.player.c.x.saturating_sub(1),
                    self.player.c.y,
                )) =>
            {
                self.player.c.x -= 1;
                self.last_key = None;
            }
            Some(KeyCode::Up)
                if self.move_allowed(&Coord::new(
                    self.player.c.x,
                    self.player.c.y.saturating_sub(1),
                )) =>
            {
                self.player.c.y -= 1;
                self.last_key = None;
            }
            _ => (),
        }
    }
}

impl Game for Mazehem {
    type Input = CustomInput;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Mazehem> {
        let mut maze = Maze::new(30, 30);
        let cells = maze.generate();
        Task::succeed(|| Mazehem {
            cells,
            last_key: None,
            player: Player::new(Color::RED, Coord::new(0, 0)),
            goal_coord: Coord::new(30, 30),
        })
    }

    fn interact(&mut self, input: &mut CustomInput, _window: &mut Window) {
        if input.keys_pressed.len() != 0 {
            let key = input.keys_pressed[0];
            match key {
                KeyCode::Right => {
                    self.last_key = Some(key);
                }
                KeyCode::Left => {
                    self.last_key = Some(key);
                }
                KeyCode::Down => {
                    self.last_key = Some(key);
                }
                KeyCode::Up => {
                    self.last_key = Some(key);
                }
                _ => (),
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        let mut mesh = Mesh::new();
        frame.clear(Color::BLACK);
        for cell in &self.cells {
            cell.1.draw(&mut mesh);
        }
        self.move_player();
        self.player.draw(&mut mesh);
        mesh.draw(&mut frame.as_target());
    }
}

struct CustomInput {
    keys_pressed: Vec<KeyCode>,
}

impl Input for CustomInput {
    fn new() -> CustomInput {
        CustomInput {
            keys_pressed: Vec::new(),
        }
    }

    fn update(&mut self, event: input::Event) {
        match event {
            input::Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::Input { key_code, state } => match state {
                    input::ButtonState::Pressed => {
                        self.keys_pressed.push(key_code);
                    }
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }

    fn clear(&mut self) {
        self.keys_pressed.clear();
    }
}
