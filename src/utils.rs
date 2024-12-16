use std::ffi::CString;

use nalgebra::Vector2;

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

pub fn generate_uv_coords(image_width : u32, image_height : u32, point : Vector2<u32>, size : Vector2<u32>) -> Vec<f32> {
    let span_x = 1.0 / image_width as f32;
    let span_y = 1.0 / image_height as f32;

    let bottom_left_x = span_x * point.x as f32;
    let bottom_left_y = span_y * point.y as f32;
    let top_left_x = bottom_left_x;
    let top_left_y = bottom_left_y + (span_y * size.y as f32);
    let top_right_x = top_left_x + (span_x * size.x as f32);
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