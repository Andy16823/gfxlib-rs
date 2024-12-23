use super::{material::Material, RenderData};

#[derive(Default)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub uv_cords: Vec<f32>,
    pub indicies : Vec<u32>,
    pub normals : Vec<f32>,
    pub material: Material,
    pub render_data : RenderData
}