use kiss3d::light::Light;
use kiss3d::nalgebra::{Point2, Point3};
use kiss3d::planar_camera::Sidescroll;
use kiss3d::window::Window;
use rand::prelude::*;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;

static HEIGHT: usize = 20;
static WIDTH: usize = 20;

#[derive(Clone, Eq, Hash, Copy)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }
    fn get_neighbors(&self, dead_index: Option<usize>) -> Vec<(Coord, bool)> {
        let mut neighbors = vec![
            (Coord::new(self.x.saturating_sub(1), self.y), true),
            (Coord::new(self.x.saturating_add(1), self.y), true),
            (Coord::new(self.x, self.y.saturating_sub(1)), true),
            (Coord::new(self.x, self.y.saturating_add(1)), true),
        ];
        if let Some(index) = dead_index {
            neighbors[index].1 = false;
        }
        neighbors
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let y_cmp = self.y.partial_cmp(&other.y);

        match y_cmp {
            Some(Ordering::Equal) => self.x.partial_cmp(&other.x),
            _ => y_cmp,
        }
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        let y_cmp = self.y.cmp(&other.y);

        match y_cmp {
            Ordering::Equal => self.x.cmp(&other.x),
            _ => y_cmp,
        }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x = {} | y = {}", self.x, self.y)
    }
}

#[derive(Clone)]
pub struct Cell {
    pub computed: bool,
    pub is_wall: bool,
    pub c: Coord,
    pub n: Vec<(Coord, bool)>,
}

impl Cell {
    fn new(c: Coord, computed: bool, is_wall: bool, dead_index: Option<usize>) -> Cell {
        Cell {
            computed,
            is_wall,
            c,
            n: c.get_neighbors(dead_index),
        }
    }
    fn draw_cell_as_wall(&self, window: &mut Window) {
        for a in &self.n {
            if a.1 {
                window.draw_planar_line(
                    &Point2::new((self.c.x * 10) as f32, (self.c.y * 10) as f32),
                    &Point2::new((a.0.x * 10) as f32, (a.0.y * 10) as f32),
                    &Point3::new(1.0, 0.0, 0.0),
                );
            }
        }
    }
}

pub struct Maze {
    pub walls: BTreeMap<Coord, Cell>,
    pub paths: Vec<Cell>,
}

impl Maze {
    fn new() -> Maze {
        let mut walls: BTreeMap<Coord, Cell> = BTreeMap::new();
        let paths = vec![Cell::new(
            Coord::new(
                rand::thread_rng().gen_range(1..(WIDTH - 1)),
                rand::thread_rng().gen_range(1..(HEIGHT - 1)),
            ),
            false,
            false,
            None,
        )];

        for x in 0..(WIDTH + 1) {
            walls.insert(
                Coord::new(x, 0),
                Cell::new(Coord::new(x, 0), true, true, Some(0)),
            );
            walls.insert(
                Coord::new(x, HEIGHT),
                Cell::new(Coord::new(x, HEIGHT), true, true, Some(2)),
            );
        }
        for y in 1..HEIGHT {
            walls.insert(
                Coord::new(0, y),
                Cell::new(Coord::new(0, y), true, true, Some(3)),
            );
            walls.insert(
                Coord::new(WIDTH, y),
                Cell::new(Coord::new(WIDTH, y), true, true, Some(1)),
            );
        }
        for y in 1..HEIGHT {
            for x in 1..WIDTH {
                walls.insert(
                    Coord::new(x, y),
                    Cell::new(Coord::new(x, y), false, true, None),
                );
            }
        }
        Maze { walls, paths }
    }

    fn remove_coord_from_adj_walls(&mut self, c: Coord) {
        let n = c.get_neighbors(None);

        for a in n {
            if a.1 && a.0.x < 21 {
                println!("{}", a.0);
                for b in &mut self.walls.get_mut(&a.0).unwrap().n {
                    if b.0 == a.0 {
                        b.1 = false;
                    }
                }
            }
        }
    }

    fn get_candidate(&mut self, c: Coord) -> Cell {
        println!("HELLO");
        let n = c.get_neighbors(None);
        let mut index = 0;

        while n[index].1 == false {
            index += 1;
        }

        let candidate = self.walls[&n[index].0].clone();
        let mut cell = self.walls.get_mut(&n[index].0).unwrap();

        cell.computed = true;
        cell.is_wall = false;
        self.remove_coord_from_adj_walls(candidate.c);
        // println!("{}", self.walls.get_mut(&n[index].0).unwrap().computed);
        candidate
    }

    fn increment_path(&mut self) {
        // choose path cell to increment
        let index = rand::thread_rng().gen_range(0..(self.paths.len()));

        // get candidate
        let candidate = self.get_candidate(self.paths[index].c);

        // push candidate
        self.paths.push(candidate);
        println!("{}", self.paths.len());
    }

    fn walls_computation(&self) -> bool {
        // println!("HELLO");
        !self.walls.clone().into_iter().all(|a| a.1.computed)
    }

    fn compute(&mut self) {
        while self.walls_computation() {
            self.increment_path();
        }
    }
}

fn main() {
    let mut window = Window::new("mazehem");
    let mut cam = Sidescroll::new();
    let mut maze = Maze::new();

    maze.compute();
    window.set_light(Light::StickToCamera);
    cam.set_at(Point2::new(
        ((WIDTH * 10) / 2) as f32,
        ((HEIGHT * 10) / 2) as f32,
    ));

    while !window.should_close() {
        for w in &maze.walls {
            w.1.draw_cell_as_wall(&mut window);
        }
        window.render_with(None, Some(&mut cam), None);
    }
}
