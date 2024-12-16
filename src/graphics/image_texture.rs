use nalgebra::Vector2;
use stb_image::image::{self, LoadResult};

#[derive(Clone)]
pub enum ImageTexture {
    PreLoad {
        path: String,
        dimensions: Vector2<u32>,
        data: Vec<u8>,
    },
    Loaded {
        id: u32,
    },
    Corrupted,
    Disposed
}

impl ImageTexture {
    
    pub fn load_from_file(file: &str) -> ImageTexture {
        unsafe {
            stb_image::stb_image::stbi_set_flip_vertically_on_load_thread(1);
        }
        let load_result = image::load(file);

        match load_result {
            LoadResult::Error(e) => {
                println!("Error loading image: {}", e);
                return ImageTexture::Corrupted;
            }
            LoadResult::ImageU8(e) => ImageTexture::PreLoad {
                path: file.to_string(),
                dimensions: Vector2::new(e.width as u32, e.height as u32),
                data: e.data,
            },
            _ => {
                println!("Unsupported image format.");
                return  ImageTexture::Corrupted;
            }
        }
    }
    
}
