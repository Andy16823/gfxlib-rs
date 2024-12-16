extern crate gl;
extern crate glfw;
extern crate nalgebra;

pub mod image_texture;
pub mod render_device;
pub mod game_window;
pub mod material;
pub mod mesh;
pub mod camera;
pub mod viewport;
pub mod render_target;
pub mod shapes;

#[derive(Default, Clone, Copy)]
pub struct RenderData {
    pub vao : u32,
    pub vbo : u32,
    pub ibo : u32,
    pub tbo : u32,
    pub index_count: u32
}