pub trait Shape {
    fn get_vertex_buffer(&self) -> Option<Vec<f32>>;
    fn get_uv_buffer(&self) -> Option<Vec<f32>>;
    fn get_index_buffer(&self) -> Option<Vec<u32>>;
    fn get_normal_buffer(&self) -> Option<Vec<f32>>;
}

pub struct FramebufferShape;
impl Shape for FramebufferShape {

    //returns the vertex buffer data
    fn get_vertex_buffer(&self) -> Option<Vec<f32>> {
        let buffer: Vec<f32> = vec![
            -1.0, -1.0, 0.0,
            -1.0, 1.0, 0.0,
            1.0, 1.0, 0.0,
            1.0, -1.0, 0.0,
        ];
        return Some(buffer);
    }

    //returns the uv buffer data
    fn get_uv_buffer(&self) -> Option<Vec<f32>> {
        let buffer: Vec<f32> = vec![
            0.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
            1.0, 0.0
        ];
        return Some(buffer);
    }

    //returns the index buffer data
    fn get_index_buffer(&self) -> Option<Vec<u32>> {
        let buffer: Vec<u32> = vec![
            0, 1, 3,
            3, 1, 2
        ];
        return Some(buffer);
    }

    fn get_normal_buffer(&self) -> Option<Vec<f32>> {
        return None;
    }

}

pub struct TextureShape;
impl Shape for TextureShape {

    //returns the vertex buffer data
    fn get_vertex_buffer(&self) -> Option<Vec<f32>> {
        let buffer: Vec<f32> = vec![
            -0.5, -0.5, 0.0,
            -0.5, 0.5, 0.0,
            0.5, 0.5, 0.0,
            0.5, -0.5, 0.0
        ];
        return Some(buffer);
    }

    //returns the uv buffer
    fn get_uv_buffer(&self) -> Option<Vec<f32>> {
        let buffer: Vec<f32> = vec![
            0.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
            1.0, 0.0
        ];
        return Some(buffer);
    }

    //returns the index buffer
    fn get_index_buffer(&self) -> Option<Vec<u32>> {
        let buffer: Vec<u32> = vec![
            0, 1, 3,
            3, 1, 2
        ];
        return  Some(buffer);
    }  

    fn get_normal_buffer(&self) -> Option<Vec<f32>> {
        return None;
    }  
    
}

pub struct RectShape;
impl Shape for RectShape {

    //returns the vertex buffer data
    fn get_vertex_buffer(&self) -> Option<Vec<f32>> {
        let buffer: Vec<f32> = vec![
            -0.5, -0.5, 0.0,
            -0.5, 0.5, 0.0,
            0.5, 0.5, 0.0,
            0.5, -0.5, 0.0
        ];
        return Some(buffer);
    }

    //returns the uv buffer
    fn get_uv_buffer(&self) -> Option<Vec<f32>> {
        return None;
    }

    //returns the index buffer
    fn get_index_buffer(&self) -> Option<Vec<u32>> {
        let buffer: Vec<u32> = vec![
            0, 1, 3,
            3, 1, 2
        ];
        return Some(buffer);
    }    

    fn get_normal_buffer(&self) -> Option<Vec<f32>> {
        return None;
    }
    
}