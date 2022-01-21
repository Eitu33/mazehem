use coffee::graphics::Mesh;
use indexmap::IndexMap;

pub trait Drawable {
    fn draw(&self, mesh: &mut Mesh);
}

impl<K, T> Drawable for IndexMap<K, T>
where
    T: Drawable,
{
    fn draw(&self, mesh: &mut Mesh) {
        for cell in self {
            cell.1.draw(mesh);
        }
    }
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
