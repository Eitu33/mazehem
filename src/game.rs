use crate::cell::Cell;
use crate::coord::Coord;
use crate::drawable::Drawable;
use crate::input::GameInput;
use crate::maze::Maze;
use crate::player::{init_players, Player};
use bincode::{deserialize, serialize};
use coffee::graphics::{Color, Frame, Mesh, Window};
use coffee::input::keyboard::KeyCode;
use coffee::load::Task;
use coffee::{Game, Timer};
use indexmap::IndexMap;
use laminar::{Packet, Socket, SocketEvent};
use local_ip_address::local_ip;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::io;
use std::net::SocketAddr;
use std::time::Instant;

// TODO: make sure the given ip is valid
// TODO: send maze
// TODO: encrypt connections

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

pub struct Mazehem {
    cells: IndexMap<Coord, Cell>,
    last_key: SerKey,
    players: Vec<Player>,
    goal: Coord,
    socket: Socket,
    clients: Vec<SocketAddr>,
    server_addr: Option<SocketAddr>,
}

fn invalid_input() -> coffee::Error {
    coffee::Error::IO(io::Error::new(
        io::ErrorKind::InvalidInput,
        "incorrect usage",
    ))
}

fn handle_args() -> coffee::Result<Option<SocketAddr>> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => Err(invalid_input()),
        2 if args[1] == "host" => {
            println!("host address: {}:9090", local_ip().unwrap());
            Ok(None)
        }
        3 if args[1] == "client" => Ok(Some(args[2].parse().unwrap())),
        _ => Err(invalid_input()),
    }
}

impl Mazehem {
    fn new() -> coffee::Result<Mazehem> {
        let server_addr = handle_args()?;
        Ok(Mazehem {
            cells: Maze::new(WIDTH, HEIGHT).generate(),
            last_key: SerKey::Undefined,
            players: init_players(),
            goal: Coord::new(WIDTH / 2, HEIGHT / 2),
            socket: if server_addr.is_some() {
                Socket::bind("0.0.0.0:7070").unwrap()
            } else {
                Socket::bind("0.0.0.0:9090").unwrap()
            },
            clients: Vec::new(),
            server_addr,
        })
    }

    fn move_allowed(&self, i: usize, to: &Coord) -> bool {
        if !to.out_of_bounds(WIDTH, HEIGHT) {
            self.cells.get(&self.players[i].coord).unwrap().n.contains(to)
                || self.cells.get(to).unwrap().n.contains(&self.players[i].coord)
        } else {
            false
        }
    }

    fn move_player(&mut self, i: usize, key: SerKey) {
        match key {
            SerKey::Right
                if self.move_allowed(
                    i,
                    &Coord::new(
                        self.players[i].coord.x.saturating_add(1),
                        self.players[i].coord.y,
                    ),
                ) =>
            {
                self.players[i].coord.x += 1;
            }
            SerKey::Down
                if self.move_allowed(
                    i,
                    &Coord::new(
                        self.players[i].coord.x,
                        self.players[i].coord.y.saturating_add(1),
                    ),
                ) =>
            {
                self.players[i].coord.y += 1;
            }
            SerKey::Left
                if self.move_allowed(
                    i,
                    &Coord::new(
                        self.players[i].coord.x.saturating_sub(1),
                        self.players[i].coord.y,
                    ),
                ) =>
            {
                self.players[i].coord.x -= 1;
            }
            SerKey::Up
                if self.move_allowed(
                    i,
                    &Coord::new(
                        self.players[i].coord.x,
                        self.players[i].coord.y.saturating_sub(1),
                    ),
                ) =>
            {
                self.players[i].coord.y -= 1;
            }
            _ => (),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerKey {
    Undefined,
    Right,
    Down,
    Left,
    Up,
}

impl From<KeyCode> for SerKey {
    fn from(key: KeyCode) -> SerKey {
        match key {
            KeyCode::Right => SerKey::Right,
            KeyCode::Left => SerKey::Left,
            KeyCode::Down => SerKey::Down,
            KeyCode::Up => SerKey::Up,
            _ => SerKey::Undefined,
        }
    }
}

#[allow(unused_must_use)]
impl Game for Mazehem {
    type Input = GameInput;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Mazehem> {
        Task::new(|| Mazehem::new())
    }

    fn interact(&mut self, input: &mut GameInput, _window: &mut Window) {
        if input.keys_pressed.len() != 0 {
            let key = input.keys_pressed[0];
            self.last_key = SerKey::from(key);
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        self.socket.manual_poll(Instant::now());
        if let Some(addr) = self.server_addr {
            // note: need to receive player positions & move should not be done
            self.move_player(1, self.last_key.clone());
            self.socket
                .send(Packet::reliable_unordered(
                    addr,
                    serialize::<SerKey>(&self.last_key).unwrap(),
                ))
                .expect("this should send");
        } else {
            // working but missing maze sending
            self.move_player(0, self.last_key.clone());
            if let Some(socket_event) = self.socket.recv() {
                match socket_event {
                    SocketEvent::Packet(packet) => {
                        if let Ok(key) = deserialize::<SerKey>(packet.payload()) {
                            let client_addr = packet.addr();
                            if let Some(index) = self.clients.iter().position(|x| x == &client_addr)
                            {
                                self.move_player(index + 1, key);
                            } else {
                                self.socket
                                    .send(Packet::reliable_unordered(
                                        client_addr,
                                        "connection allowed".as_bytes().to_vec(),
                                    ))
                                    .unwrap();
                            }
                        }
                    }
                    SocketEvent::Connect(addr) => {
                        if self.clients.len() < 3 {
                            let ser_cells: Vec<Cell> = self
                                .cells
                                .clone()
                                .into_iter()
                                .map(|x| x.1)
                                .collect::<Vec<Cell>>()
                                .split_off(10);
                            self.socket
                                .send(Packet::reliable_unordered(
                                    addr,
                                    serialize::<Vec<Cell>>(&ser_cells).unwrap(),
                                ))
                                .unwrap();
                            self.clients.push(addr);
                        }
                    }
                    SocketEvent::Timeout(_) => (),
                    SocketEvent::Disconnect(_) => (),
                }
            }
        }
        let mut mesh = Mesh::new();
        frame.clear(Color::BLACK);
        self.cells.draw(&mut mesh);
        self.players.draw(&mut mesh);
        self.goal.draw(&mut mesh);
        mesh.draw(&mut frame.as_target());
    }
}
