use crate::cell::Cell;
use crate::coord::Coord;
use crate::drawable::Drawable;
use crate::goals::Goals;
use crate::input::CustomInput;
use crate::maze::Maze;
use crate::player::Player;
use bincode::{deserialize, serialize};
use coffee::graphics::{Color, Frame, Mesh, Window};
use coffee::input::keyboard::KeyCode;
use coffee::load::Task;
use coffee::{Game, Timer};
use indexmap::IndexMap;
use laminar::{Packet, Socket, SocketEvent};
use std::env;
use std::time::Instant;

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

pub struct Mazehem {
    cells: IndexMap<Coord, Cell>,
    last_key: Option<KeyCode>,
    player: Player,
    goals: Goals,
    bind: Socket,
}

impl<T> Drawable for IndexMap<Coord, T>
where
    T: Drawable,
{
    fn draw(&self, mesh: &mut Mesh) {
        for cell in self {
            cell.1.draw(mesh);
        }
    }
}

impl Mazehem {
    fn move_allowed(&self, to: &Coord) -> bool {
        if !to.out_of_bounds(WIDTH, HEIGHT) {
            self.cells.get(&self.player.coord).unwrap().n.contains(to)
                || self.cells.get(to).unwrap().n.contains(&self.player.coord)
        } else {
            false
        }
    }

    fn move_player(&mut self) {
        match self.last_key {
            Some(KeyCode::Right)
                if self.move_allowed(&Coord::new(
                    self.player.coord.x.saturating_add(1),
                    self.player.coord.y,
                )) =>
            {
                self.player.coord.x += 1;
                self.last_key = None;
            }
            Some(KeyCode::Down)
                if self.move_allowed(&Coord::new(
                    self.player.coord.x,
                    self.player.coord.y.saturating_add(1),
                )) =>
            {
                self.player.coord.y += 1;
                self.last_key = None;
            }
            Some(KeyCode::Left)
                if self.move_allowed(&Coord::new(
                    self.player.coord.x.saturating_sub(1),
                    self.player.coord.y,
                )) =>
            {
                self.player.coord.x -= 1;
                self.last_key = None;
            }
            Some(KeyCode::Up)
                if self.move_allowed(&Coord::new(
                    self.player.coord.x,
                    self.player.coord.y.saturating_sub(1),
                )) =>
            {
                self.player.coord.y -= 1;
                self.last_key = None;
            }
            _ => (),
        }
    }
}

#[allow(unused_must_use)]
impl Game for Mazehem {
    type Input = CustomInput;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Mazehem> {
        // read client / server and ip input here?
        let args: Vec<String> = env::args().collect();
        println!("{:#?}", args);

        let mut maze = Maze::new(WIDTH, HEIGHT);
        let cells = maze.generate();
        Task::succeed(|| Mazehem {
            cells,
            last_key: None,
            player: Player::new(1),
            goals: Goals::new(vec![Coord::new(WIDTH - 1, HEIGHT - 1)]),
            bind: Socket::bind("0.0.0.0:7070").unwrap(),
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
        // clear & compute
        frame.clear(Color::BLACK);
        self.move_player();

        // send & receive
        self.bind.send(Packet::unreliable(
            "0.0.0.0:8080".parse().unwrap(),
            serialize(&self.player).unwrap(),
        ));
        self.bind.manual_poll(Instant::now());
        while let Some(pkt) = self.bind.recv() {
            match pkt {
                SocketEvent::Packet(pkt) => {
                    println!["{:?}", deserialize::<Player>(pkt.payload()).unwrap()]
                }
                _ => {}
            }
        }

        // display
        let mut mesh = Mesh::new();
        self.cells.draw(&mut mesh);
        self.player.draw(&mut mesh);
        self.goals.draw(&mut mesh);
        mesh.draw(&mut frame.as_target());
    }
}
