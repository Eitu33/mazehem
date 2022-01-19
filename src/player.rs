use crate::coord::Coord;
use crate::drawable::Drawable;
use coffee::graphics::{Color, Mesh, Rectangle, Shape};

#[derive(Debug, Clone)]
pub struct Player {
    pub coord: Coord,
    pub color: Color,
}

pub fn init_players() -> Vec<Player> {
    vec![
        Player::new(Coord::new(0, 0), Color::RED),
        Player::new(Coord::new(29, 0), Color::BLUE),
        Player::new(Coord::new(0, 29), Color::GREEN),
        Player::new(Coord::new(29, 29), Color::from_rgb_u32(0xDEC20B)),
    ]
}

impl Player {
    pub fn new(coord: Coord, color: Color) -> Player {
        Player { coord, color }
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
