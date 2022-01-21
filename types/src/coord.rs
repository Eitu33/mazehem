use crate::drawable::Drawable;
use coffee::graphics::{Color, Mesh, Rectangle, Shape};
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }

    pub fn out_of_bounds(&self, x: usize, y: usize) -> bool {
        self.x >= x || self.y >= y
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Drawable for Coord {
    fn draw(&self, mesh: &mut Mesh) {
        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: (self.x * 20 + 10) as f32,
                y: (self.y * 20 + 10) as f32,
                width: 10.0,
                height: 10.0,
            }),
            Color::from_rgb_u32(0x71ffec),
        );
    }
}
