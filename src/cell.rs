use crate::coord::Coord;
use crate::drawable::Drawable;
use coffee::graphics::{Color, Mesh, Rectangle, Shape};

#[derive(Clone)]
pub struct Cell {
    pub computed: bool,
    pub c: Coord,
    pub n: Vec<Coord>,
}

impl Cell {
    pub fn new(c: Coord) -> Cell {
        Cell {
            computed: false,
            c,
            n: Vec::new(),
        }
    }

    pub fn get_basic_neighbors(&self) -> Vec<Coord> {
        vec![
            Coord::new(self.c.x.saturating_sub(1), self.c.y),
            Coord::new(self.c.x.saturating_add(1), self.c.y),
            Coord::new(self.c.x, self.c.y.saturating_sub(1)),
            Coord::new(self.c.x, self.c.y.saturating_add(1)),
        ]
    }

    pub fn push_neighbor(&mut self, coord: Coord) {
        self.n.push(coord);
    }
}

impl Drawable for Cell {
    fn draw(&self, mesh: &mut Mesh) {
        for neighbor in &self.n {
            let mut width = 10.0;
            let mut height = 10.0;
            let mut a = 0;
            let mut b = 0;
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
