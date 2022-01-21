use coffee::graphics::Mesh;

pub trait Drawable {
    fn draw(&self, mesh: &mut Mesh);
}

impl<T> Drawable for Vec<T>
where
    T: Drawable,
{
    fn draw(&self, mesh: &mut Mesh) {
        for elem in self {
            elem.draw(mesh);
        }
    }
}
