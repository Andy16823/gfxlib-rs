pub use nalgebra::*;

#[derive(Clone, Copy, Default)]
pub struct Rect<T> {
    pub x : T,
    pub y : T,
    pub widht: T,
    pub height : T
}

impl<T: Copy> Rect<T> {
    pub fn get_position_vector(&self) -> Vector2<T> {
        return Vector2::new(self.x, self.y)
    }

    pub fn get_size_vector(&self) -> Vector2<T> {
        return Vector2::new(self.widht, self.height);
    }
}