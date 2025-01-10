use nalgebra::{Matrix4, Vector4};

extern crate gl;
extern crate glfw;
extern crate nalgebra;

pub mod camera;
pub mod game_window;
pub mod image_texture;
pub mod spritesheet;
pub mod material;
pub mod mesh;
pub mod render_device;
pub mod render_target;
pub mod shapes;
pub mod viewport;
pub mod font;
pub mod light;

/// Represents flags for the text alignment
pub enum TextAlignment {
    BottomLeft,
    BottomCenter,
    BottomRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    TopLeft,
    TopCenter,
    TopRight
}

/// Represents data required for rendering, such as vertex array, buffer objects, and index count.
#[derive(Default, Clone, Copy)]
pub struct RenderData {
    pub vao: u32,
    pub vbo: u32,
    pub ibo: u32,
    pub tbo: u32,
    pub nbo: u32,
    pub tabo: u32,
    pub index_count: u32,
}

/// Represents an instance of a 2D texture with its transformation matrix, color, and UV transformation.
#[derive(Default, Clone)]
pub struct Texture2DInstance {
    pub transform: Matrix4<f32>,
    pub color: Vector4<f32>,
    pub uv_transform: Vector4<f32>,
    pub visible : bool
}


impl Texture2DInstance {

    /// Creates a new `Texture2DInstance` with the given transformation, color, and UV transformation.
    ///
    /// # Arguments
    /// - `transform`: The transformation matrix.
    /// - `color`: The RGBA color vector.
    /// - `uv_transform`: The UV transformation vector.
    ///
    /// # Returns
    /// - A new `Texture2DInstance` object.
    pub fn new(transform : Matrix4<f32>, color : Vector4<f32>, uv_transform : Vector4<f32>, visible : bool) -> Texture2DInstance {
        Texture2DInstance{
            transform, color, uv_transform, visible
        }
    }

    pub fn create_extras_vec4(&self) -> Vector4<f32> {
        let visible = self.visible as i32;
        let extras = Vector4::new(visible as f32, 0.0, 0.0, 0.0);
        return extras;
    }

    /// Provides a default UV transformation vector.
    ///
    /// # Returns
    /// - A `Vector4<f32>` representing the default UV transformation `[1.0, 1.0, 0.0, 0.0]`.
    pub fn default_uv_transform() -> Vector4<f32> {
        return Vector4::new(1.0, 1.0, 0.0, 0.0);
    }

}

/// Represents a batch of 2D textures, which can be in either a preloaded or loaded state.
#[derive(Clone)]
pub enum Texture2DBatch {
    /// Preloaded state, where texture instances are stored but no GPU buffers are allocated yet.
    PreLoad {
        instances: Vec<Texture2DInstance>,
    },
    /// Loaded state, where GPU buffers are allocated for texture instances.
    Loaded {
        instances: Vec<Texture2DInstance>,
        mbo: u32,
        cbo: u32,
        uvto: u32,
        exbo: u32
    },
    /// Disposed state
    Disposed {
        instances: Vec<Texture2DInstance>
    }
}


impl Texture2DBatch {

    /// Creates a new `Texture2DBatch` in the `PreLoad` state.
    ///
    /// # Returns
    /// - A new `Texture2DBatch` object in the preloaded state.
    pub fn new() -> Self {
        Texture2DBatch::PreLoad {
            instances: Vec::new(),
        }
    }

    /// Adds a new texture instance to the batch in the `PreLoad` state.
    ///
    /// # Arguments
    /// - `transform`: The transformation matrix for the new instance.
    /// - `color`: The RGBA color vector for the new instance.
    /// - `uv_transform`: The UV transformation vector for the new instance.
    ///
    /// # Returns
    /// - The number of instances in the batch after adding, or -1 if the batch is not in the `PreLoad` state.
    pub fn add_instance(&mut self, transform : Matrix4<f32>, color : Vector4<f32>, uv_transform : Vector4<f32>, visible : bool) -> i32 {
        match self {
            Texture2DBatch::PreLoad { instances } => {
                instances.push(Texture2DInstance::new(transform, color, uv_transform, visible));
                return instances.len() as i32 + 1;
            }
            _ => {
                return -1;
            }
        }
    }

    /// Generates GPU-ready buffers for transformations, colors, and UV transformations.
    ///
    /// # Arguments
    /// - `instances`: A mutable reference to the vector of `Texture2DInstance` objects.
    ///
    /// # Returns
    /// - A tuple of three `Vec<f32>` buffers:
    ///   - Transform buffer: Contains the transformation matrices.
    ///   - Color buffer: Contains the RGBA color values.
    ///   - UV transform buffer: Contains the UV transformation vectors.
    pub fn create_buffers(instances: &mut Vec<Texture2DInstance>) -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>) {
        let mut transform_buffer = Vec::new();
        let mut color_buffer = Vec::new();
        let mut uv_transform_buffer = Vec::new();
        let mut extras_buffer = Vec::new();

        for instance in instances {
            let matrix = instance.transform;
            transform_buffer.extend_from_slice(matrix.as_slice());
            color_buffer.extend_from_slice(instance.color.as_slice());
            uv_transform_buffer.extend_from_slice(instance.uv_transform.as_slice());
            extras_buffer.extend_from_slice(instance.create_extras_vec4().as_slice());
        }

        (transform_buffer, color_buffer, uv_transform_buffer, extras_buffer)
    }
}

