use std::ffi::CString;

use nalgebra::Vector2;

use crate::image_texture::ImageTexture;

pub fn load_file_as_string(file : String) -> String{
    match std::fs::read_to_string(file) {
        Ok(contents) => {
            return  contents;
        }
        Err(e) => {
            eprint!("Error while loading the file {}", e);
            return String::default();
        }
    }
}

pub fn to_cstr(value : String) -> CString {
   return CString::new(value).expect("CString::new failed");
}

pub fn generate_uv_coords(image_width : u32, image_height : u32, point : Vector2<f32>, size : Vector2<f32>) -> Vec<f32> {
    let span_x = 1.0 / image_width as f32;
    let span_y = 1.0 / image_height as f32;

    let bottom_left_x = span_x * point.x;
    let bottom_left_y = span_y * point.y;
    let top_left_x = bottom_left_x;
    let top_left_y = bottom_left_y + (span_y * size.y);
    let top_right_x = top_left_x + (span_x * size.x);
    let top_right_y = top_left_y;
    let bottom_right_x = top_right_x;
    let bottom_right_y = bottom_left_y;

    let buffer: Vec<f32> = vec![
        bottom_left_x, bottom_left_y,
        top_left_x, top_left_y,
        top_right_x, top_right_y,
        bottom_right_x, bottom_right_y
    ];
    return buffer;
}

pub fn get_subimage(image_texture : &mut ImageTexture, columns : u32, rows : u32, column_index : u32, row_index : u32) -> (Vector2<f32>, Vector2<f32>) {
    match image_texture {
        ImageTexture::Loaded { id: _, dimensions } => {
            let cell_width = dimensions.x as f32 / columns as f32;
            let cell_height = dimensions.y as f32 / rows as f32;
            let pos_x = column_index as f32 * cell_width;
            let pos_y = row_index as f32 * cell_height;

            return (Vector2::new(pos_x, pos_y), Vector2::new(cell_width, cell_height));
        }
        ImageTexture::PreLoad { path: _, dimensions, data : _ } => {
            let cell_width = dimensions.x as f32 / columns as f32;
            let cell_height = dimensions.y as f32 / rows as f32;
            let pos_x = column_index as f32 * cell_width;
            let pos_y = row_index as f32 * cell_height;

            return (Vector2::new(pos_x, pos_y), Vector2::new(cell_width, cell_height));
        }
        _ => {
            return (Vector2::default(), Vector2::default())
        }
    }
}