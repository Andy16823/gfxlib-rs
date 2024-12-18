use std::{collections::HashMap, ffi::CString};
use gl::types::*;
use glfw::PWindow;
use nalgebra::{Matrix4, Vector2, Vector4};
use crate::{core::transform::{ITransform, Transform3D}, shader::ShaderProgram, utils};
use super::{camera::{Camera, ICamera}, image_texture::ImageTexture, mesh::Mesh, render_target::RenderTarget, shapes::{FramebufferShape, Shape, TextureShape}, viewport::Viewport, InstanceBatch, RenderData};

#[derive(Default)]
pub struct RenderDevice {
    viewport : Viewport,
    view_matrix : Matrix4<f32>,
    projection_matrix : Matrix4<f32>,
    render_shapes : HashMap<String, RenderData>,
    shader_program: u32
}

impl RenderDevice {

    pub fn init(&mut self, window: &mut PWindow) {

        //initial opengl with the glfw window
        gl::load_with(|s| window.get_proc_address(s) as *const _);
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        //initial the render_shapes
        let framebuffer_shape = self.init_shape(FramebufferShape);
        self.render_shapes.insert(String::from("framebuffer_shape"), framebuffer_shape);
        let texture_shape = self.init_shape(TextureShape);
        self.render_shapes.insert(String::from("texture_shape"), texture_shape);
        let texture_batch_shape = self.init_shape(TextureShape);
        self.render_shapes.insert(String::from("texture_batch_shape"), texture_batch_shape);
    }

