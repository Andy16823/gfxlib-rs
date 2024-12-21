use crate::utils;

pub mod prebuild_shader;

#[derive(Default, Clone)]
pub struct Shader {
    pub source : String,
}

#[derive(Clone)]
pub enum ShaderProgram{
    PreBuild { 
        fragment_shader : Shader,
        vertex_shader : Shader,
    },
    Builded {
        program_id : u32
    },
    Disposed {}
}

impl ShaderProgram {    
    pub fn load_vertex_shader(&mut self, file : String) {
        if let ShaderProgram::PreBuild { ref mut vertex_shader, .. } = self {
            vertex_shader.source = utils::load_file_as_string(file);
        } else {
            panic!("Cannot load vertex shader when the program is not in PreBuild state.");
        }
    }

    pub fn load_fragment_shader(&mut self, file : String) {
        if let ShaderProgram::PreBuild { ref mut fragment_shader, ..} = self {
            fragment_shader.source = utils::load_file_as_string(file);
        }
        else {
            panic!("Cannot load fragment shader when the program is not in PreBuild state.");
        }
    }
}