use std::ffi::CString;

use gl::types::*;
use glfw::PWindow;
use nalgebra::{Matrix4, Vector4};

use crate::{core::{Entity, Sprite, Transform}, shader::ShaderProgram};

use super::{camera::{Camera, ICamera}, image_texture::ImageTexture, mesh::Mesh, viewport::Viewport};

#[derive(Default)]
pub struct RenderDevice {
    viewport : Viewport,
    view_matrix : Matrix4<f32>,
    projection_matrix : Matrix4<f32>
}

impl RenderDevice {

    pub fn init(&mut self, window: &mut PWindow) {
        gl::load_with(|s| window.get_proc_address(s) as *const _);
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
    }

    pub fn clear(&mut self, color: Vector4<f32>) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, color.w);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn set_viewport(&mut self, viewport : Viewport) {
        unsafe {
            gl::Viewport(0, 0, viewport.size.x as i32, viewport.size.y as i32);
        }
        self.viewport = viewport;
    }

    pub fn set_camera(&mut self, camera : &mut Camera) {
        self.view_matrix = camera.get_view_matrix();
        self.projection_matrix = camera.get_projection_matrix(self.viewport, 1.0);
    }

    pub fn load_texture(&mut self, image_texture: &mut ImageTexture) {
        unsafe {
            let mut texture_id: GLuint = 0;
            gl::GenTextures(1, &mut texture_id);
            image_texture.texture_id = texture_id;

            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA  as GLint,
                image_texture.width as GLsizei,
                image_texture.height as GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image_texture.data.as_ptr() as *const GLvoid,
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn compile_shader(&mut self, source : &str, shader_type : GLuint) -> u32{
        unsafe {
            let shader_id = gl::CreateShader(shader_type);
            let shader_source = CString::new(source).expect("CString::new failed");
            let shader_length = shader_source.to_bytes().len() as GLint;
            gl::ShaderSource(shader_id, 1, &shader_source.as_ptr(), &shader_length);
            gl::CompileShader(shader_id);
            println!("Compiled shader {} with error {}", shader_id, gl::GetError());
            return shader_id;
        }
    }

    pub fn create_shader_program(&mut self, shader_program : &mut ShaderProgram) {
        unsafe {
            let vertex_shader = self.compile_shader(&shader_program.vertex_shader.source, gl::VERTEX_SHADER);
            let fragment_shader = self.compile_shader(&shader_program.fragment_shader.source, gl::FRAGMENT_SHADER);

            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader);
            gl::AttachShader(program_id, fragment_shader);
            gl::LinkProgram(program_id);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            shader_program.program_id = program_id;
            println!("Created shader program {} with error {}", program_id, gl::GetError());
        }
    }

    pub fn get_uniform_location(&mut self, shader_program : &mut ShaderProgram, name : &str) -> i32 {
        let name = CString::new(name).expect("CString::new failed");
        unsafe {
            let location= gl::GetUniformLocation(shader_program.program_id, name.as_ptr());
            return location;
        }
    }

    pub fn init_mesh(&mut self, mesh : &mut Mesh) {
        unsafe {
            let mut vao: GLuint = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            //Generate vertex buffer
            let mut vbo: GLuint = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.vertices.len() * std::mem::size_of::<f32>()) as isize,
                mesh.vertices.as_ptr() as *const _,
                gl::DYNAMIC_DRAW
            );
            gl::EnableVertexAttribArray(0); 
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                0,
                std::ptr::null()
            );
            mesh.render_data.vbo = vbo;

            //Generate index buffer
            let mut ibo: GLuint = 0;
            gl::GenBuffers(1, &mut ibo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (mesh.indicies.len() * std::mem::size_of::<u32>()) as isize,
                mesh.indicies.as_ptr() as *const _,
                gl::DYNAMIC_DRAW
            );
            mesh.render_data.ibo = ibo;

            //Generate texture cords
            let mut tbo: GLuint = 0;
            gl::GenBuffers(1, &mut tbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, tbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.uv_cords.len() * std::mem::size_of::<f32>()) as isize,
                mesh.uv_cords.as_ptr() as *const _,
                gl::DYNAMIC_DRAW
            );
            gl::EnableVertexAttribArray(1); 
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                0,
                std::ptr::null()
            );
            mesh.render_data.tbo = tbo;

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            mesh.render_data.vao = vao;
        }
    }

    pub fn draw_sprite(&mut self, sprite : &mut Sprite, shader_program : &mut ShaderProgram) {
        let mut transform = sprite.get_transform().clone();
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
        
        self.draw_mesh(&mut transform, sprite.get_mesh(), shader_program);
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
    }

    pub fn draw_mesh(&mut self, transform : &mut Transform, mesh : &mut Mesh, shader_program : &mut ShaderProgram) {
        unsafe {
            gl::Enable(gl::TEXTURE_2D);

            let mut success = 1;
            let mut log = vec![0; 512];  // Puffer für Log-Fehlermeldungen

            // Überprüfe, ob das Programm korrekt verlinkt wurde
            gl::GetProgramiv(shader_program.program_id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                gl::GetProgramInfoLog(shader_program.program_id, 512, std::ptr::null_mut(), log.as_mut_ptr() as *mut i8);
                println!("Shader Program Linking Failed: {}", String::from_utf8_lossy(&log));  // Ausgeben des Logs
            }

            gl::UseProgram(shader_program.program_id);
            gl::UniformMatrix4fv(self.get_uniform_location(shader_program, "p_mat"), 1, gl::FALSE, self.projection_matrix.as_ptr());
            gl::UniformMatrix4fv(self.get_uniform_location(shader_program, "v_mat"), 1, gl::FALSE, self.view_matrix.as_ptr());
            gl::UniformMatrix4fv(self.get_uniform_location(shader_program, "m_mat"), 1, gl::FALSE, transform.get_model_matrix().as_ptr());
            gl::Uniform4f(self.get_uniform_location(shader_program, "vertexColor"), mesh.material.diffuse_color.x, mesh.material.diffuse_color.y, mesh.material.diffuse_color.z, mesh.material.diffuse_color.w);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, mesh.material.diffuse_texture.texture_id);
            gl::Uniform1i(self.get_uniform_location(shader_program, "textureSampler"), 0);

            gl::BindVertexArray(mesh.render_data.vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.render_data.ibo);
            gl::DrawElements(gl::TRIANGLES, mesh.indicies.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);

            println!("Error while render {}", gl::GetError());
        }
    }

    pub fn dispose_mesh(&mut self) {

    }

    pub fn dispose_shader_program(&mut self, shader_program : &mut ShaderProgram) {
        unsafe {
            gl::DeleteProgram(shader_program.program_id);
            println!("Disposed shader programm {}", shader_program.program_id);
            shader_program.program_id = 0;
        }
    }

    pub fn get_error(&self) -> u32 {
        unsafe { gl::GetError() }
    }
}