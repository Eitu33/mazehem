use coffee::graphics::{Color, Frame, Mesh, Rectangle, Shape, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Timer};
use rand::Rng;
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
    fn add_candidates(&mut self, candidates: &mut HashMap<Coord, ()>) {
        let basic = self.get_basic_neighbors();
        for coord in basic {
            if coord != self.c && !candidates.contains_key(&coord) {
                candidates.insert(coord, ());
            }
        }
        self.computed = true;
    }
    fn find_neighbors(&mut self, candidates: &HashMap<Coord, ()>) -> Vec<Coord> {
        let basic = self.get_basic_neighbors();
        let mut neighbors = Vec::new();
        for coord in basic {
            if candidates.contains_key(&coord) {
                neighbors.push(coord.clone());
            }
        }
        neighbors
    }
    fn chose_candidate(&mut self, candidates: &mut HashMap<Coord, ()>) -> Coord {
        let neighbors = self.find_neighbors(candidates);
        let index = rand::thread_rng().gen_range(0..neighbors.len());
        candidates.remove(&neighbors[index]);
        neighbors[index]
    }
    fn push_neighbor(&mut self, coord: Coord) {
        self.n.push(coord.clone());
    }
    fn draw(&self, mesh: &mut Mesh) {
        let mut width: f32;
        let mut height: f32;
        let mut a: usize;
        let mut b: usize;
        for neighbor in &self.n {
            width = 10.0;
            height = 10.0;
            a = 0;
            b = 0;
            if neighbor.y == self.c.y && neighbor.x < self.c.x {
                width = 30.0;
                a = 20;
            } else if neighbor.y == self.c.y && neighbor.x > self.c.x {
                width = 30.0;
            } else if neighbor.x == self.c.x && neighbor.y < self.c.y {
                height = 30.0;
                b = 20;
            } else if neighbor.x == self.c.x && neighbor.y > self.c.y {
                height = 30.0;
            }
            mesh.fill(
                Shape::Rectangle(Rectangle {
                    x: (self.c.x * 20 - a) as f32,
                    y: (self.c.y * 20 - b) as f32,
                    width,
                    height,
                }),
                Color::WHITE,
            );
        }
    }
}

pub struct Maze {
    unconnected: HashMap<Coord, ()>,
    candidates: HashMap<Coord, ()>,
    connected: Vec<Cell>,
}

impl Maze {
    fn new(width: usize, height: usize) -> Maze {
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
        let mut mesh = Mesh::new();
        frame.clear(Color::BLACK);
        for cell in &self.cells {
            cell.draw(&mut mesh);
        }
        mesh.draw(&mut frame.as_target());
    }
}
