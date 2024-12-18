use std::ffi::CString;

use nalgebra::{Vector2, Vector4};

use crate::image_texture::ImageTexture;

/// Loads the content of a file into a string.
/// 
/// # Arguments
/// - `file`: The path to the file as a `String`.
/// 
/// # Returns
/// - A `String` containing the file's contents, or an empty string if the file cannot be read.
/// 
/// # Errors
/// - Prints an error message to `stderr` if the file cannot be opened or read.
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

/// Converts a Rust `String` to a C-style string (`CString`).
/// 
/// # Arguments
/// - `value`: The input `String` to be converted.
/// 
/// # Returns
/// - A `CString` containing the same data as the input string.
/// 
/// # Panics
/// - Panics if the input string contains a null byte, as it cannot be represented in a `CString`.
pub fn to_cstr(value : String) -> CString {
   return CString::new(value).expect("CString::new failed");
}

/// Generates UV coordinates for a specific point and size on an image.
/// 
/// # Arguments
/// - `image_width`: The width of the image in pixels.
/// - `image_height`: The height of the image in pixels.
/// - `point`: The starting point (bottom-left corner) as a `Vector2<f32>`.
/// - `size`: The size of the region as a `Vector2<f32>`.
/// 
/// # Returns
/// - A `Vec<f32>` containing the UV coordinates for the region in the following order:
///   `[bottom-left, top-left, top-right, bottom-right]`.
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

/// Retrieves a subimage's position and size from an image texture, based on grid division.
/// 
/// # Arguments
/// - `image_texture`: A mutable reference to an `ImageTexture` object.
/// - `columns`: The number of columns in the grid.
/// - `rows`: The number of rows in the grid.
/// - `column_index`: The column index of the desired subimage (starting at 0).
/// - `row_index`: The row index of the desired subimage (starting at 0).
/// 
/// # Returns
/// - A tuple containing:
///   - `Vector2<f32>`: The position of the subimage's bottom-left corner.
///   - `Vector2<f32>`: The dimensions (width, height) of the subimage.
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

/// Calculates the UV transformation for a clipped section of a texture.
/// 
/// # Arguments
/// - `texture_width`: The full width of the texture in pixels.
/// - `texture_height`: The full height of the texture in pixels.
/// - `clip_x`: The X-coordinate of the clipping region's top-left corner in pixels.
/// - `clip_y`: The Y-coordinate of the clipping region's top-left corner in pixels.
/// - `clip_width`: The width of the clipping region in pixels.
/// - `clip_height`: The height of the clipping region in pixels.
/// 
/// # Returns
/// - A `Vector4<f32>` containing the UV transformation parameters in the following order:
///   `[scale_x, scale_y, offset_x, offset_y]`.
pub fn calculate_uv_transform(texture_width : f32, texture_height : f32, clip_x : f32, clip_y : f32, clip_width : f32, clip_height : f32) -> Vector4<f32> {
    let scale_x = clip_width / texture_width;
    let scale_y = clip_height / texture_height;
    let offset_x = clip_x / texture_width;
    let offset_y = clip_y / texture_height;

    return Vector4::new(scale_x, scale_y, offset_x, offset_y);
}