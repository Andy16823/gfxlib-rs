use super::material::Material;

#[derive(Default)]
pub struct RenderData {
    pub vao : u32,
    pub vbo : u32,
    pub ibo : u32,
    pub tbo : u32
}

pub struct Mesh {
    pub vertices: Vec<f32>,
    pub uv_cords: Vec<f32>,
    pub indicies : Vec<u32>,
    pub material: Material,
    pub render_data : RenderData
}