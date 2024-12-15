use nalgebra::{Matrix4, Vector3, Vector4};
use uuid::Uuid;

use crate::{image_texture::ImageTexture, material::Material, mesh::Mesh};

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>
}

impl Transform {

    pub fn translate(&mut self, vec : Vector3<f32>) {
        self.position = self.position + vec;
    }

    pub fn translate_xyz(&mut self, x : f32, y : f32, z : f32) {
        self.position = self.position + Vector3::new(x,y,z);
    }

    pub fn translate_xy(&mut self, x : f32, y : f32) {
        self.position = self.position + Vector3::new(x, y, 0.0);
    }

    pub fn get_model_matrix(&mut self) -> Matrix4<f32> {
        let translation = Matrix4::new_translation(&self.position);
        let rotation = Matrix4::new_rotation(self.rotation);
        let scale = Matrix4::new_nonuniform_scaling(&self.scale);

        return translation * rotation * scale;
    }
}

pub trait Entity {
    fn set_transform(&mut self, transform : Transform);
    fn get_uuid(&mut self) -> Uuid;
    fn get_mesh(&mut self) -> &mut Mesh;
    fn get_transform(&mut self) -> &mut Transform;
}

pub struct Sprite {
    transform: Transform,
    mesh: Mesh,
    uuid: Uuid
}

impl Entity for Sprite {

    fn set_transform(&mut self, transform : Transform) {
        self.transform = transform;
    } 

    fn get_transform(&mut self) -> &mut Transform{
        return &mut self.transform;
    } 

    fn get_uuid(&mut self) -> Uuid {
        return self.uuid;
    }

    fn get_mesh(&mut self) -> &mut Mesh {
        return &mut self.mesh;
    }
}

pub fn create_sprite(position : Vector3<f32>, rotation : Vector3<f32>, scale : Vector3<f32>, image_texture : ImageTexture) -> Sprite{
    
    let material = Material {
        diffuse_texture : image_texture,
        diffuse_color: Vector4::new(1.0, 1.0, 1.0, 1.0)
    };

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        -0.5, 0.5, 0.0,
        0.5, 0.5, 0.0,
        0.5, -0.5, 0.0
    ];

    let indicies: Vec<u32> = vec![
        0, 1, 3,
        3, 1, 2
    ];

    let uv_cords: Vec<f32> = vec![
        0.0, 0.0,
        0.0, 1.0,
        1.0, 1.0,
        1.0, 0.0
    ];

    let mesh = Mesh{
        vertices: vertices,
        indicies: indicies,
        uv_cords: uv_cords,
        material: material,
        render_data: Default::default()
    };

    Sprite {
        transform: Transform{
            position: position,
            rotation: rotation,
            scale: scale
        },
        mesh: mesh,
        uuid: Uuid::new_v4()
    }
}