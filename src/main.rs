use kiss3d::light::Light;
use kiss3d::nalgebra::{Point2, Point3};
use kiss3d::planar_camera::Sidescroll;
use kiss3d::window::Window;
use rand::prelude::*;
use std::cmp::Ordering;
use std::collections::BTreeMap;

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
                rand::thread_rng().gen_range(1..WIDTH),
                rand::thread_rng().gen_range(1..HEIGHT),
            ),
            false,
            false,
            None,
        )];

        for x in 0..(WIDTH + 1) {
            walls.insert(
                Coord::new(0, x),
                Cell::new(Coord::new(0, x), true, true, Some(0)),
            );
            walls.insert(
                Coord::new(HEIGHT, x),
                Cell::new(Coord::new(HEIGHT, x), true, true, Some(2)),
            );
        }
        for y in 1..HEIGHT {
            walls.insert(
                Coord::new(y, 0),
                Cell::new(Coord::new(y, 0), true, true, Some(3)),
            );
            walls.insert(
                Coord::new(y, WIDTH),
                Cell::new(Coord::new(y, WIDTH), true, true, Some(1)),
            );
        }
        // for y in 1..HEIGHT {
        //     for x in 1..WIDTH {
        //         walls.insert(Coord::new(y, x), Cell::new(Coord::new(y, x), None));
        //     }
        // }
        Maze { walls, paths }
    }

    fn get_candidate(&mut self, c: Coord) -> Cell {
        let n = c.get_neighbors(None);
        let mut index = rand::thread_rng().gen_range(0..4);

        while n[index].1 == false {
            index = rand::thread_rng().gen_range(0..4);
        }
    
        let candidate = self.walls[&n[index].0].clone();
        let mut cell = self.walls.get_mut(&n[index].0).unwrap();
    
        cell.computed = true;
        cell.is_wall = false;
        // println("{}", elf.walls.get_mut(&n[index].0).unwrap().computed);
        candidate
        // now cut the wall from other neighbors vectors
    }

    fn increment_path(&mut self) {
        // choose path cell to increment
        let index = rand::thread_rng().gen_range(0..(self.paths.len()));

        // get candidate
        let candidate = self.get_candidate(self.paths[index].c);

        // push candidate
        self.paths.push(candidate);
    }
}

fn main() {
    let mut window = Window::new("mazehem");
    let mut cam = Sidescroll::new();
    let maze = Maze::new();

    window.set_light(Light::StickToCamera);
    cam.set_at(Point2::new(
        ((WIDTH * 10) / 2) as f32,
        ((HEIGHT * 10) / 2) as f32,
    ));

    while !window.should_close() {
        for w in &maze.walls {
            window.draw_planar_line(
                &Point2::new((w.1.c.x * 10) as f32, (w.1.c.y * 10) as f32),
                &Point2::new((w.1.c.x * 10) as f32, ((w.1.c.y + 2) * 10) as f32),
                &Point3::new(1.0, 0.0, 0.0),
            )
        }
        window.render_with(None, Some(&mut cam), None);
    }
}
