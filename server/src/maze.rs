use indexmap::IndexMap;
use rand::Rng;
use std::collections::HashMap;
use types::cell::Cell;
use types::coord::Coord;

pub struct Maze {
    unconnected: HashMap<Coord, ()>,
    connected: IndexMap<Coord, Cell>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let starting_coord = Coord::new(width / 2, height / 2);
        let connected: IndexMap<Coord, Cell> =
            IndexMap::from([(starting_coord, Cell::new(starting_coord))]);
        let mut unconnected: HashMap<Coord, ()> = HashMap::new();
        for x in 0..width {
            for y in 0..height {
                unconnected.insert(Coord::new(x, y), ());
            }
        }
        unconnected.remove(&starting_coord);
        Maze {
            unconnected,
            connected,
        }
    }

    fn list_candidates(&mut self, index: usize) -> Vec<Coord> {
        let mut candidates = self.connected[index].get_adjacent_coords();
        candidates.retain(|x| x != &self.connected[index].coord);
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
