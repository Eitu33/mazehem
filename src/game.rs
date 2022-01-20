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

// TODO: encrypt connections

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

// pub struct Host {
//     cells: IndexMap<Coord, Cell>,
//     clients: Vec<SocketAddr>,
// }

// pub struct Client {
//     cells: Vec<Cell>,
//     server_addr: Option<SocketAddr>,
// }

pub struct Mazehem {
    cells: IndexMap<Coord, Cell>,
    v_cells: Vec<Cell>,
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
            v_cells: Vec::new(),
            last_key: SerKey::Undefined,
            players: init_players(),
            goal: Coord::new(WIDTH / 2, HEIGHT / 2),
            socket: if server_addr.is_some() {
                Socket::bind("0.0.0.0:7070").unwrap()
            } else {
                Socket::bind_with_config(
                    "0.0.0.0:9090",
                    laminar::Config {
                        max_packets_in_flight: ((WIDTH * HEIGHT) * 2) as u16,
                        ..Default::default()
                    },
                )
                .unwrap()
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

    fn handle_host_packets(&mut self) {
        while let Some(event) = self.socket.recv() {
            match event {
                SocketEvent::Packet(packet) => match deserialize::<Data>(packet.payload()) {
                    Ok(Data::Cell(cell)) => {
                        if self.v_cells.len() < (WIDTH * HEIGHT) * 2 {
                            self.v_cells.push(cell);
                        }
                    }
                    Ok(Data::Players(players)) => {
                        self.players = players.into_iter().map(|p| p.colored()).collect()
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    fn send_inputs(&mut self) {
        self.socket
            .send(Packet::reliable_unordered(
                self.server_addr.unwrap(),
                serialize::<Data>(&Data::Key(self.last_key.clone())).unwrap(),
            ))
            .expect("should send");
    }

    fn handle_clients_packets(&mut self) {
        while let Some(event) = self.socket.recv() {
            match event {
                SocketEvent::Packet(packet) => match deserialize::<Data>(packet.payload()) {
                    Ok(Data::Key(key)) => {
                        let client_addr = packet.addr();
                        if let Some(index) = self.clients.iter().position(|x| x == &client_addr) {
                            self.move_player(index + 1, key);
                        } else {
                            self.socket
                                .send(Packet::reliable_unordered(
                                    client_addr,
                                    "connection allowed".as_bytes().to_vec(),
                                ))
                                .expect("should send");
                        }
                    }
                    _ => (),
                },
                SocketEvent::Connect(addr) => {
                    if self.clients.len() < 3 {
                        println!("client ip {} connected and was registered", addr);
                        for c in &self.cells {
                            self.socket
                                .send(Packet::reliable_unordered(
                                    addr,
                                    serialize::<Data>(&Data::Cell(c.1.clone())).unwrap(),
                                ))
                                .expect("should send");
                        }
                        self.clients.push(addr);
                    }
                }
                SocketEvent::Disconnect(addr) => {
                    println!("client ip {} disconnected", addr)
                }
                _ => (),
            }
        }
    }

    fn compute_and_send_players(&mut self) {
        self.move_player(0, self.last_key.clone());
        for addr in &self.clients {
            self.socket
                .send(Packet::reliable_unordered(
                    *addr,
                    serialize::<Data>(&Data::Players(self.players.clone())).unwrap(),
                ))
                .expect("should send");
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Data {
    Cell(Cell),
    Key(SerKey),
    Players(Vec<Player>),
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
        } else {
            self.last_key = SerKey::Undefined;
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        let mut mesh = Mesh::new();
        frame.clear(Color::BLACK);

        if self.server_addr.is_none() {
            self.cells.draw(&mut mesh);
        } else {
            self.v_cells.draw(&mut mesh);
        }
        self.players.draw(&mut mesh);
        self.goal.draw(&mut mesh);
        mesh.draw(&mut frame.as_target());
    }

    fn update(&mut self, _window: &Window) {
        self.socket.manual_poll(Instant::now());
        if self.server_addr.is_some() {
            self.handle_host_packets();
            self.send_inputs();
        } else {
            self.handle_clients_packets();
            self.compute_and_send_players();
        }
    }
}
