use kiss3d::light::Light;
use kiss3d::nalgebra::{Point2, Point3};
use kiss3d::planar_camera::Sidescroll;
use kiss3d::window::Window;
use rand::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Eq, Hash, Copy, Default)]
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

#[derive(Clone, Default)]
pub struct Cell {
    pub is_path: bool,
    pub cmptd: bool,
    pub c: Coord,
    pub n: Vec<Coord>,
}

impl Cell {
    fn new(c: Coord) -> Cell {
        Cell {
            is_path: false,
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
    fn set_to_path(&mut self) {
        self.is_path = true;
    }
}

fn init(width: usize, height: usize) -> HashMap<Coord, Cell> {
    let mut maze: HashMap<Coord, Cell> = HashMap::new();
    for x in 0..width {
        for y in 0..height {
            let c = Coord::new(x, y);
            maze.insert(c, Cell::new(c));
        }
    }
    maze
}

fn main() {
    let mut window = Window::new("mazehem");
    window.set_light(Light::StickToCamera);

    let width = 21;
    let height = 21;

    let mut candidates: HashMap<Coord, Coord> = HashMap::new();
    let mut maze: HashMap<Coord, Cell> = init(width, height);
    let mut connected: Vec<Coord> = Vec::new();

    let first = Coord::new(width / 2, height / 2);
    connected.push(first);

    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        // generate a random number
        let nbr = rng.gen_range(0..(connected.len()));
        // add adjacent cells to the list of candidates
        maze.get_mut(&connected[nbr])
            .unwrap()
            .add_candidates(&mut candidates);
        // chose a candidate
        let cand = maze
            .get_mut(&connected[nbr])
            .unwrap()
            .chose_candidate(&mut candidates);
        // HERE CHANGE
        let one = maze.get_mut(&cand).unwrap();
        if one.is_path == false {
            one.set_to_path();
            connected.push(one.c);
            maze.get_mut(&connected[nbr]).unwrap().push_neighbor(cand);
        }
    }

    let mut cam = Sidescroll::new();
    cam.set_at(Point2::new(
        ((width * 10) / 2) as f32,
        ((height * 10) / 2) as f32,
    ));
    while !window.should_close() {
        for v in &maze {
            for v2 in &v.n {
                window.draw_planar_line(
                    &Point2::new((v.c.x * 10) as f32, (v.c.y * 10) as f32),
                    &Point2::new((v2.x * 10) as f32, (v2.y * 10) as f32),
                    &Point3::new(1.0, 0.0, 0.0),
                )
            }
        }
        window.render_with(None, Some(&mut cam), None);
    }
}
