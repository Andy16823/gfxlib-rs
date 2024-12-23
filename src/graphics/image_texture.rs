use gl::types::GLint;
use nalgebra::Vector2;
use stb_image::image::{self, LoadResult};

#[derive(Clone, Copy)]
pub enum ColorMode {
    RGBA = gl::RGBA as isize,
    RGB = gl::RGB as isize
}

impl Default for ColorMode {
    fn default() -> Self {
        ColorMode::RGBA
    }
}

impl ColorMode {
    pub fn as_i32(self) -> i32 {
        self as i32 
    }

    pub fn as_glint(self) -> GLint {
        self as GLint
    }

    pub fn as_u32(self) -> u32 {
        self as u32
    }
}

#[derive(Clone)]
pub enum ImageTexture {
    PreLoad {
        path: String,
        dimensions: Vector2<u32>,
        data: Vec<u8>,
        mode: ColorMode
    },
    Loaded {
        id: u32,
        dimensions: Vector2<u32>
    },
    Corrupted,
    Disposed
}

impl ImageTexture {
    
    pub fn load_from_file(file: &str, flip_vertically : bool) -> ImageTexture {
        unsafe {
            stb_image::stb_image::stbi_set_flip_vertically_on_load_thread(flip_vertically as i32);
        }
        let load_result = image::load(file);

        match load_result {
            LoadResult::Error(e) => {
                println!("Error loading image: {}", e);
                return ImageTexture::Corrupted;
            }
            LoadResult::ImageU8(e) => {
                
                let mode = match e.depth {
                    3 => ColorMode::RGB,
                    4 => ColorMode::RGBA,
                    _ => {
                        println!("Unsupported number of channels: {}", e.depth);
                        ColorMode::default()
                    }
                };

                return ImageTexture::PreLoad {
                    path: file.to_string(),
                    dimensions: Vector2::new(e.width as u32, e.height as u32),
                    data: e.data,
                    mode: mode
                }
            }
            _ => {
                println!("Unsupported image format.");
                return  ImageTexture::Corrupted;
            }
        }
    }

    pub fn load_from_data(data : Vec<u8>, dimensions : Vector2<u32>) -> ImageTexture {
        return ImageTexture::PreLoad { 
            path: String::new(), 
            dimensions: dimensions, 
            data: data,
            mode: ColorMode::default()
        };
    }
    
}
