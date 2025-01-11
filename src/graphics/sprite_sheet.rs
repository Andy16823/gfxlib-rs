use nalgebra::Vector2;

use crate::{math::Rect, utils};

use super::image_texture::ImageTexture;

/// A structure representing a sprite sheet.
/// A sprite sheet is a texture divided into multiple rows and columns of tiles or sprites.
#[derive(Clone, Copy)]
pub struct SpriteSheet {
    /// The number of columns in the sprite sheet.
    columns : u32,
    /// The number of rows in the sprite sheet.
    rows : u32,
    /// The ID of the texture associated with the sprite sheet.
    texture_id : u32,
    /// The dimensions of the texture (width and height in pixels).
    texture_dimensions : Vector2<u32>
}

impl SpriteSheet {

    /// Creates a new `SpriteSheet` from a given texture, number of columns, and rows.
    /// 
    /// # Arguments
    /// - `columns`: The number of columns in the sprite sheet.
    /// - `rows`: The number of rows in the sprite sheet.
    /// - `texture`: The texture used for the sprite sheet.
    /// 
    /// # Returns
    /// Returns `Some(SpriteSheet)` if the texture is loaded successfully, or `None` if the texture is uninitialized.
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

    /// Calculates the number of columns in the sprite sheet based on the texture's width and the tile width.
    /// 
    /// # Arguments
    /// - `texture`: The texture used for the sprite sheet.
    /// - `tile_width`: The width of each tile in pixels.
    /// 
    /// # Returns
    /// Returns the number of columns (`u32`) if successful, or `None` if the texture is invalid.
    pub fn calc_columns(texture : ImageTexture, tile_width : u32) -> Option<u32> {
        match texture {
            ImageTexture::PreLoad { path: _, dimensions, data: _, mode: _ } => {
                return Some(dimensions.x / tile_width);
            }
            ImageTexture::Loaded { id: _, dimensions } => {
                return Some(dimensions.x / tile_width);
            }
            _ => {
                return None;
            }
        }
    }

    /// Calculates the number of rows in the sprite sheet based on the texture's height and the tile height.
    /// 
    /// # Arguments
    /// - `texture`: The texture used for the sprite sheet.
    /// - `tile_height`: The height of each tile in pixels.
    /// 
    /// # Returns
    /// Returns the number of rows (`u32`) if successful, or `None` if the texture is invalid.
    pub fn calc_rows(texture : ImageTexture, tile_height : u32) -> Option<u32> {
        match texture {
            ImageTexture::PreLoad { path: _, dimensions, data: _, mode: _ } => {
                return Some(dimensions.y / tile_height);
            }
            ImageTexture::Loaded { id: _, dimensions } => {
                return Some(dimensions.y / tile_height);
            }
            _ => {
                return None;
            }
        }
    }

    /// Calculates the layout of tiles (number of columns and rows) in the sprite sheet.
    /// 
    /// # Arguments
    /// - `texture`: The texture used for the sprite sheet.
    /// - `tile_width`: The width of each tile in pixels.
    /// - `tile_height`: The height of each tile in pixels.
    /// 
    /// # Returns
    /// Returns a tuple `(columns, rows)` representing the number of tiles in each dimension.
    pub fn calc_tile_layout(texture : ImageTexture, tile_width : u32, tile_height : u32) -> (u32, u32) {
        let columns = SpriteSheet::calc_columns(texture.clone(), tile_width).unwrap_or(0);
        let rows = SpriteSheet::calc_rows(texture.clone(), tile_height).unwrap_or(0);
        return (columns, rows);
    }

    /// Retrieves a rectangular subimage (tile) from the sprite sheet based on the column and row index.
    /// 
    /// # Arguments
    /// - `column`: The column index of the desired tile (0-based).
    /// - `row`: The row index of the desired tile (0-based).
    /// 
    /// # Returns
    /// Returns a `Rect<f32>` representing the portion of the texture corresponding to the specified tile.
    pub fn get_subimage(&mut self, column : u32, row : u32) -> Rect<f32> {
        return utils::get_subimage(self.texture_dimensions.x, self.texture_dimensions.y, self.columns, self.rows, column, row);
    }

    /// Gets the texture ID associated with the sprite sheet.
    /// 
    /// # Returns
    /// Returns the texture ID (`u32`).
    pub fn get_texture_id(&mut self) -> u32 {
        return self.texture_id;
    }

    /// Gets the dimensions of the texture (width and height in pixels).
    /// 
    /// # Returns
    /// Returns a `Vector2<u32>` containing the texture dimensions.
    pub fn get_texture_dimensions(&mut self) -> Vector2<u32> {
        return self.texture_dimensions;
    }
    
    /// Returns the tile column and row from the given index
    /// 
    /// # Returns
    /// Returns a `(u32, u32)` where 0 is the column and 1 is the row
    pub fn tile_from_index(&mut self, index : u32, reversed_row : bool) -> (u32, u32) {
        let col = index % self.columns;
        let mut row = index / self.columns;
        if reversed_row {
            row = self.rows - 1 - row;
        }
        return (col, row);
    }

}

