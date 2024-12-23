use nalgebra::Vector4;

use super::image_texture::ImageTexture;

#[derive(Default)]
pub struct Material {
    pub base_color_texture : Option<ImageTexture>,
    pub normal_map : Option<ImageTexture>,
    pub metallic_roughness_texture : Option<ImageTexture>,
    pub base_color_friction : Vector4<f32>,
    pub metallic_factor : f32,    
    pub roughness_factor : f32
}

impl Material {
    pub fn new() -> Material {
        return Material{
            base_color_texture: None,
            normal_map: None,
            metallic_roughness_texture: None,
            base_color_friction: Vector4::new(1.0, 1.0, 1.0, 1.0),
            metallic_factor: 1.0,
            roughness_factor: 0.0
        };
    }
}