use crate::coord::Coord;
use crate::drawable::Drawable;
use coffee::graphics::{Color, Mesh, Rectangle, Shape};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub number: usize,
    pub coord: Coord,
    #[serde(skip)]
    pub color: Option<Color>,
}

impl Player {
    pub fn new(number: usize) -> Player {
        Player {
            number,
            coord: Coord::new(0, 0),
            color: None,
        }
    }

    pub fn init(&mut self) {
        match self.number {
            1 => {
                self.coord = Coord::new(0, 0);
                self.color = Some(Color::RED)
            }
            2 => {
                self.coord = Coord::new(29, 0);
                self.color = Some(Color::BLUE)
            }
            3 => {
                self.coord = Coord::new(0, 29);
                self.color = Some(Color::GREEN)
            }
            4 => {
                self.coord = Coord::new(29, 29);
                self.color = Some(Color::from_rgb_u32(0xDEC20B))
            }
            _ => (),
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
            self.color.unwrap_or(Color::WHITE),
        );
    }
}
