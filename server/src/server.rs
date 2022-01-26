use crate::maze::Maze;
use bincode::{deserialize, serialize};
use crossbeam_channel::{Receiver, Sender};
use indexmap::IndexMap;
use laminar::{Packet, Socket, SocketEvent};
use rand::rngs::OsRng;
use rsa::{PaddingScheme, RsaPrivateKey};
use std::net::SocketAddr;
use types::cell::Cell;
use types::constants::{HEIGHT, WIDTH};
use types::coord::Coord;
use types::data::Data;
use types::input::SerKey;
use types::player::{init_players, Player};

pub struct Server {
    private_key: RsaPrivateKey,
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    clients: Vec<SocketAddr>,
    players: Vec<Player>,
    cells: IndexMap<Coord, Cell>,
}

impl Server {
    pub fn new() -> Server {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("failed to generate a key");
        let mut socket = Socket::bind("0.0.0.0:9090").unwrap();
        let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());
        std::thread::spawn(move || socket.start_polling());
        Server {
            private_key,
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
                    SocketEvent::Disconnect(addr) => self.on_disconnected_client(addr),
                    _ => (),
                }
            }
            self.send_computed_data();
        }
    }

    fn on_packet_received(&mut self, packet: Packet) {
        let client_addr = packet.addr();
        match deserialize::<Data>(packet.payload()) {
            Ok(Data::Key(key)) => {
                if let Some(index) = self.clients.iter().position(|x| x == &client_addr) {
                    self.move_player(index, key);
                }
            }
            Ok(Data::Connection) => {
                let client_addr = packet.addr();
                self.sender
                    .send(Packet::reliable_unordered(
                        client_addr,
                        serialize::<Data>(&Data::PrivateKey(self.private_key.clone())).unwrap(),
                    ))
                    .expect("should send");
            }
            Ok(Data::Handshake(enc_data)) => {
                let dec_data = self
                    .private_key
                    .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &enc_data)
                    .expect("failed to decrypt");
                if dec_data == b"game client connection" {
                    self.on_connected_client(client_addr);
                }
            }
            _ => (),
        }
    }

    fn on_connected_client(&mut self, addr: SocketAddr) {
        println!("client ip {} connected", addr);
        if self.clients.len() < 4 && !self.clients.contains(&addr) {
            println!("client ip {} has been registered", addr);
            self.clients.push(addr);
        }
        let vec = self.cells.clone().into_iter().map(|x| x.1).collect::<Vec<Cell>>();
        for chunk in vec.chunks(10) {
            self.sender
                .send(Packet::reliable_unordered(
                    addr,
                    serialize::<Data>(&Data::Cells(chunk.to_vec())).unwrap(),
                ))
                .expect("should send");
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
