use stb_image::image::{self, LoadResult};

pub enum LoadState {
    
}

#[derive(Clone, Default)]
pub struct ImageTexture {
    pub texture_id: u32,
    pub texture_path: String,
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl ImageTexture {

    pub fn load_from_file(file: &str) -> ImageTexture {
        unsafe  {
            stb_image::stb_image::stbi_set_flip_vertically_on_load_thread(1);
        }
        let load_result = image::load(file);

        match load_result {
            LoadResult::Error(e) => {
                println!("Error loading image: {}", e);
                ImageTexture::default()
            }
            LoadResult::ImageU8(e) => ImageTexture {
                texture_id: 0,
                texture_path: file.to_string(),
                width: e.width,
                height: e.height,
                data: e.data,
            },
            _ => {
                println!("Unsupported image format.");
                ImageTexture::default()
            }
        }
    }

    pub fn drop_data(&mut self) {
        self.data.clear();
    }

}
