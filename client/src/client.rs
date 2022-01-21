use bincode::{deserialize, serialize};
use coffee::graphics::{Color, Frame, Mesh, Window};
use coffee::load::Task;
use coffee::{Game, Timer};
use laminar::{Packet, Socket, SocketEvent};
use std::env;
use std::io;
use std::net::SocketAddr;
use std::time::Instant;
use types::cell::Cell;
use types::coord::Coord;
use types::data::Data;
use types::drawable::Drawable;
use types::input::{GameInput, SerKey};
use types::player::{init_players, Player};

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

fn handle_args() -> coffee::Result<Option<SocketAddr>> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => Ok(Some(args[1].parse().unwrap())),
        _ => Err(coffee::Error::IO(io::Error::new(
            io::ErrorKind::InvalidInput,
            "incorrect usage",
        ))),
    }
}
pub struct Client {
    socket: Socket,
    host_addr: Option<SocketAddr>,
    last_key: SerKey,
    cells: Vec<Cell>,
    goal: Coord,
    players: Vec<Player>,
}

impl Client {
    fn new() -> coffee::Result<Client> {
        let host_addr = handle_args()?;
        Ok(Client {
            socket: Socket::bind("0.0.0.0:7070").unwrap(),
            host_addr,
            last_key: SerKey::Undefined,
            cells: Vec::new(),
            goal: Coord::new(WIDTH / 2, HEIGHT / 2),
            players: init_players(),
        })
    }

    fn handle_received_packets(&mut self) {
        while let Some(event) = self.socket.recv() {
            match event {
                SocketEvent::Packet(packet) => match deserialize::<Data>(packet.payload()) {
                    Ok(Data::Cells(mut cells)) => {
                        self.cells.append(&mut cells);
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
                self.host_addr.unwrap(),
                serialize::<Data>(&Data::Key(self.last_key.clone())).unwrap(),
            ))
            .expect("should send");
    }
}

impl Game for Client {
    type Input = GameInput;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Client> {
        Task::new(|| Client::new())
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
        self.cells.draw(&mut mesh);
        self.goal.draw(&mut mesh);
        self.players.draw(&mut mesh);
        mesh.draw(&mut frame.as_target());
    }

    fn update(&mut self, _window: &Window) {
        self.socket.manual_poll(Instant::now());
        self.handle_received_packets();
        self.send_inputs();
    }
}