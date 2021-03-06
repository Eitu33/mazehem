use bincode::{deserialize, serialize};
use coffee::graphics::{Color, Font, Frame, Mesh, Point, Text, Window};
use coffee::load::Task;
use coffee::{Game, Timer};
use laminar::{Packet, Socket, SocketEvent};
use std::env;
use std::io;
use std::net::SocketAddr;
use std::time::Instant;
use types::cell::Cell;
use types::constants::{HEIGHT, WALLS_COLOR, WIDTH};
use types::coord::Coord;
use types::data::Data;
use types::drawable::Drawable;
use types::input::{GameInput, SerKey};
use types::player::Player;

fn handle_args() -> coffee::Result<SocketAddr> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => Ok(args[1].parse().unwrap()),
        _ => Err(coffee::Error::IO(io::Error::new(
            io::ErrorKind::InvalidInput,
            "incorrect usage",
        ))),
    }
}

pub struct Client {
    socket: Socket,
    server_addr: SocketAddr,
    connected: bool,
    last_key: SerKey,
    cells: Vec<Cell>,
    players: Vec<Player>,
    goal: Coord,
}

impl Client {
    fn new() -> coffee::Result<Client> {
        let mut client = Client {
            socket: Socket::bind("0.0.0.0:7070").unwrap(),
            server_addr: handle_args()?,
            connected: false,
            last_key: SerKey::Undefined,
            cells: Vec::new(),
            players: Vec::new(),
            goal: Coord::new(WIDTH / 2, HEIGHT / 2),
        };
        client
            .socket
            .send(Packet::reliable_unordered(
                client.server_addr,
                serialize::<Data>(&Data::Connection).unwrap(),
            ))
            .expect("should send");
        Ok(client)
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
                SocketEvent::Connect(_) => {
                    println!("connected");
                    self.connected = true;
                }
                SocketEvent::Timeout(_) => {
                    eprintln!("connection failed");
                    std::process::exit(1);
                }
                SocketEvent::Disconnect(_) => {
                    eprintln!("connection to the server has been lost");
                    std::process::exit(1);
                }
            }
        }
    }

    fn send_inputs(&mut self) {
        self.socket
            .send(Packet::reliable_unordered(
                self.server_addr,
                serialize::<Data>(&Data::Key(self.last_key.clone())).unwrap(),
            ))
            .expect("should send");
    }
}

impl Game for Client {
    type Input = GameInput;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Client> {
        Task::new(Client::new)
    }

    fn interact(&mut self, input: &mut GameInput, _window: &mut Window) {
        if !input.keys_pressed.is_empty() {
            let key = input.keys_pressed[0];
            self.last_key = SerKey::from(key);
        } else {
            self.last_key = SerKey::Undefined;
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::from_rgb_u32(WALLS_COLOR));
        if self.connected {
            let mut mesh = Mesh::new();
            self.cells.draw(&mut mesh);
            self.goal.draw(&mut mesh);
            self.players.draw(&mut mesh);
            mesh.draw(&mut frame.as_target());
        } else {
            let mut font =
                Font::from_bytes(frame.gpu(), include_bytes!("../../resources/visitor2.ttf"))
                    .expect("failed to load font");
            font.add(Text {
                content: "connecting...",
                position: Point::new(350.0, 300.0),
                size: 50.0,
                color: Color::WHITE,
                ..Text::default()
            });
            font.draw(&mut frame.as_target());
        }
    }

    fn update(&mut self, _window: &Window) {
        self.socket.manual_poll(Instant::now());
        self.handle_received_packets();
        self.send_inputs();
    }
}
