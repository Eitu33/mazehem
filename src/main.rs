use kiss3d::light::Light;
use kiss3d::nalgebra::{Point2, Point3};
use kiss3d::planar_camera::Sidescroll;
use kiss3d::window::Window;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Eq, Hash, Copy)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Clone)]
pub struct Cell {
    pub cmptd: bool,
    pub c: Coord,
    pub n: Vec<Coord>,
}

impl Cell {
    fn new(c: Coord) -> Cell {
        Cell {
            cmptd: false,
            c,
            n: Vec::new(),
        }
    }
    fn get_basic_neighbors(&mut self) -> Vec<Coord> {
        vec![
            Coord::new(self.c.x.saturating_sub(1), self.c.y),
            Coord::new(self.c.x.saturating_add(1), self.c.y),
            Coord::new(self.c.x, self.c.y.saturating_sub(1)),
            Coord::new(self.c.x, self.c.y.saturating_add(1)),
        ]
    }
    fn add_candidates(&mut self, candidates: &mut HashMap<Coord, Coord>) {
        let basic = self.get_basic_neighbors();
        for coord in basic {
            if coord != self.c && !candidates.contains_key(&coord) {
                candidates.insert(coord, coord);
            }
        }
        self.cmptd = true;
    }
    fn find_neighbors(&mut self, candidates: &HashMap<Coord, Coord>) -> Vec<Coord> {
        let basic = self.get_basic_neighbors();
        let mut neighbors = Vec::new();
        for n in basic {
            if let Some(c) = candidates.get(&n) {
                neighbors.push(c.clone());
            }
        }
        neighbors
    }
    fn chose_candidate(&mut self, candidates: &mut HashMap<Coord, Coord>) -> Coord {
        let neighbors = self.find_neighbors(candidates);
        let nbr = rand::thread_rng().gen_range(0..neighbors.len());
        candidates.remove(&neighbors[nbr]);
        neighbors[nbr]
    }
    fn push_neighbor(&mut self, coord: Coord) {
        self.n.push(coord.clone());
    }
}

fn init(width: usize, height: usize) -> HashMap<Coord, Coord> {
    let mut maze: HashMap<Coord, Coord> = HashMap::new();
    for x in 0..width {
        for y in 0..height {
            maze.insert(Coord::new(x, y), Coord::new(x, y));
        }
    }
    maze.remove(&Coord::new(width / 2, height / 2));
    maze
}

pub struct Maze {
    candidates: HashMap<Coord, Coord>,
    unconnected: HashMap<Coord, Coord>,
    connected: Vec<Cell>,
}

impl Maze {
    fn new(width: usize, height: usize) -> Maze {
        Maze {
            candidates: HashMap::new(),
            unconnected: init(width, height),
            connected: vec![Cell::new(Coord::new(width / 2, height / 2))],
        }
    }
    fn generate(&mut self) -> Vec<Cell> {
        let mut rng = rand::thread_rng();

        while !self.unconnected.is_empty() {
            // generate a random number
            let nbr = rng.gen_range(0..(self.connected.len()));
            // add adjacent cells to the list of candidates
            self.connected[nbr].add_candidates(&mut self.candidates);
            // chose a candidate
            let candidate = self.connected[nbr].chose_candidate(&mut self.candidates);
            // add candidate if it could be removed from the unconnected list
            if let Some(_) = self.unconnected.remove(&candidate) {
                self.connected.push(Cell::new(candidate.clone()));
                self.connected[nbr].push_neighbor(candidate);
            }
        }
        self.connected.clone()
    }
}

fn main() {
    let width = 42;
    let height = 42;
    let mut maze = Maze::new(width, height);
    let cells = maze.generate();
    let mut window = Window::new("mazehem");
    window.set_light(Light::StickToCamera);

    let mut cam = Sidescroll::new();
    cam.set_at(Point2::new(
        ((width * 30) / 2) as f32,
        ((height * 30) / 2) as f32,
    ));
    let mut sup: usize;
    while !window.should_close() {
        for v in &cells {
            for v2 in &v.n {
                for i in 0..100 {
                    let a = i as f32 * 0.1;
                    if v.c.x == v2.x {
                        if v2.y > v.c.y {
                            sup = 10;
                        } else {
                            sup = 0;
                        }
                        window.draw_planar_line(
                            &Point2::new(((v.c.x * 30) + 0) as f32 + a, ((v.c.y * 30) + sup) as f32),
                            &Point2::new(((v2.x * 30) + 0) as f32 + a, ((v2.y * 30) + sup) as f32),
                            &Point3::new(0.0, 1.0, 0.0),
                        )
                    } else if v.c.y == v2.y {
                        if v2.x > v.c.x {
                            sup = 10;
                        } else {
                            sup = 0;
                        }
                        window.draw_planar_line(
                            &Point2::new(((v.c.x * 30) + sup) as f32, ((v.c.y * 30) + 0) as f32 + a),
                            &Point2::new(((v2.x * 30) + sup) as f32, ((v2.y * 30) + 0) as f32 + a),
                            &Point3::new(1.0, 0.0, 0.0),
                        )
                    }
                }
            }
        }
        window.render_with(None, Some(&mut cam), None);
    }
}
