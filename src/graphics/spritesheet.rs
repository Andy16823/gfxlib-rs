use nalgebra::Vector2;

use crate::{math::Rect, utils};

use super::image_texture::ImageTexture;

#[derive(Clone, Copy)]
pub struct SpriteSheet {
    columns : u32,
    rows : u32,
    texture_id : u32,
    texture_dimensions : Vector2<u32>
}

impl SpriteSheet {
    pub fn new(columns : u32, rows : u32, texture : ImageTexture) -> Option<SpriteSheet> {
        if let ImageTexture::Loaded { id, dimensions } = texture {
            let spritesheet = SpriteSheet {
                columns: columns,
                rows: rows,
                texture_id: id,
                texture_dimensions: dimensions
            };
            return Some(spritesheet);
        }
        else{
            eprintln!("You try to create an spritesheet from an unitialized texture");
            return None;
        }
    }

    pub fn get_subimage(&mut self, column : u32, row : u32) -> Rect<f32> {
        return utils::get_subimage(self.texture_dimensions.x, self.texture_dimensions.y, self.columns, self.rows, column, row);
    }

    pub fn get_texture_id(&mut self) -> u32 {
        return self.texture_id;
    }

    pub fn get_texture_dimensions(&mut self) -> Vector2<u32> {
        return self.texture_dimensions;
    }
}

