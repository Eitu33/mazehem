use crate::coord::Coord;
use crate::drawable::Drawable;
use coffee::graphics::{Color, Mesh, Rectangle, Shape};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Cell {
    pub coord: Coord,
    pub n: Vec<Coord>,
}

impl Cell {
    pub fn new(coord: Coord) -> Cell {
        Cell {
            coord,
            n: Vec::new(),
        }
    }

    pub fn get_adjacent_coords(&self) -> Vec<Coord> {
        vec![
            Coord::new(self.coord.x.saturating_sub(1), self.coord.y),
            Coord::new(self.coord.x.saturating_add(1), self.coord.y),
            Coord::new(self.coord.x, self.coord.y.saturating_sub(1)),
            Coord::new(self.coord.x, self.coord.y.saturating_add(1)),
        ]
    }

    pub fn push_neighbor(&mut self, coord: Coord) {
        self.n.push(coord);
    }
}

impl Drawable for Cell {
    fn draw(&self, mesh: &mut Mesh) {
        for neighbor in &self.n {
            let (mut width, mut height) = (10.0, 10.0);
            let (mut a, mut b) = (0, 0);
            if neighbor.x < self.coord.x {
                width = 30.0;
                a = 20;
            } else if neighbor.x > self.coord.x {
                width = 30.0;
            } else if neighbor.y < self.coord.y {
                height = 30.0;
                b = 20;
            } else if neighbor.y > self.coord.y {
                height = 30.0;
            }
            mesh.fill(
                Shape::Rectangle(Rectangle {
                    x: (self.coord.x * 20 - a) as f32,
                    y: (self.coord.y * 20 - b) as f32,
                    width,
                    height,
                }),
                Color::from_rgb_u32(0xC0C0C0),
            );
        }
    }
}
