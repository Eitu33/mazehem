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
use local_ip_address::local_ip;
use serde_derive::Deserialize;
use std::env;
use std::io;
use std::net::SocketAddr;
use std::time::Instant;

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

pub struct Mazehem {
    cells: IndexMap<Coord, Cell>,
    last_key: Option<KeyCode>,
    player: Player,
    goals: Goals,
    is_host: bool,
    current: Socket,
    external: Vec<SocketAddr>,
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

fn invalid_input() -> coffee::Error {
    coffee::Error::IO(io::Error::new(
        io::ErrorKind::InvalidInput,
        "incorrect usage",
    ))
}

fn handle_args() -> Result<(bool, Vec<SocketAddr>), coffee::Error> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => Err(invalid_input()),
        2 if args[1] == "host" => {
            println!("host address: {}:7070", local_ip().unwrap());
            Ok((true, Vec::new()))
        }
        // ensure the given ip is valid
        3 if args[1] == "client " => Ok((false, vec![args[2].parse().unwrap()])),
        _ => Err(invalid_input()),
    }
}

impl Mazehem {
    fn new() -> Result<Mazehem, coffee::Error> {
        let mut maze = Maze::new(WIDTH, HEIGHT);
        let cells = maze.generate();
        let args = handle_args()?;
        Ok(Mazehem {
            cells,
            last_key: None,
            player: Player::new(1),
            goals: Goals::new(vec![Coord::new(WIDTH - 1, HEIGHT - 1)]),
            is_host: args.0,
            current: Socket::bind("0.0.0.0:7070").unwrap(),
            external: args.1,
        })
    }
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

#[derive(Debug, Deserialize)]
enum Data {
    Player(Player),
    SocketAddr(SocketAddr),
}

#[allow(unused_must_use)]
impl Game for Mazehem {
    type Input = CustomInput;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Mazehem> {
        Task::new(|| Mazehem::new())
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
        for addr in &self.external {
            self.current
                .send(Packet::unreliable(*addr, serialize(&self.player).unwrap()));
        }
        self.current.manual_poll(Instant::now());
        while let Some(pkt) = self.current.recv() {
            match pkt {
                SocketEvent::Packet(pkt) => {
                    println!["{:#?}", deserialize::<Data>(pkt.payload()).unwrap()];
                    match deserialize::<Data>(pkt.payload()).unwrap() {
                        Data::Player(player) => (),
                        Data::SocketAddr(addr) => (),
                        _ => ()
                    }
                }
                _ => ()
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
