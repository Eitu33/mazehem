use coffee::graphics::Mesh;

pub trait Drawable {
    fn draw(&self, mesh: &mut Mesh);
}