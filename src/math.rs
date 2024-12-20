pub use nalgebra::*;

#[derive(Clone, Copy, Default)]
pub struct Rect<T> {
    pub x : T,
    pub y : T,
    pub widht: T,
    pub height : T
}