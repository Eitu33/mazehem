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

// TODO: directly associate ips to players
// TODO: encrypt connections

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
        let mut socket = Socket::bind("0.0.0.0:9090").unwrap();
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

    pub fn run(&mut self) {
        loop {
            if let Ok(event) = self.receiver.recv() {
                match event {
                    SocketEvent::Packet(packet) => self.on_packet_received(packet),
                    SocketEvent::Connect(addr) => self.on_connected_client(addr),
                    SocketEvent::Disconnect(addr) => self.on_disconnected_client(addr),
                    _ => (),
                }
            }
            self.send_computed_data();
        }
    }

    fn on_packet_received(&mut self, packet: Packet) {
        match deserialize::<Data>(packet.payload()) {
            Ok(Data::Key(key)) => {
                let client_addr = packet.addr();
                if let Some(index) = self.clients.iter().position(|x| x == &client_addr) {
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
        }
    }

    fn on_connected_client(&mut self, addr: SocketAddr) {
        if self.clients.len() < 4 {
            println!("client ip {} connected and was registered", addr);
            let vec = self.cells.clone().into_iter().map(|x| x.1).collect::<Vec<Cell>>();
            for chunk in vec.chunks(10) {
                self.sender
                    .send(Packet::reliable_unordered(
                        addr,
                        serialize::<Data>(&Data::Cell(chunk.to_vec())).unwrap(),
                    ))
                    .expect("should send");
            }
            self.clients.push(addr);
        }
    }

    fn on_disconnected_client(&mut self, addr: SocketAddr) {
        println!("client ip {} disconnected", addr);
    }

    fn send_computed_data(&mut self) {
        for addr in &self.clients {
            self.sender
                .send(Packet::reliable_unordered(
                    *addr,
                    serialize::<Data>(&Data::Players(self.players.clone())).unwrap(),
                ))
                .expect("should send");
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

    fn move_allowed(&self, i: usize, to: &Coord) -> bool {
        if !to.out_of_bounds(WIDTH, HEIGHT) {
            self.cells.get(&self.players[i].coord).unwrap().n.contains(to)
                || self.cells.get(to).unwrap().n.contains(&self.players[i].coord)
        } else {
            false
        }
    }
}
