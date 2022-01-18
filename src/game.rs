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
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::io;
use std::net::SocketAddr;
use std::time::Instant;

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

// use struct instead of enum because coffee interface uses a mutable reference
// move out of the ref to update the enums content not allowed
pub struct NetworkIdendity {
    host: bool,
    self_addr: SocketAddr,
    host_addr: Option<SocketAddr>,
    socket: Option<Socket>,
    clients: Vec<SocketAddr>,
}

impl NetworkIdendity {
    fn server() -> NetworkIdendity {
        NetworkIdendity {
            host: true,
            self_addr: SocketAddr::new(local_ip().unwrap(), 7070),
            host_addr: None,
            socket: Some(Socket::bind("0.0.0.0:7070").unwrap()),
            clients: Vec::new(),
        }
    }

    fn client(host_addr: SocketAddr) -> NetworkIdendity {
        NetworkIdendity {
            host: false,
            self_addr: SocketAddr::new(local_ip().unwrap(), 9090),
            host_addr: Some(host_addr),
            socket: Some(Socket::bind("0.0.0.0:9090").unwrap()),
            clients: Vec::new(),
        }
    }
}

pub struct Mazehem {
    cells: IndexMap<Coord, Cell>,
    last_key: Option<KeyCode>,
    player: Player,
    goals: Goals,
    network: NetworkIdendity,
}

fn handle_args() -> Result<Option<SocketAddr>, coffee::Error> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => Err(coffee::Error::IO(io::Error::new(
            io::ErrorKind::InvalidInput,
            "missing arguments",
        ))),
        2 if args[1] == "host" => {
            println!("host address: {}:7070", local_ip().unwrap());
            Ok(None)
        }
        // TODO: make sure the given ip is valid
        3 if args[1] == "client" => Ok(Some(args[2].parse().unwrap())),
        _ => Err(coffee::Error::IO(io::Error::new(
            io::ErrorKind::InvalidInput,
            "incorrect usage",
        ))),
    }
}

impl Mazehem {
    fn new() -> Result<Mazehem, coffee::Error> {
        let mut maze = Maze::new(WIDTH, HEIGHT);
        let cells = maze.generate();
        let arg = handle_args()?;
        Ok(Mazehem {
            cells,
            last_key: None,
            player: Player::new(1),
            goals: Goals::new(vec![Coord::new(WIDTH - 1, HEIGHT - 1)]),
            network: match arg {
                Some(addr) => NetworkIdendity::client(addr),
                None => NetworkIdendity::server(),
            },
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

#[derive(Debug, Serialize, Deserialize)]
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
        frame.clear(Color::BLACK);
        self.move_player();
        let mut mesh = Mesh::new();
        let mut players: Vec<Player> = Vec::new();

        // TODO: server authority
        // TODO: send maze
        // TODO: encrypt connections
        // TODO: add clients version
        match self.network.host {
            true => {
                self.network
                    .socket
                    .as_mut()
                    .unwrap()
                    .manual_poll(Instant::now());
                // NEXT TODO: LIMIT ADDR SENDING
                while let Some(pkt) = self.network.socket.as_mut().unwrap().recv() {
                    match pkt {
                        SocketEvent::Packet(pkt) => {
                            println![
                                "received by server: {:#?}",
                                deserialize::<Data>(pkt.payload()).unwrap()
                            ];
                            match deserialize::<Data>(pkt.payload()).unwrap() {
                                Data::SocketAddr(addr) => self.network.clients.push(addr),
                                _ => (),
                            }
                        }
                        _ => (),
                    }
                }
                for addr in &self.network.clients {
                    self.network
                        .socket
                        .as_mut()
                        .unwrap()
                        .send(Packet::unreliable(
                            *addr,
                            serialize(&Data::Player(self.player.clone())).unwrap(),
                        ));
                }
            }
            false => {
                self.network
                    .socket
                    .as_mut()
                    .unwrap()
                    .manual_poll(Instant::now());
                while let Some(pkt) = self.network.socket.as_mut().unwrap().recv() {
                    match pkt {
                        SocketEvent::Packet(pkt) => {
                            println![
                                "received by client: {:#?}",
                                deserialize::<Data>(pkt.payload()).unwrap()
                            ];
                            match deserialize::<Data>(pkt.payload()).unwrap() {
                                Data::Player(player) => players.push(player),
                                _ => (),
                            }
                        }
                        _ => (),
                    }
                }
                self.network
                    .socket
                    .as_mut()
                    .unwrap()
                    .send(Packet::unreliable(
                        self.network.host_addr.unwrap(),
                        serialize(&Data::SocketAddr(self.network.self_addr)).unwrap(),
                    ));
            }
        }
        players.push(self.player.clone());
        println!("PLAYERS LIST: {:#?}", players);

        self.cells.draw(&mut mesh);
        self.goals.draw(&mut mesh);
        players.draw(&mut mesh);
        mesh.draw(&mut frame.as_target());
    }
}
