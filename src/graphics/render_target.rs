use nalgebra::Vector2;

use super::RenderData;

#[derive(Clone, Copy, Default)]
pub struct RenderTarget {
    pub size: Vector2<u32>,
    pub framebuffer_id: u32,
    pub texture_id: u32,
    pub renderbuffer_id: u32,
    pub render_data : RenderData
}

impl RenderTarget {

    pub fn get_verticies() -> Vec<f32> {
        let vertices: Vec<f32> = vec![
            -1.0, -1.0, 0.0,    //down left
            -1.0, 1.0, 0.0,     //up left
            1.0, 1.0, 0.0,      //up right
            1.0, -1.0, 0.0,     //down right

            0.0, 0.0,           //uv down left
            0.0, 1.0,           //uv up left
            1.0, 1.0,           //uv up right
            1.0, 0.0            //uv down right
        ];
        return vertices;
    }

    pub fn get_indices() -> Vec<u32> {
        let indices: Vec<u32> = vec![
            0, 1, 3,
            3, 1, 2
        ];
        return indices;
    }
}
