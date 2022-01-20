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

pub fn init_players() -> Vec<Player> {
    let mut players = Vec::new();
    for i in 1..5 {
        players.push(Player::new(i).colored());
    }
    players
}

impl Player {
    pub fn new(number: usize) -> Player {
        Player {
            number,
            coord: match number {
                2 => Coord::new(29, 0),
                3 => Coord::new(0, 29),
                4 => Coord::new(29, 29),
                _ => Coord::new(0, 0),
            },
            color: None,
        }
    }

    pub fn colored(&self) -> Player {
        Player {
            number: self.number,
            coord: self.coord,
            color: Some(match self.number {
                2 => Color::BLUE,
                3 => Color::GREEN,
                4 => Color::from_rgb_u32(0xDEC20B),
                _ => Color::RED,
            }),
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
