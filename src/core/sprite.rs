use nalgebra::{Vector3, Vector4};
use uuid::Uuid;
use crate::{image_texture::ImageTexture, material::Material, mesh::Mesh};
use super::{entity::Entity, transform::Transform};


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