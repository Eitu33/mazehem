mod cell;
mod coord;
mod drawable;
mod maze;
mod player;

use cell::Cell;
use coffee::graphics::{
    Color, Font, Frame, Mesh, Point, Rectangle, Shape, Text, Window, WindowSettings,
};
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
}

impl Mazehem {
    fn move_player(&mut self) {
        // if let Some(a) = self.last_key {
        //     println!("{:#?}", a);
        // }
        match self.last_key {
            Some(KeyCode::Right) => {
                // println!("pos: {}", self.player.c);
                // for a in &self
                //     .cells
                //     .get(&Coord::new(self.player.c.x, self.player.c.y))
                //     .unwrap()
                //     .n
                // {
                //     println!("CURRENT N: {}", a);
                // }
                // for a in &self
                //     .cells
                //     .get(&Coord::new(self.player.c.x + 1, self.player.c.y))
                //     .unwrap()
                //     .n
                // {
                //     println!("NEXT N: {}", a);
                // }
                if self
                    .cells
                    .get(&Coord::new(self.player.c.x, self.player.c.y))
                    .unwrap()
                    .n
                    .contains(&Coord::new(self.player.c.x + 1, self.player.c.y))
                {
                    self.player.c.x += 1
                } else if self
                    .cells
                    .get(&Coord::new(self.player.c.x + 1, self.player.c.y))
                    .unwrap()
                    .n
                    .contains(&Coord::new(self.player.c.x, self.player.c.y))
                {
                    self.player.c.x += 1
                }
            }
            Some(KeyCode::Down) => {
                if self
                    .cells
                    .get(&Coord::new(self.player.c.x, self.player.c.y))
                    .unwrap()
                    .n
                    .contains(&Coord::new(self.player.c.x, self.player.c.y + 1))
                {
                    self.player.c.y += 1
                } else if self
                    .cells
                    .get(&Coord::new(self.player.c.x, self.player.c.y + 1))
                    .unwrap()
                    .n
                    .contains(&Coord::new(self.player.c.x, self.player.c.y))
                {
                    self.player.c.y += 1
                }
            }
            Some(KeyCode::Left) => self.player.c.x -= 1,
            Some(KeyCode::Up) => self.player.c.y -= 1,
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
