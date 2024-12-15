use nalgebra::Vector4;

use super::image_texture::ImageTexture;

pub struct Material {
    pub diffuse_texture : ImageTexture,
    pub diffuse_color : Vector4<f32>
}

pub fn create_material(diffuse_texture: ImageTexture) -> Material {
    Material {
        diffuse_texture : diffuse_texture,
        diffuse_color: Vector4::new(1.0, 1.0, 1.0, 1.0)
    }
}