use crate::constants::{LINK_OFFSET, LINK_SIZE, SCALE, SQUARE_SIZE, WINDOW_OFFSET};
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
            let (mut width, mut height) = (SQUARE_SIZE, SQUARE_SIZE);
            let (mut a, mut b) = (0.0, 0.0);
            if neighbor.x < self.coord.x {
                width = LINK_SIZE;
                a = LINK_OFFSET;
            } else if neighbor.x > self.coord.x {
                width = LINK_SIZE;
            } else if neighbor.y < self.coord.y {
                height = LINK_SIZE;
                b = LINK_OFFSET;
            } else if neighbor.y > self.coord.y {
                height = LINK_SIZE;
            }
            mesh.fill(
                Shape::Rectangle(Rectangle {
                    x: (self.coord.x as f32) * SCALE - a + WINDOW_OFFSET,
                    y: (self.coord.y as f32) * SCALE - b + WINDOW_OFFSET,
                    width,
                    height,
                }),
                Color::from_rgb_u32(0xecc28a),
            );
        }
    }
}
