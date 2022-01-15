use crate::cell::Cell;
use crate::coord::Coord;
use indexmap::IndexMap;
use rand::Rng;
use std::collections::HashMap;

pub struct Maze {
    unconnected: HashMap<Coord, ()>,
    connected: IndexMap<Coord, Cell>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let mut unconnected: HashMap<Coord, ()> = HashMap::new();
        let mut connected: IndexMap<Coord, Cell> = IndexMap::new();
        let starting_coord = Coord::new(width / 2, height / 2);
        connected.insert(starting_coord, Cell::new(starting_coord));
        for x in 0..width {
            for y in 0..height {
                unconnected.insert(Coord::new(x, y), ());
            }
        }
        Maze {
            unconnected,
            connected,
        }
    }

    fn list_candidates(&mut self, index: usize) -> Vec<Coord> {
        let mut candidates: Vec<Coord> = Vec::new();
        let basic = self.connected[index].get_basic_neighbors();
        for coord in basic {
            if coord != self.connected[index].coord {
                candidates.push(coord);
            }
        }
        candidates
    }

    fn chose_candidate(&mut self, index: usize) -> Coord {
        let neighbors = self.list_candidates(index);
        neighbors[rand::thread_rng().gen_range(0..neighbors.len())]
    }

    pub fn generate(&mut self) -> IndexMap<Coord, Cell> {
        let mut rng = rand::thread_rng();
        while !self.unconnected.is_empty() {
            // generate a random number
            let index = rng.gen_range(0..(self.connected.len()));
            // chose a candidate
            let chosen = self.chose_candidate(index);
            // add candidate if it could be removed from the unconnected list
            if self.unconnected.remove(&chosen).is_some() {
                self.connected.insert(chosen, Cell::new(chosen));
                self.connected[index].push_neighbor(chosen);
            }
        }
        self.connected.clone()
    }
}
