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
use std::thread;
use std::time::Instant;

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

pub struct Mazehem {
    cells: IndexMap<Coord, Cell>,
    last_key: Option<KeyCode>,
    player: Player,
    goals: Goals,
    socket: Socket,
    clients: Vec<SocketAddr>,
}

impl Mazehem {
    fn new() -> Mazehem {
        println!("host address: {}:7070", local_ip().unwrap());
        Mazehem {
            cells: Maze::new(WIDTH, HEIGHT).generate(),
            last_key: None,
            player: Player::new(1),
            goals: Goals::new(vec![Coord::new(WIDTH - 1, HEIGHT - 1)]),
            socket: Socket::bind("0.0.0.0:9090").unwrap(),
            clients: Vec::new(),
        }
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
    Cells(Vec<Cell>),
}

#[allow(unused_must_use)]
impl Game for Mazehem {
    type Input = CustomInput;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Mazehem> {
        Task::succeed(|| Mazehem::new())
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

        self.socket.manual_poll(Instant::now());
        if let Some(socket_event) = self.socket.recv() {
            match socket_event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();
                    let msg = String::from_utf8_lossy(msg);
                    let ip = packet.addr().ip();
                    println!("Received {:?} from {:?}", msg, ip);

                    self.socket
                        .send(Packet::reliable_unordered(
                            packet.addr(),
                            "Copy that!".as_bytes().to_vec(),
                        ))
                        .expect("This should send");
                }
                SocketEvent::Connect(connect_event) => { /* a client connected */ }
                SocketEvent::Timeout(timeout_event) => { /* a client timed out */ }
                SocketEvent::Disconnect(disconnect_event) => { /* a client disconnected */ }
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
