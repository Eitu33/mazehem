use crate::maze::Maze;
use bincode::{deserialize, serialize};
use indexmap::IndexMap;
use laminar::{Packet, Socket, SocketEvent};
use local_ip_address::local_ip;
use serde_derive::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::Instant;
use types::cell::Cell;
use types::coord::Coord;
use types::input::SerKey;
use types::player::{init_players, Player};

// TODO: split game file content
// TODO: encrypt connections
// TODO: async receiving?
// TODO: send maze in 1 packet?

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

pub struct Server {
    players: Vec<Player>,
    socket: Socket,
    cells: IndexMap<Coord, Cell>,
    clients: Vec<SocketAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Data {
    Cell(Cell),
    Key(SerKey),
    Players(Vec<Player>),
}

impl Server {
    pub fn new() -> Server {
        println!("server address: {}:9090", local_ip().unwrap());
        Server {
            players: init_players(),
            socket: Socket::bind_with_config(
                "0.0.0.0:9090",
                laminar::Config {
                    max_packets_in_flight: ((WIDTH * HEIGHT) * 2) as u16,
                    ..Default::default()
                },
            )
            .unwrap(),
            cells: Maze::new(WIDTH, HEIGHT).generate(),
            clients: Vec::new(),
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

    pub fn handle_received_packets(&mut self) {
        self.socket.manual_poll(Instant::now());
        while let Some(event) = self.socket.recv() {
            match event {
                SocketEvent::Packet(packet) => match deserialize::<Data>(packet.payload()) {
                    Ok(Data::Key(key)) => {
                        let client_addr = packet.addr();
                        if let Some(index) = self.clients.iter().position(|x| x == &client_addr) {
                            self.move_player(index, key);
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
                    if self.clients.len() < 4 {
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

    pub fn send(&mut self) {
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
