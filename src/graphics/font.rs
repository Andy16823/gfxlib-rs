use std::collections::HashMap;
use nalgebra::Vector2;

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
}
