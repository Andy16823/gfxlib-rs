use super::{material::Material, RenderData};

pub struct Mesh {
    pub vertices: Vec<f32>,
    pub uv_cords: Vec<f32>,
    pub indicies : Vec<u32>,
    pub material: Material,
    pub render_data : RenderData
}