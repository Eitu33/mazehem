use crate::cell::Cell;
use crate::coord::Coord;
use crate::drawable::Drawable;
use crate::goals::Goals;
use crate::input::GameInput;
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
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::io;
use std::net::SocketAddr;
use std::time::Instant;

// TODO: make sure the given ip is valid
// TODO: server authority
// TODO: send maze
// TODO: encrypt connections
// TODO: add clients version

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

pub struct Mazehem {
    cells: IndexMap<Coord, Cell>,
    last_key: Option<KeyCode>,
    player: Player,
    goals: Goals,
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
        let mut player = Player::new(1);
        player.init();
        Ok(Mazehem {
            cells: Maze::new(WIDTH, HEIGHT).generate(),
            last_key: None,
            player,
            goals: Goals::new(vec![Coord::new(WIDTH - 1, HEIGHT - 1)]),
            socket: if server_addr.is_some() {
                Socket::bind("0.0.0.0:7070").unwrap()
            } else {
                Socket::bind("0.0.0.0:9090").unwrap()
            },
            clients: Vec::new(),
            server_addr,
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
        frame.clear(Color::BLACK);
        self.move_player();
        let mut mesh = Mesh::new();
        let mut players: Vec<Player> = Vec::new();

        self.socket.manual_poll(Instant::now());
        if let Some(addr) = self.server_addr {
            self.socket
                .send(Packet::reliable_unordered(
                    addr,
                    "Hello server!".as_bytes().to_vec(),
                ))
                .expect("This should send");
        } else {
            if let Some(socket_event) = self.socket.recv() {
                match socket_event {
                    SocketEvent::Packet(packet) => {
                        let msg = String::from_utf8_lossy(packet.payload());
                        let ip = packet.addr().ip();
                        println!("Received {:?} from {:?}", msg, ip);

                        self.socket
                            .send(Packet::reliable_unordered(
                                packet.addr(),
                                "Copy that!".as_bytes().to_vec(),
                            ))
                            .expect("This should send");
                    }
                    SocketEvent::Connect(addr) => {
                        println!("ip = {} connected", addr);
                        self.clients.push(addr);
                    }
                    SocketEvent::Timeout(addr) => println!("ip = {} timed out", addr),
                    SocketEvent::Disconnect(addr) => {
                        println!("ip = {} disconnected", addr);
                        let index = self.clients.iter().position(|x| x == &addr).unwrap();
                        self.clients.remove(index);
                    }
                }
            }
        }

        players.push(self.player.clone());
        // println!("PLAYERS LIST: {:#?}", players);
        self.cells.draw(&mut mesh);
        // self.goals.draw(&mut mesh);
        players.draw(&mut mesh);
        mesh.draw(&mut frame.as_target());
    }
}
