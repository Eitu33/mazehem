use kiss3d::light::Light;
use kiss3d::nalgebra::{Point2, Point3};
use kiss3d::window::Window;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Eq, Hash, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    fn new(mut x: i32, mut y: i32) -> Coord {
        if x < 0 {
            x = 0;
        }
        if y < 0 {
            y = 0;
        }
        Coord { x, y }
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct Cell {
    pub cptd: bool,
    pub c: Coord,
    pub n: Vec<Coord>,
}

impl Cell {
    fn new(c: Coord) -> Cell {
        Cell {
            cptd: false,
            c,
            n: vec![],
        }
    }
    fn get_basic_neighbors(&mut self) -> Vec<Coord> {
        vec![
            Coord::new(self.c.x - 1, self.c.y),
            Coord::new(self.c.x + 1, self.c.y),
            Coord::new(self.c.x, self.c.y - 1),
            Coord::new(self.c.x, self.c.y + 1),
        ]
    }
    fn add_candidates(&mut self, candidates: &mut HashMap<Coord, Coord>) {
        let neighbors = self.get_basic_neighbors();
        for coord in neighbors {
            if coord != self.c && !candidates.contains_key(&coord) {
                candidates.insert(coord, coord);
            }
        }
        self.cptd = true;
    }
    fn find_neighbors(&mut self, candidates: &HashMap<Coord, Coord>) -> Vec<Coord> {
        let neighbors = self.get_basic_neighbors();
        let mut good = Vec::new();
        for n in neighbors {
            if let Some(a) = candidates.get(&n) {
                good.push(a.clone());
            }
        }
        good
    }
    fn chose_candidate(&mut self, candidates: &mut HashMap<Coord, Coord>) -> Coord {
        let good = self.find_neighbors(candidates);
        let a = rand::thread_rng().gen_range(0..good.len());
        // self.n.push(good[a].clone());
        candidates.remove(&good[a]);
        good[a]
    }
    fn push_neighbor(&mut self, coord: Coord) {
        self.n.push(coord.clone());
    }
}

fn init(width: usize, height: usize) -> HashMap<Coord, Coord> {
    let mut maze: HashMap<Coord, Coord> = HashMap::new();
    for x in 0..width {
        for y in 0..height {
            maze.insert(
                Coord::new(x as i32, y as i32),
                Coord::new(x as i32, y as i32),
            );
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
    let mut unconnected: HashMap<Coord, Coord> = init(width, height);
    let mut connected: Vec<Cell> = vec![Cell::new(Coord::new(width as i32 / 2, height as i32 / 2))];
    unconnected.remove(&Coord::new(width as i32 / 2, height as i32 / 2));
    let mut rng = rand::thread_rng();
    while !unconnected.is_empty() {
        let a = rng.gen_range(0..(connected.len()));
        connected[a].add_candidates(&mut candidates);
        // is neighbor
        let cand = connected[a].chose_candidate(&mut candidates);
        // end
        if let Some(_) = unconnected.remove(&cand) {
            connected.push(Cell::new(cand.clone()));
            connected[a].push_neighbor(cand);
        }
    }

    while window.render() {
        for v in &connected {
            for v2 in &v.n {
                window.draw_planar_line(
                    &Point2::new((v.c.x * 10) as f32, (v.c.y * 10) as f32),
                    &Point2::new((v2.x * 10) as f32, (v2.y * 10) as f32),
                    &Point3::new(1.0, 0.0, 0.0),
                )
            }
        }
    }
}
