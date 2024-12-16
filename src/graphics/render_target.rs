use nalgebra::Vector2;

#[derive(Clone, Copy, Default)]
pub struct RenderTarget {
    pub size: Vector2<u32>,
    pub framebuffer_id: u32,
    pub texture_id: u32,
    pub renderbuffer_id: u32,
}
