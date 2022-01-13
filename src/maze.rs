use crate::cell::Cell;
use crate::coord::Coord;
use rand::Rng;
use std::collections::HashMap;

pub struct Maze {
    unconnected: HashMap<Coord, ()>,
    candidates: HashMap<Coord, ()>,
    connected: Vec<Cell>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let mut unconnected: HashMap<Coord, ()> = HashMap::new();
        for x in 0..width {
            for y in 0..height {
                unconnected.insert(Coord::new(x, y), ());
            }
        }
        Maze {
            candidates: HashMap::new(),
            unconnected,
            connected: vec![Cell::new(Coord::new(width / 2, height / 2))],
        }
    }

    pub fn generate(&mut self) -> Vec<Cell> {
        let mut rng = rand::thread_rng();
        while !self.unconnected.is_empty() {
            // generate a random number
            let index = rng.gen_range(0..(self.connected.len()));
            // add adjacent cells to the list of candidates
            self.connected[index].add_candidates(&mut self.candidates);
            // chose a candidate
            let chosen = self.connected[index].chose_candidate(&mut self.candidates);
            // add candidate if it could be removed from the unconnected list
            if self.unconnected.remove(&chosen).is_some() {
                self.connected.push(Cell::new(chosen));
                self.connected[index].push_neighbor(chosen);
            }
        }
        self.connected.clone()
    }
}
