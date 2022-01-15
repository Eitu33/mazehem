use crate::coord::Coord;
use crate::drawable::Drawable;
use coffee::graphics::{Color, Mesh, Rectangle, Shape};

pub struct Player {
    pub color: Color,
    pub coord: Coord,
}

impl Player {
    pub fn new(color: Color, coord: Coord) -> Player {
        Player { color, coord }
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
            self.color,
        );
    }
}
