use std::collections::HashMap;
use nalgebra::Vector2;

use super::TextAlignment;

pub struct Character {
    pub texture_id: u32,
    pub size: Vector2<i32>,
    pub bearing: Vector2<i32>,
    pub advance: i32,
}

#[derive(Default)]
pub struct Font {
    pub characters: HashMap<char, Character>,
    pub vao : u32,
    pub vbo : u32
}

impl Font {
    pub fn new() -> Font {
        Font {
            characters: HashMap::new(),
            vao: 0,
            vbo: 0
        }
    }

    pub fn get_string_bounds(&mut self, text : &str, scale : f32) -> Vector2<f32> {
        let mut width = 0.0;
        let mut max_height = 0.0;

        for c in text.chars() {
            if let Some(character) = self.characters.get(&c) {

                width += ((character.advance as i32 >> 6) as f32) * scale;

                let height = character.size.y as f32 * scale;
                if height > max_height {
                    max_height = height;
                }
            }
        }
        Vector2::new(width, max_height)
    }

    pub fn get_offset(&mut self, text : &str, scale : f32, alignment : TextAlignment) -> Vector2<f32>{
        let bounds = self.get_string_bounds(text, scale);
        let bounds_half_width = bounds.x / 2.0;
        let bounds_half_height = bounds.y / 2.0;
        match alignment {
            TextAlignment::BottomLeft => {
                return Vector2::new(0.0, 0.0);
            }
            TextAlignment::BottomCenter => {
                return Vector2::new(-bounds_half_width, 0.0);
            }
            TextAlignment::BottomRight => {
                return Vector2::new(-bounds.x, 0.0);
            }
            TextAlignment::MiddleLeft => {
                return Vector2::new(0.0, -bounds_half_height);
            }
            TextAlignment::MiddleCenter => {
                return Vector2::new(-bounds_half_width, -bounds_half_height);
            }
            TextAlignment::MiddleRight  => {
                return Vector2::new(-bounds.x, -bounds_half_height);
            }
            TextAlignment::TopLeft => {
                return Vector2::new(0.0, -bounds.y);
            }
            TextAlignment::TopCenter => {
                return Vector2::new(-bounds_half_width, -bounds.y);
            }
            TextAlignment::TopRight => {
                return Vector2::new(-bounds.x, -bounds.y);
            }
        }
    }
}