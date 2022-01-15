use crate::cell::Cell;
use crate::coord::Coord;
use indexmap::IndexMap;
use rand::Rng;
use std::collections::HashMap;

pub struct Maze {
    unconnected: HashMap<Coord, ()>,
    candidates: HashMap<Coord, ()>,
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
            candidates: HashMap::new(),
            unconnected,
            connected,
        }
    }

    fn save_candidates(&mut self, index: usize) {
        let basic = self.connected[index].get_basic_neighbors();
        for coord in basic {
            if coord != self.connected[index].c && !self.candidates.contains_key(&coord) {
                self.candidates.insert(coord, ());
            }
        }
        self.connected[index].computed = true;
    }

    fn list_adjacent_candidates(&self, index: usize) -> Vec<Coord> {
        let basic = self.connected[index].get_basic_neighbors();
        let mut neighbors = Vec::new();
        for coord in basic {
            if self.candidates.contains_key(&coord) {
                neighbors.push(coord);
            }
        }
        neighbors
    }

    fn chose_candidate(&mut self, index: usize) -> Coord {
        let neighbors = self.list_adjacent_candidates(index);
        let i = rand::thread_rng().gen_range(0..neighbors.len());
        self.candidates.remove(&neighbors[i]);
        neighbors[i]
    }

    pub fn generate(&mut self) -> IndexMap<Coord, Cell> {
        let mut rng = rand::thread_rng();
        while !self.unconnected.is_empty() {
            // generate a random number
            let index = rng.gen_range(0..(self.connected.len()));
            // add adjacent cells to the list of candidates
            self.save_candidates(index);
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
