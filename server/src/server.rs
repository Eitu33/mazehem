use crate::maze::Maze;
use bincode::{deserialize, serialize};
use crossbeam_channel::{Receiver, Sender};
use indexmap::IndexMap;
use laminar::{Packet, Socket, SocketEvent};
use std::net::SocketAddr;
use types::cell::Cell;
use types::coord::Coord;
use types::data::Data;
use types::input::SerKey;
use types::player::{init_players, Player};

// TODO: encrypt connections
// TODO: send maze in 1 packet
// TODO: directly associate ips to players

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

pub struct Server {
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    clients: Vec<SocketAddr>,
    players: Vec<Player>,
    cells: IndexMap<Coord, Cell>,
}

impl Server {
    pub fn new() -> Server {
        let mut socket = Socket::bind_with_config(
            "0.0.0.0:9090",
            laminar::Config {
                max_packets_in_flight: ((WIDTH * HEIGHT) * 3) as u16,
                ..Default::default()
            },
        )
        .unwrap();
        let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
        std::thread::spawn(move || socket.start_polling());
        Server {
            sender,
            receiver,
            clients: Vec::new(),
            players: init_players(),
            cells: Maze::new(WIDTH, HEIGHT).generate(),
        }
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

    pub fn run(&mut self) {
        loop {
            if let Ok(event) = self.receiver.recv() {
                match event {
                    SocketEvent::Packet(packet) => match deserialize::<Data>(packet.payload()) {
                        Ok(Data::Key(key)) => {
                            let client_addr = packet.addr();
                            if let Some(index) = self.clients.iter().position(|x| x == &client_addr)
                            {
                                self.move_player(index, key);
                            } else {
                                self.sender
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
                        if self.clients.len() < 4 {
                            println!("client ip {} connected and was registered", addr);
                            for c in &self.cells {
                                self.sender
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
            for addr in &self.clients {
                self.sender
                    .send(Packet::reliable_unordered(
                        *addr,
                        serialize::<Data>(&Data::Players(self.players.clone())).unwrap(),
                    ))
                    .expect("should send");
            }
        }
    }
}
