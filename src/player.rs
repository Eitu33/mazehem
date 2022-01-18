use crate::coord::Coord;
use crate::drawable::Drawable;
use coffee::graphics::{Color, Mesh, Rectangle, Shape};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub number: usize,
    pub coord: Coord,
}

impl Player {
    pub fn new(number: usize) -> Player {
        Player {
            number,
            coord: match number {
                1 => Coord::new(0, 0),
                _ => Coord::new(29, 29),
            },
        }
    }
}

impl Drawable for Player {
    fn draw(&self, mesh: &mut Mesh) {
        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: (self.coord.x * 20) as f32,
                y: (self.coord.y * 20) as f32,
                width: 10.0,
                height: 10.0,
            }),
            match self.number {
                1 => Color::RED,
                _ => Color::BLUE,
            },
        );
    }
}
