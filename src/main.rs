use kiss3d::light::Light;
use kiss3d::nalgebra::{Point2, Point3};
use kiss3d::planar_camera::Sidescroll;
use kiss3d::window::Window;
use rand::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Pos {
    Top,
    Bot,
    Left,
    Right,
}

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

pub struct Cell {
    pub cmptd: bool,
    pub c: Coord,
    pub n: HashMap<Pos, Coord>,
}

impl Cell {
    fn new(c: Coord) -> Cell {
        Cell {
            cmptd: false,
            c,
            n: HashMap::new(),
        }
    }
    fn get_basic_neighbors(&mut self) -> HashMap<Pos, Coord> {
        let mut n = HashMap::new();
        n.insert(Pos::Top, Coord::new(self.c.x.saturating_sub(1), self.c.y));
        n.insert(Pos::Bot, Coord::new(self.c.x.saturating_add(1), self.c.y));
        n.insert(Pos::Left, Coord::new(self.c.x, self.c.y.saturating_sub(1)));
        n.insert(Pos::Right, Coord::new(self.c.x, self.c.y.saturating_add(1)));
        n
    }
    fn add_candidates(&mut self, candidates: &mut HashMap<Coord, Coord>) {
        let basic = self.get_basic_neighbors();
        for (_, coord) in basic {
            if coord != self.c && !candidates.contains_key(&coord) {
                candidates.insert(coord, coord);
            }
        }
        self.cmptd = true;
    }
    fn find_neighbors(&mut self, candidates: &HashMap<Coord, Coord>) -> Vec<(Pos, Coord)> {
        let basic = self.get_basic_neighbors();
        let mut neighbors = Vec::new();
        for (pos, coord) in basic {
            if let Some(_) = candidates.get(&coord) {
                neighbors.push((pos, coord));
            }
        }
        neighbors
    }
    // Add Option Here
    fn chose_candidate(&mut self, candidates: &mut HashMap<Coord, Coord>) -> (Pos, Coord) {
        let neighbors = self.find_neighbors(candidates);
        let nbr = rand::thread_rng().gen_range(0..neighbors.len());
        candidates.remove(&neighbors[nbr].1);
        neighbors[nbr]
    }
    fn push_neighbor(&mut self, t: (Pos, Coord)) {
        self.n.insert(t.0, t.1);
    }
    fn draw_cell(&self, window: &mut Window, color: Point3<f32>) {
        let here = 5;
        if let None = self.n.get(&Pos::Top) {
            window.draw_planar_line(
                &Point2::new((self.c.x * 10 - here) as f32, (self.c.y * 10 + here) as f32),
                &Point2::new((self.c.x * 10 + here) as f32, (self.c.y * 10 + here) as f32),
                &color,
            )
        }
        // if let None = self.n.get(&Pos::Bot) {
        //     window.draw_planar_line(
        //         &Point2::new((self.c.x * 10 - here) as f32, (self.c.y * 10 - here) as f32),
        //         &Point2::new((self.c.x * 10 + here) as f32, (self.c.y * 10 - here) as f32),
        //         &color,
        //     )
        // }
        // if let None = self.n.get(&Pos::Left) {
        //     window.draw_planar_line(
        //         &Point2::new((self.c.x * 10 - here) as f32, (self.c.y * 10 - here) as f32),
        //         &Point2::new((self.c.x * 10 - here) as f32, (self.c.y * 10 + here) as f32),
        //         &color,
        //     )
        // }
        // if let None = self.n.get(&Pos::Right) {
        //     window.draw_planar_line(
        //         &Point2::new((self.c.x * 10 + here) as f32, (self.c.y * 10 - here) as f32),
        //         &Point2::new((self.c.x * 10 + here) as f32, (self.c.y * 10 + here) as f32),
        //         &color,
        //     )
        // }
    }
}

fn init(width: usize, height: usize) -> HashMap<Coord, Coord> {
    let mut maze: HashMap<Coord, Coord> = HashMap::new();
    for x in 1..(width + 1) {
        for y in 1..(height + 1) {
            maze.insert(Coord::new(x, y), Coord::new(x, y));
        }
    }
    maze
}

fn main() {
    let mut window = Window::new("mazehem");
    window.set_light(Light::StickToCamera);

    let width = 10;
    let height = 10;

    let mut candidates: HashMap<Coord, Coord> = HashMap::new();
    let mut unconnected: HashMap<Coord, Coord> = init(width, height);
    let mut connected: Vec<Cell> = vec![Cell::new(Coord::new(width / 2, height / 2))];
    unconnected.remove(&Coord::new(width / 2, height / 2));
    let mut rng = rand::thread_rng();
    while !unconnected.is_empty() {
        // generate a random number
        let nbr = rng.gen_range(0..(connected.len()));
        // add adjacent cells to the list of candidates
        connected[nbr].add_candidates(&mut candidates);
        // chose a candidate
        let cand = connected[nbr].chose_candidate(&mut candidates);
        // add candidate if it could be removed from the unconnected list
        if let Some(_) = unconnected.remove(&cand.1) {
            connected.push(Cell::new(cand.1));
            connected[nbr].push_neighbor(cand);
        }
    }
    println!("HERE: {}", connected.len());

    let mut cam = Sidescroll::new();
    cam.set_at(Point2::new(
        ((width * 10)) as f32,
        ((height * 10)) as f32,
    ));
    while !window.should_close() {
        for v in &connected {
            v.draw_cell(&mut window, Point3::new(1.0, 0.0, 0.0));
            for v2 in &v.n {
                window.draw_planar_line(
                    &Point2::new((v.c.x * 10) as f32, (v.c.y * 10) as f32),
                    &Point2::new((v2.1.x * 10) as f32, (v2.1.y * 10) as f32),
                    &Point3::new(0.0, 1.0, 0.0),
                )
            }
        }
        window.render_with(None, Some(&mut cam), None);
    }
}