    pub fn clear_color(&mut self, color: Vector4<f32>) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, color.w);
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn disable_depth_test(&mut self) {
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
    }

    pub fn enable_depth_test(&mut self) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
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

    pub fn create_render_target(&mut self, width : u32, height : u32) -> RenderTarget {
        unsafe {
            let mut framebuffer_id : GLuint = 0;
            gl::GenFramebuffers(1, &mut framebuffer_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer_id);

            let mut texture_id : GLuint = 0;
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint, width as GLsizei, height as GLsizei, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture_id, 0);
            
            let mut renderbuffer_id : GLuint = 0;
            gl::GenRenderbuffers(1, &mut renderbuffer_id);
            gl::BindRenderbuffer(gl::RENDERBUFFER, renderbuffer_id);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, width as GLsizei, height as GLsizei);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, renderbuffer_id);
            
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE {
                println!("RenderTarget created");
            }
            else {
                println!("Error while creating RenderTarget");
            }
            
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            return RenderTarget {
                size : Vector2::new(width, height),
                renderbuffer_id: renderbuffer_id,
                texture_id: texture_id,
                framebuffer_id: framebuffer_id
            }
        }
    }

    pub fn resize_render_target(&mut self, render_target : &mut RenderTarget, width : u32, height : u32) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, render_target.texture_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint, width as GLsizei, height as GLsizei, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null());
            gl::BindTexture(gl::TEXTURE_2D, 0);

            gl::BindRenderbuffer(gl::RENDERBUFFER, render_target.renderbuffer_id);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, width as GLsizei, height as GLsizei);
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

            render_target.size = Vector2::new(width, height);
        }
    }

    pub fn load_texture(&mut self, image_texture: &mut ImageTexture) {
        match image_texture {
            ImageTexture::PreLoad { path: _, dimensions, data } => {
                unsafe {
                    let mut texture_id: GLuint = 0;
                    gl::GenTextures(1, &mut texture_id);
                    gl::BindTexture(gl::TEXTURE_2D, texture_id);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        gl::RGBA  as GLint,
                        dimensions.x as GLsizei,
                        dimensions.y as GLsizei,
                        0,
                        gl::RGBA,
                        gl::UNSIGNED_BYTE,
                        data.as_ptr() as *const GLvoid,
                    );
                    gl::BindTexture(gl::TEXTURE_2D, 0);
                    *image_texture = ImageTexture::Loaded { id: texture_id, dimensions: Vector2::new(dimensions.x, dimensions.y) };
                }
            }
            _ => {
                eprintln!("You try to load an corruped or unloaded texture!");
            }
        }
    }

    pub fn load_instance_batch(&mut self, instance_batch : &mut InstanceBatch) {
        match instance_batch {
            InstanceBatch::PreLoad { instances } => {
                let buffers = InstanceBatch::create_buffers(instances);
                let mut mbo : GLuint = 0;
                let mut cbo : GLuint = 0;
                unsafe {
                    gl::GenBuffers(1, &mut mbo);
                    gl::BindBuffer(gl::ARRAY_BUFFER, mbo);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (buffers.0.len() * std::mem::size_of::<f32>()) as isize,
                        buffers.0.as_ptr() as *const _,
                        gl::DYNAMIC_DRAW
                    );

                    gl::GenBuffers(1, &mut cbo);
                    gl::BindBuffer(gl::ARRAY_BUFFER, cbo);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (buffers.1.len() * std::mem::size_of::<f32>()) as isize,
                        buffers.1.as_ptr() as *const _,
                        gl::DYNAMIC_DRAW
                    );

                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                }
                *instance_batch = InstanceBatch::Loaded { instances: instances.clone(), mbo: mbo, cbo : cbo }
            }
            _ => {
                eprintln!("You try to load an allready loaded instance batch.");
            }
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

            let mut success = 1;
            let mut log = vec![0; 512];

            gl::GetProgramiv(shader_program.program_id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                gl::GetProgramInfoLog(shader_program.program_id, 512, std::ptr::null_mut(), log.as_mut_ptr() as *mut i8);
                println!("Shader Program Linking Failed: {}", String::from_utf8_lossy(&log)); 
            }
            else 
            {
                println!("Shader Program {} created with error {}", program_id, gl::GetError());
            }
        }
    }

    pub fn bind_shader_program(&mut self, shader_program : &mut ShaderProgram) {
        unsafe {
            gl::UseProgram(shader_program.program_id);
            self.shader_program = shader_program.program_id;
        }
    }

    pub fn unbind_shader_program(&mut self) {
        unsafe {
            gl::UseProgram(0);
            self.shader_program = 0;
        }
    }

    pub fn get_uniform_location(&mut self, program_id : u32, name : &str) -> i32 {
        let name = CString::new(name).expect("CString::new failed");
        unsafe {
            let location= gl::GetUniformLocation(program_id, name.as_ptr());
            return location;
        }
    }

    pub fn bind_render_target(&mut self, render_target : RenderTarget) 
    {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, render_target.framebuffer_id);
        }
    }

    pub fn unbind_render_target(&mut self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn init_shape<T: Shape>(&mut self, shape : T) -> RenderData{
        unsafe {
            //create and bin vertex array object
            let mut vao : GLuint = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            //create and bind the vertex buffer for the vao
            let vertex_buffer = shape.get_vertex_buffer();
            let mut vbo : GLuint = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertex_buffer.len() * std::mem::size_of::<f32>()) as isize,
                vertex_buffer.as_ptr() as *const _,
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

            //create and bind the uv buffer for the vao
            let uv_buffer = shape.get_uv_buffer();
            let mut tbo : GLuint = 0;
            gl::GenBuffers(1, &mut tbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, tbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (uv_buffer.len() * std::mem::size_of::<f32>()) as isize,
                uv_buffer.as_ptr() as *const _,
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

            //create and bind the index buffer for the vao
            let index_buffer = shape.get_index_buffer();
            let mut ibo : GLuint = 0;
            gl::GenBuffers(1, &mut ibo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (index_buffer.len() * std::mem::size_of::<u32>()) as isize,
                index_buffer.as_ptr() as *const _,
                gl::DYNAMIC_DRAW
            );

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            //return the render data for the shape
            return RenderData{
                vao: vao,
                vbo: vbo,
                ibo: ibo,
                tbo: tbo,
                index_count: index_buffer.len() as u32
            };
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
            mesh.render_data.index_count = mesh.indicies.len() as u32;
        }
    }

    pub fn draw_mesh(&mut self, transform : &mut Transform3D, mesh : &mut Mesh) {
        match mesh.material.diffuse_texture {
            ImageTexture::Loaded { id, dimensions: _ } => {
                unsafe {
                    gl::Enable(gl::TEXTURE_2D);
                    gl::UniformMatrix4fv(self.get_uniform_location(self.shader_program, "p_mat"), 1, gl::FALSE, self.projection_matrix.as_ptr());
                    gl::UniformMatrix4fv(self.get_uniform_location(self.shader_program, "v_mat"), 1, gl::FALSE, self.view_matrix.as_ptr());
                    gl::UniformMatrix4fv(self.get_uniform_location(self.shader_program, "m_mat"), 1, gl::FALSE, transform.get_model_matrix().as_ptr());
                    gl::Uniform4f(self.get_uniform_location(self.shader_program, "vertexColor"), mesh.material.diffuse_color.x, mesh.material.diffuse_color.y, mesh.material.diffuse_color.z, mesh.material.diffuse_color.w);
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, id);
                    gl::Uniform1i(self.get_uniform_location(self.shader_program, "textureSampler"), 0);
        
                    gl::BindVertexArray(mesh.render_data.vao);
                    gl::DrawElements(gl::TRIANGLES, mesh.indicies.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
                    gl::BindVertexArray(0);
                    gl::Disable(gl::TEXTURE_2D);
                }
            }
            _ => {
                eprintln!("Texture not loaded");
            }
        }
    }

    pub fn draw_texture2drt<T : ITransform>(&mut self, transform : T, render_target: &mut RenderTarget, color: Vector4<f32>) {
        let uv_buffer = utils::generate_uv_coords(render_target.size.x, render_target.size.y, Vector2::new(0.0, 0.0), Vector2::new(render_target.size.x as f32, render_target.size.y as f32));
        self.draw_texture2di_internal(transform, render_target.texture_id, color, uv_buffer);
    }

    pub fn draw_texture2d<T: ITransform>(&mut self, transform : T, image_texture: &mut ImageTexture, color: Vector4<f32>) {
        match image_texture {
            ImageTexture::Loaded { id, dimensions } => {
                let uv_buffer = utils::generate_uv_coords(dimensions.x, dimensions.y, Vector2::new(0.0, 0.0), Vector2::new(dimensions.x as f32, dimensions.y as f32));
                self.draw_texture2di_internal(transform, *id, color, uv_buffer);
            }
            _ => {
                eprintln!("Texture not loaded");
            }
        }
    }

    pub fn draw_sub_texture2d<T: ITransform>(&mut self, transform : T, point : Vector2<f32>, size : Vector2<f32>, image_texture: &mut ImageTexture, color: Vector4<f32>) {
        match image_texture {
            ImageTexture::Loaded { id, dimensions } => {
                let uv_buffer = utils::generate_uv_coords(dimensions.x, dimensions.y, point, size); 
                self.draw_texture2di_internal(transform, *id, color, uv_buffer);
            }
            _ => {
                eprintln!("Texture not loaded"); 
            }
        }
    }

    pub fn draw_texture2di<T: ITransform>(&mut self, transform : T, texture_size : Vector2<f32>, texture_id: u32, color: Vector4<f32>) {
        let uv_buffer = utils::generate_uv_coords(texture_size.x as u32, texture_size.y as u32, Vector2::new(0.0, 0.0), Vector2::new(texture_size.x, texture_size.y)); 
        self.draw_texture2di_internal(transform, texture_id, color, uv_buffer);
    }

    fn draw_texture2di_internal<T: ITransform>(&mut self, transform : T, texture_id: u32, color: Vector4<f32>, uv_buffer : Vec<f32>) {
        let shape = self.render_shapes.get("texture_shape").copied();
        match shape {
            Some(shape) => {
                self.disable_depth_test();
                unsafe {
                    //change buffer data
                    gl::BindBuffer(gl::ARRAY_BUFFER, shape.tbo);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (uv_buffer.len() * std::mem::size_of::<f32>()) as isize,
                        uv_buffer.as_ptr() as *const _,
                        gl::DYNAMIC_DRAW
                    );
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                    
                    //draw
                    gl::Enable(gl::TEXTURE_2D);
                    gl::UniformMatrix4fv(self.get_uniform_location(self.shader_program, "p_mat"), 1, gl::FALSE, self.projection_matrix.as_ptr());
                    gl::UniformMatrix4fv(self.get_uniform_location(self.shader_program, "v_mat"), 1, gl::FALSE, self.view_matrix.as_ptr());
                    gl::UniformMatrix4fv(self.get_uniform_location(self.shader_program, "m_mat"), 1, gl::FALSE, transform.get_model_matrix().as_ptr());
                    gl::Uniform4f(self.get_uniform_location(self.shader_program, "vertexColor"), color.x, color.y, color.z, color.w);
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, texture_id);
                    gl::Uniform1i(self.get_uniform_location(self.shader_program, "textureSampler"), 0);

                    gl::BindVertexArray(shape.vao);
                    gl::DrawElements(gl::TRIANGLES, shape.index_count as i32, gl::UNSIGNED_INT, std::ptr::null());
                    gl::BindVertexArray(0);
                    gl::Disable(gl::TEXTURE_2D);
                }
                self.enable_depth_test();
            }
            None => {
                eprintln!("Texture shape not found!");
            }
        }
    }


    pub fn draw_texture2d_batch(&mut self, image_texture: &mut ImageTexture, instance_batch : &mut InstanceBatch) {
        match image_texture {
            ImageTexture::Loaded { id, dimensions: _ } => {
                match instance_batch {
                    InstanceBatch::Loaded { instances, mbo, cbo } => {
                        let shape = self.render_shapes.get("texture_batch_shape").copied();
                        match shape {
                            Some(shape) => {
                                unsafe {
                                    let vec4_size = std::mem::size_of::<f32>() * 4;
                                    let matrix_stride = vec4_size * 4;

                                    //Prepare shader
                                    gl::UniformMatrix4fv(self.get_uniform_location(self.shader_program, "p_mat"), 1, gl::FALSE, self.projection_matrix.as_ptr());
                                    gl::UniformMatrix4fv(self.get_uniform_location(self.shader_program, "v_mat"), 1, gl::FALSE, self.view_matrix.as_ptr());
                                    gl::ActiveTexture(gl::TEXTURE0);
                                    gl::BindTexture(gl::TEXTURE_2D, *id);
                                    gl::Uniform1i(self.get_uniform_location(self.shader_program, "textureSampler"), 0);

                                    //Bind the vao
                                    gl::BindVertexArray(shape.vao);

                                    //Bind the color buffer and assign it to the location per instance
                                    gl::BindBuffer(gl::ARRAY_BUFFER, *cbo);
                                    gl::EnableVertexAttribArray(2);
                                    gl::VertexAttribPointer(2, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
                                    gl::VertexAttribDivisor(2, 1);

                                    //Bind the transforms buffer and assign it to the locations per instance
                                    gl::BindBuffer(gl::ARRAY_BUFFER, *mbo);
                                    gl::EnableVertexAttribArray(3);
                                    gl::VertexAttribPointer(3, 4, gl::FLOAT, gl::FALSE, matrix_stride as i32, (0 * vec4_size) as *const _);
                                    gl::VertexAttribDivisor(3, 1);
                                    gl::EnableVertexAttribArray(4);
                                    gl::VertexAttribPointer(4, 4, gl::FLOAT, gl::FALSE, matrix_stride as i32, (1 * vec4_size) as *const _);
                                    gl::VertexAttribDivisor(4, 1);
                                    gl::EnableVertexAttribArray(5);
                                    gl::VertexAttribPointer(5, 4, gl::FLOAT, gl::FALSE, matrix_stride as i32, (2 * vec4_size) as *const _);
                                    gl::VertexAttribDivisor(5, 1);
                                    gl::EnableVertexAttribArray(6);
                                    gl::VertexAttribPointer(6, 4, gl::FLOAT, gl::FALSE, matrix_stride as i32, (3 * vec4_size) as *const _);
                                    gl::VertexAttribDivisor(6, 1);

                                    //Draw the elements instanced
                                    gl::DrawElementsInstanced(gl::TRIANGLES, shape.index_count as i32, gl::UNSIGNED_INT, std::ptr::null(), instances.len() as i32);
                                    gl::BindVertexArray(0);
                                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                                }
                            }
                            None => {
                                eprintln!("Shape not found");
                            }
                        }
                    }
                    _ => {
                        eprintln!("You try to draw an instance batch wich isn't loaded");
                    }
                }
            }
            _ => {
                eprintln!("You try to draw an instance batch wich isn't loaded");
            }
        }
    }

    pub fn draw_render_target(&mut self, render_target : RenderTarget) {
        let shape = self.render_shapes.get("framebuffer_shape").copied();
        match shape {
           Some(shape) => {
                self.disable_depth_test();
                unsafe {
                    gl::Enable(gl::TEXTURE_2D);
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, render_target.texture_id);
                    gl::Uniform1i(self.get_uniform_location(self.shader_program, "textureSampler"), 0);
                    gl::BindVertexArray(shape.vao);
                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
                    gl::BindVertexArray(0);
                    gl::Disable(gl::TEXTURE_2D);
                }
                self.enable_depth_test();
           }
           None => {
                eprintln!("Framebuffer shape not found!");
           } 
        }
    }

    pub fn dispose(&mut self) {
        let render_shapes = std::mem::take(&mut self.render_shapes);
        for (_key, mut value) in render_shapes {
            self.dispose_render_data(&mut value);
        }
    }

    pub fn dispose_render_data(&mut self, render_data : &mut RenderData) {
        unsafe {
            if render_data.vbo != 0 {
                gl::DeleteBuffers(1, &render_data.vbo);
                render_data.vbo = 0;
            }
            
            if render_data.tbo != 0 {
                gl::DeleteBuffers(1, &render_data.tbo);
                render_data.tbo = 0;
            }
            
            if render_data.ibo != 0 {
                gl::DeleteBuffers(1, &render_data.ibo);
                render_data.ibo = 0;
            }
            
            if render_data.vao != 0 {
                gl::DeleteVertexArrays(1, &render_data.vao);
                render_data.vao = 0;
            }
        }
    }

    pub fn dispose_image_texture(&mut self, image_texture: &mut ImageTexture) {
        match image_texture {
            ImageTexture::Loaded { id, dimensions: _ } => {
                unsafe {
                    gl::DeleteTextures(1, &*id);
                    *image_texture = ImageTexture::Disposed;
                }
            }
            _ => {
                println!("Texture was not loaded!")
            }
        }
    }

    pub fn dispose_render_target(&mut self, render_target : &mut RenderTarget) {
        unsafe {

            if render_target.texture_id != 0 {
                gl::DeleteTextures(1, &render_target.texture_id);
                render_target.texture_id = 0;
            }

            if render_target.renderbuffer_id != 0 {
                gl::DeleteRenderbuffers(1, &render_target.renderbuffer_id);
                render_target.renderbuffer_id = 0;
            }

            if render_target.framebuffer_id != 0 {
                gl::DeleteFramebuffers(1, &render_target.framebuffer_id);
                render_target.framebuffer_id = 0;
            }
        }
    }

    pub fn dispose_mesh(&mut self, mesh : &mut Mesh, dispose_material : bool) {
        self.dispose_render_data(&mut mesh.render_data);
        if dispose_material == true {
            self.dispose_image_texture(&mut mesh.material.diffuse_texture);
        }
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