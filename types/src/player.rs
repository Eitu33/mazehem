use crate::constants::{
    HEIGHT, P1_COLOR, P2_COLOR, P3_COLOR, P4_COLOR, SCALE, SQUARE_SIZE, WIDTH, WINDOW_OFFSET,
};
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
                2 => Coord::new(WIDTH - 1, 0),
                3 => Coord::new(0, HEIGHT - 1),
                4 => Coord::new(WIDTH - 1, HEIGHT - 1),
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
                2 => Color::from_rgb_u32(P2_COLOR),
                3 => Color::from_rgb_u32(P3_COLOR),
                4 => Color::from_rgb_u32(P4_COLOR),
                _ => Color::from_rgb_u32(P1_COLOR),
            }),
        }
    }
}

impl Drawable for Player {
    fn draw(&self, mesh: &mut Mesh) {
        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: (self.coord.x as f32) * SCALE + WINDOW_OFFSET,
                y: (self.coord.y as f32) * SCALE + WINDOW_OFFSET,
                width: SQUARE_SIZE,
                height: SQUARE_SIZE,
            }),
            self.color.unwrap_or(Color::WHITE),
        );
    }
}
