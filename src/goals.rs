use crate::coord::Coord;
use crate::drawable::Drawable;
use coffee::graphics::{Color, Mesh, Rectangle, Shape};

pub struct Goals {
    goals: Vec<Coord>,
}

impl Goals {
    pub fn new(goals: Vec<Coord>) -> Goals {
        Goals {
            goals,
        }
    }
}

impl Drawable for Goals {
    fn draw(&self, mesh: &mut Mesh) {
        for goal in &self.goals {
            mesh.fill(
                Shape::Rectangle(Rectangle {
                    x: (goal.x * 20) as f32,
                    y: (goal.y * 20) as f32,
                    width: 10.0,
                    height: 10.0,
                }),
                Color::RED,
            );
        }
    }
}
