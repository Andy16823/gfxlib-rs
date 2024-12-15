use crate::utils;

#[derive(Default)]
pub struct Shader {
    pub source : String,
}

#[derive(Default)]
pub struct ShaderProgram{
    pub fragment_shader : Shader,
    pub vertex_shader : Shader,
    pub program_id : u32
}

impl ShaderProgram {    
    pub fn load_vertex_shader(&mut self, file : String) {
        self.vertex_shader.source = utils::load_file_as_string(file);
    }

    pub fn load_fragment_shader(&mut self, file : String) {
        self.fragment_shader.source = utils::load_file_as_string(file);
    }
}