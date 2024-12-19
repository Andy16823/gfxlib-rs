pub trait Shape {
    fn get_vertex_buffer(&self) -> Vec<f32>;
    fn get_uv_buffer(&self) -> Vec<f32>;
    fn get_index_buffer(&self) -> Vec<u32>;
}

pub struct FramebufferShape;
impl Shape for FramebufferShape {

    //returns the vertex buffer data
    fn get_vertex_buffer(&self) -> Vec<f32> {
        let buffer: Vec<f32> = vec![
            -1.0, -1.0, 0.0,
            -1.0, 1.0, 0.0,
            1.0, 1.0, 0.0,
            1.0, -1.0, 0.0,
        ];
        return buffer;
    }

    //returns the uv buffer data
    fn get_uv_buffer(&self) -> Vec<f32> {
        let buffer: Vec<f32> = vec![
            0.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
            1.0, 0.0
        ];
        return buffer;
    }

    //returns the index buffer data
    fn get_index_buffer(&self) -> Vec<u32> {
        let buffer: Vec<u32> = vec![
            0, 1, 3,
            3, 1, 2
        ];
        return buffer;
    }

}

pub struct TextureShape;
impl Shape for TextureShape {

    //returns the vertex buffer data
    fn get_vertex_buffer(&self) -> Vec<f32> {
        let buffer: Vec<f32> = vec![
            -0.5, -0.5, 0.0,
            -0.5, 0.5, 0.0,
            0.5, 0.5, 0.0,
            0.5, -0.5, 0.0
        ];
        return buffer;
    }

    //returns the uv buffer
    fn get_uv_buffer(&self) -> Vec<f32> {
        let buffer: Vec<f32> = vec![
            0.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
            1.0, 0.0
        ];
        return buffer;
    }

    //returns the index buffer
    fn get_index_buffer(&self) -> Vec<u32> {
        let buffer: Vec<u32> = vec![
            0, 1, 3,
            3, 1, 2
        ];
        return  buffer;
    }    
    
}

pub struct RectShape;
impl Shape for RectShape {

    //returns the vertex buffer data
    fn get_vertex_buffer(&self) -> Vec<f32> {
        let buffer: Vec<f32> = vec![
            -0.5, -0.5, 0.0,
            -0.5, 0.5, 0.0,
            0.5, 0.5, 0.0,
            0.5, -0.5, 0.0
        ];
        return buffer;
    }

    //returns the uv buffer
    fn get_uv_buffer(&self) -> Vec<f32> {
        return Vec::new();
    }

    //returns the index buffer
    fn get_index_buffer(&self) -> Vec<u32> {
        let buffer: Vec<u32> = vec![
            0, 1, 3,
            3, 1, 2
        ];
        return  buffer;
    }    
    
}