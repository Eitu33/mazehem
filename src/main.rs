use coffee::graphics::{Color, Frame, Mesh, Rectangle, Shape, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Timer};
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
    pub computed: bool,
    pub c: Coord,
    pub n: Vec<Coord>,
}

impl Cell {
    fn new(c: Coord) -> Cell {
        Cell {
            computed: false,
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
        self.computed = true;
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
    maze
}

pub struct Maze {
    unconnected: HashMap<Coord, Coord>,
    candidates: HashMap<Coord, Coord>,
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
            let chosen = self.connected[nbr].chose_candidate(&mut self.candidates);
            // add candidate if it could be removed from the unconnected list
            if let Some(_) = self.unconnected.remove(&chosen) {
                self.connected.push(Cell::new(chosen.clone()));
                self.connected[nbr].push_neighbor(chosen);
            }
        }
        self.connected.clone()
    }
}

fn main() -> coffee::Result<()> {
    Mazehem::run(WindowSettings {
        title: String::from("Mazehem"),
        size: (590, 590),
        resizable: false,
        fullscreen: false,
        maximized: false,
    })
}

struct Mazehem {
    cells: Vec<Cell>,
}

impl Game for Mazehem {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Mazehem> {
        let mut maze = Maze::new(30, 30);
        let cells = maze.generate();
        Task::succeed(|| Mazehem { cells })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);
        let mut mesh = Mesh::new();
        for cell in &self.cells {
            for neighbor in &cell.n {
                if neighbor.x == cell.c.x && neighbor.y < cell.c.y {
                    mesh.fill(
                        Shape::Rectangle(Rectangle {
                            x: (cell.c.x * 20) as f32,
                            y: (cell.c.y * 20 - 20) as f32,
                            width: 10.0,
                            height: 30.0,
                        }),
                        Color::WHITE,
                    );
                } else if neighbor.x == cell.c.x && neighbor.y > cell.c.y {
                    mesh.fill(
                        Shape::Rectangle(Rectangle {
                            x: (cell.c.x * 20) as f32,
                            y: (cell.c.y * 20) as f32,
                            width: 10.0,
                            height: 30.0,
                        }),
                        Color::WHITE,
                    );
                } else if neighbor.y == cell.c.y && neighbor.x < cell.c.x {
                    mesh.fill(
                        Shape::Rectangle(Rectangle {
                            x: (cell.c.x * 20 - 20) as f32,
                            y: (cell.c.y * 20) as f32,
                            width: 30.0,
                            height: 10.0,
                        }),
                        Color::WHITE,
                    );
                } else if neighbor.y == cell.c.y && neighbor.x > cell.c.x {
                    mesh.fill(
                        Shape::Rectangle(Rectangle {
                            x: (cell.c.x * 20) as f32,
                            y: (cell.c.y * 20) as f32,
                            width: 30.0,
                            height: 10.0,
                        }),
                        Color::WHITE,
                    );
                }
            }
        }
        mesh.draw(&mut frame.as_target());
    }
}
