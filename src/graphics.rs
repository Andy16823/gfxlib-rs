use nalgebra::{Matrix4, Vector4};

extern crate gl;
extern crate glfw;
extern crate nalgebra;

pub mod camera;
pub mod game_window;
pub mod image_texture;
pub mod material;
pub mod mesh;
pub mod render_device;
pub mod render_target;
pub mod shapes;
pub mod viewport;

#[derive(Default, Clone, Copy)]
pub struct RenderData {
    pub vao: u32,
    pub vbo: u32,
    pub ibo: u32,
    pub tbo: u32,
    pub index_count: u32,
}

#[derive(Default, Clone)]
pub struct RenderInstance {
    pub transform: Matrix4<f32>,
    pub color: Vector4<f32>,
}

impl RenderInstance {
    pub fn new(transform : Matrix4<f32>, color : Vector4<f32>) -> RenderInstance {
        RenderInstance{
            transform, color
        }
    }
}

#[derive(Clone)]
pub enum InstanceBatch {
    PreLoad {
        instances: Vec<RenderInstance>,
    },
    Loaded {
        instances: Vec<RenderInstance>,
        mbo: u32,
        cbo: u32
    },
}

impl InstanceBatch {
    pub fn new() -> Self {
        InstanceBatch::PreLoad {
            instances: Vec::new(),
        }
    }

    pub fn add_instance(&mut self, transform : Matrix4<f32>, color : Vector4<f32>) -> i32 {
        match self {
            InstanceBatch::PreLoad { instances } => {
                instances.push(RenderInstance::new(transform, color));
                return instances.len() as i32 + 1;
            }
            _ => {
                return -1;
            }
        }
    }

    pub fn create_buffers(instances: &mut Vec<RenderInstance>) -> (Vec<f32>, Vec<f32>) {
        let mut transform_buffer = Vec::new();
        let mut color_buffer = Vec::new();

        for instance in instances {
            let matrix = instance.transform;
            transform_buffer.extend_from_slice(matrix.as_slice());
            color_buffer.extend_from_slice(instance.color.as_slice());
        }

        (transform_buffer, color_buffer)
    }
}
