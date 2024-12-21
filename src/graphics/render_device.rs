use std::{collections::HashMap, ffi::CString};
use freetype::{face::LoadFlag, Library};
use gl::types::*;
use glfw::PWindow;
use nalgebra::{Matrix4, Vector2, Vector4};
use crate::{core::transform::{ITransform, Transform3D}, shader::ShaderProgram, utils};
use super::{camera::ICamera, font::{Character, Font}, image_texture::ImageTexture, mesh::Mesh, render_target::RenderTarget, shapes::{FramebufferShape, RectShape, Shape, TextureShape}, viewport::Viewport, RenderData, TextAlignment, Texture2DBatch, Texture2DInstance};

#[derive(Default)]
pub struct RenderDevice {
    viewport : Viewport,
    view_matrix : Matrix4<f32>,
    projection_matrix : Matrix4<f32>,
    render_shapes : HashMap<String, RenderData>,
    shader_program: u32
}

impl RenderDevice {

    /// Initializes the render device with the provided window.
    /// Sets up OpenGL context, enables depth testing and blending,
    /// and initializes default shapes used for rendering.
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
        let rect_shape = self.init_shape(RectShape);
        self.render_shapes.insert(String::from("rect_shape"), rect_shape);
    }

    /// Sets the clear color for the OpenGL context.
    pub fn clear_color(&mut self, color: Vector4<f32>) {
        unsafe {
            gl::ClearColor(color.x, color.y, color.z, color.w);
        }
    }

    /// Clears the screen using the current clear color and clears the depth buffer.
    pub fn clear(&mut self) {
        unsafe {
            
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    /// Disables depth testing in OpenGL.
    pub fn disable_depth_test(&mut self) {
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
    }

    /// Enables depth testing in OpenGL.
    pub fn enable_depth_test(&mut self) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
    }

    /// Sets the viewport size for rendering.
    pub fn set_viewport(&mut self, viewport : Viewport) {
        unsafe {
            gl::Viewport(0, 0, viewport.size.x as i32, viewport.size.y as i32);
        }
        self.viewport = viewport;
    }

    /// Configures the camera's view and projection matrices.
    pub fn set_camera<T: ICamera>(&mut self, camera : &mut T) {
        self.view_matrix = camera.get_view_matrix();
        self.projection_matrix = camera.get_projection_matrix(self.viewport, 1.0);
    }

    /// Configures the view matrix.
    pub fn set_view_matrix(&mut self, matrix : Matrix4<f32>) {
        self.view_matrix = matrix;
    }

    /// Configures the projection matrix.
    pub fn set_projection_matrix(&mut self, matrix : Matrix4<f32>) {
        self.projection_matrix = matrix;
    }

    /// Creates a render target (framebuffer) with the specified width and height.
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

    /// Resizes an existing render target to the specified dimensions.
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

    /// Loads a texture from an ImageTexture object into OpenGL.
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

    /// Loads a texture batch into GPU memory, creating necessary buffers and updating the batch state to `Loaded`.
    pub fn load_texture2d_batch(&mut self, instance_batch : &mut Texture2DBatch) {
        match instance_batch {
            Texture2DBatch::PreLoad { instances } => {
                let buffers = Texture2DBatch::create_buffers(instances);
                let mut mbo : GLuint = 0;
                let mut cbo : GLuint = 0;
                let mut uvto: GLuint = 0;
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

                    gl::GenBuffers(1, &mut uvto);
                    gl::BindBuffer(gl::ARRAY_BUFFER, uvto);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (buffers.2.len() * std::mem::size_of::<f32>()) as isize,
                        buffers.2.as_ptr() as *const _,
                        gl::DYNAMIC_DRAW
                    );

                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                }
                *instance_batch = Texture2DBatch::Loaded { instances: instances.clone(), mbo: mbo, cbo : cbo, uvto: uvto}
            }
            _ => {
                eprintln!("You try to load an allready loaded instance batch.");
            }
        }
    }

    /// Updates a specific instance in a loaded `Texture2DBatch`, modifying its transformation, color, and UV data in GPU buffers.
    pub fn update_texture2d_batch_instance(&mut self, texture2d_batch : &mut Texture2DBatch, index : isize, instance : Texture2DInstance) {
        match texture2d_batch {
            Texture2DBatch::Loaded { instances, mbo, cbo, uvto } => {
                unsafe {
                    instances[index as usize] = instance.clone();
                    //update the transform
                    let mut size = 16 * std::mem::size_of::<f32>() as isize;
                    let mut offset = index * size;
                    gl::BindBuffer(gl::ARRAY_BUFFER, *mbo);
                    gl::BufferSubData(gl::ARRAY_BUFFER, offset, size, instance.transform.as_slice().as_ptr() as *const _);
                    //update the color
                    size = 4 * std::mem::size_of::<f32>() as isize;
                    offset = index * size;
                    gl::BindBuffer(gl::ARRAY_BUFFER, *cbo);
                    gl::BufferSubData(gl::ARRAY_BUFFER, offset, size, instance.color.as_slice().as_ptr() as *const _);
                    //update uv transform
                    size = 4 * std::mem::size_of::<f32>() as isize;
                    offset = index * size;
                    gl::BindBuffer(gl::ARRAY_BUFFER, *uvto);
                    gl::BufferSubData(gl::ARRAY_BUFFER, offset, size, instance.uv_transform.as_slice().as_ptr() as *const _);
                    //reset buffer
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                }
            }
            _ => {
                eprintln!("Error: Invalid Texture2DBatch state. Expected 'Loaded' state. Ensure the Texture2DBatch is properly loaded before updating.");
            }
        }
    }

    /// Loads a font into GPU memory using FreeType, generating character textures and related data for text rendering.
    pub fn load_font(&mut self, font_file : &str, font_height : u32) -> Font{
        let mut font = Font::new();
        let library = Library::init().expect("Failed to initialize FreeType library");
        let face = library.new_face(font_file, 0).expect("Failed to load font");
        face.set_pixel_sizes(0, font_height).expect("Failed to set pixel size");

        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }

        for c in 0 .. 128 {
            let character = c as u8 as char;
            let index = face.get_char_index(c);
            match index {
                Some(index) => {
                    if index != 0 {
                        // println!("Loading cahr: {} code: {}", character, character as usize);
                        face.load_glyph(index, LoadFlag::RENDER).expect("Failed to load glyphe");
                        let glyph = face.glyph();       
                        unsafe {
                            let mut texture : GLuint = 0;
                            gl::GenTextures(1, &mut texture);
                            gl::BindTexture(gl::TEXTURE_2D, texture);
                            gl::TexImage2D(
                                gl::TEXTURE_2D,
                                0,
                                gl::RED as GLint,
                                glyph.bitmap().width() as GLsizei,
                                glyph.bitmap().rows() as GLsizei,
                                0,
                                gl::RED,
                                gl::UNSIGNED_BYTE,
                                glyph.bitmap().buffer().as_ptr() as *const GLvoid
                            );
                            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
                            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
                            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
                            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
        
                            let gfx_char = Character {
                                texture_id: texture,
                                size: Vector2::new(glyph.bitmap().width(), glyph.bitmap().rows()),
                                bearing: Vector2::new(glyph.bitmap_left(), glyph.bitmap_top()),
                                advance: glyph.advance().x
                            };
                            font.characters.insert(character, gfx_char);
                        }
                    }
                }
                None => {
                    println!("Unable to load char {}", character);
                }
            }
        };

        unsafe {
            let mut font_vao : GLuint = 0;
            gl::GenVertexArrays(1, &mut font_vao);
            gl::BindVertexArray(font_vao);
            
            let buffer_size = std::mem::size_of::<f32>() * 6 * 4;
            let stride = 4 * std::mem::size_of::<f32>();
            let mut font_vbo : GLuint = 0;
            gl::GenBuffers(1, &mut font_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, font_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                buffer_size as isize,
                std::ptr::null() as *const _,
                gl::DYNAMIC_DRAW
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, stride as i32, std::ptr::null());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            font.vao = font_vao;
            font.vbo = font_vbo;
        }
        return font;
    }

    /// Compiles an OpenGL shader from the given source code.
    /// Takes shader source and type (e.g., vertex or fragment) and compiles it into a shader ID for use.
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

    /// Creates and links a shader program from vertex and fragment shaders.
    /// Compiles vertex and fragment shaders, links them into an OpenGL program, and stores the program ID.
    pub fn build_shader_program(&mut self, shader_program : &mut ShaderProgram) {
        match shader_program {
            ShaderProgram::PreBuild { fragment_shader, vertex_shader } => {
                unsafe {
                    let vertex_shader = self.compile_shader(&vertex_shader.source, gl::VERTEX_SHADER);
                    let fragment_shader = self.compile_shader(&fragment_shader.source, gl::FRAGMENT_SHADER);
        
                    let program_id = gl::CreateProgram();
                    gl::AttachShader(program_id, vertex_shader);
                    gl::AttachShader(program_id, fragment_shader);
                    gl::LinkProgram(program_id);
                    gl::DeleteShader(vertex_shader);
                    gl::DeleteShader(fragment_shader);
                    
                    let mut success = 1;
                    let mut log = vec![0; 512];
        
                    gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
                    if success == 0 {
                        gl::GetProgramInfoLog(program_id, 512, std::ptr::null_mut(), log.as_mut_ptr() as *mut i8);
                        println!("Shader Program Linking Failed: {}", String::from_utf8_lossy(&log)); 
                    }
                    else 
                    {
                        println!("Shader Program {} created with error {}", program_id, gl::GetError());
                    }

                    *shader_program = ShaderProgram::Builded { program_id: program_id };
                }
            }
            ShaderProgram::Builded { program_id:_ } => {
                panic!("Error: Attempted to build a shader program that has already been built.");
            }
            ShaderProgram::Disposed {} => {
                panic!("Error: Cannot build a shader program that has been disposed.")
            }
        }
    }

    /// Binds a shader program for use in rendering.
    /// Sets the specified shader program as the current OpenGL program for drawing.
    pub fn bind_shader_program(&mut self, shader_program : &mut ShaderProgram) {
        match shader_program {
            ShaderProgram::Builded { program_id } => {
                unsafe {
                    gl::UseProgram(*program_id);
                    self.shader_program = *program_id;
                }
            }
            ShaderProgram::PreBuild { fragment_shader:_ , vertex_shader:_ } => {
                panic!("You try to bind an pre builded program!");
            }
            ShaderProgram::Disposed {} => {
                panic!("You try to bind an disposed program!");
            }
        }
        
    }

    /// Unbinds the currently bound shader program.
    /// Deactivates the custom shader program, resetting OpenGL to use the default program.
    pub fn unbind_shader_program(&mut self) {
        unsafe {
            gl::UseProgram(0);
            self.shader_program = 0;
        }
    }

    /// Retrieves the location of a uniform variable in a shader program.
    /// Queries OpenGL for the uniform's location and returns it, or -1 if not found.
    /// The location is used to set or get uniform values during rendering.
    pub fn get_uniform_location(&mut self, program_id : u32, name : &str) -> i32 {
        let name = CString::new(name).expect("CString::new failed");
        unsafe {
            let location= gl::GetUniformLocation(program_id, name.as_ptr());
            return location;
        }
    }

    /// Binds the specified render target (framebuffer) for rendering.
    /// This directs rendering operations to the given framebuffer instead of the default one.
    /// ### Notes:
    /// - After rendering, call `unbind_render_target` to return to the default framebuffer.
    pub fn bind_render_target(&mut self, render_target : RenderTarget) 
    {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, render_target.framebuffer_id);
        }
    }

    /// Unbinds the currently bound render target and reverts to the default framebuffer.
    /// This ensures that further rendering will happen on the default framebuffer (usually the screen).
    ///
    /// ### Notes:
    /// - Call this after rendering to a custom framebuffer to reset the rendering target.
    pub fn unbind_render_target(&mut self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    /// Initializes a shape's vertex data by creating and binding the necessary OpenGL buffers (VAO, VBO, IBO, TBO) 
    /// for the shape (e.g., vertex buffer, UV buffer, index buffer). It works with any type implementing the `Shape` trait, 
    /// providing a generic way to handle different shapes (triangles, squares, etc.).
    ///
    /// ### Returns:
    /// - `RenderData`: A struct containing the VAO, VBO, IBO, TBO, and index count for rendering the shape.
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

    /// Initializes the mesh by creating and binding the necessary OpenGL buffers (VBO, IBO, TBO) 
    /// for vertex data, index data, and texture coordinates. These buffers are stored in the provided `mesh` object 
    /// and bound to the OpenGL vertex array object (VAO) for rendering.
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

    /// Draws the mesh with the given transformation and material properties.
    /// The function applies the transformation matrix (`transform`), binds the texture for the mesh, 
    /// and uses the appropriate shaders to render the mesh. The material properties, including texture and color, 
    /// are handled, and OpenGL's `gl::DrawElements` is used to draw the mesh.
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

    /// Renders a framebuffer (render target) as a texture with the specified transformation and color tint.
    /// The render target's texture is used to represent its content, and UV coordinates are generated based on its size.
    /// This function is often used for rendering off-screen content (like post-processing effects) to the screen.
    /// The texture is rendered with the provided transformation (position, rotation, scale) and optional color tint.
    pub fn draw_texture2drt<T : ITransform>(&mut self, transform : T, render_target: &mut RenderTarget, color: Vector4<f32>) {
        let uv_buffer = utils::generate_uv_coords(render_target.size.x, render_target.size.y, Vector2::new(0.0, 0.0), Vector2::new(render_target.size.x as f32, render_target.size.y as f32));
        self.draw_texture2di_internal(transform, render_target.texture_id, color, uv_buffer);
    }

    /// Renders a 2D texture with the specified transformation and color tint.
    /// The texture's UV coordinates are generated based on its dimensions, and the texture is rendered with a given transformation
    /// (position, rotation, scale) and optional color tint. This function ensures the texture is loaded before rendering.
    /// If the texture is not loaded, an error message will be logged.
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

    /// Renders a specific subregion of a 2D texture to the screen with the specified transformation and color tint.
    /// The subregion is defined by the `point` (top-left corner) and `size` (width and height) parameters in texture space.
    /// The texture is rendered with the provided transformation (position, rotation, scale) and color tint.
    /// Useful for rendering parts of a texture, such as sprite sheets or icons.
    /// If the texture is not loaded or an error occurs, a message will be logged.
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

    /// Renders a 2D texture to the screen using the provided transformation and color tint.
    /// The texture is rendered with the specified transformation, which includes position, rotation, and scaling.
    /// The color parameter applies a tint to the texture, modifying its original colors.
    /// Assumes the full texture is rendered, and the texture size is used to generate correct UV coordinates.
    /// If the texture is not loaded or there are issues with the texture, it may fail to render and log an error.
    pub fn draw_texture2di<T: ITransform>(&mut self, transform : T, texture_size : Vector2<f32>, texture_id: u32, color: Vector4<f32>) {
        let uv_buffer = utils::generate_uv_coords(texture_size.x as u32, texture_size.y as u32, Vector2::new(0.0, 0.0), Vector2::new(texture_size.x, texture_size.y)); 
        self.draw_texture2di_internal(transform, texture_id, color, uv_buffer);
    }

    /// Renders a 2D texture with a given transformation, color filter, and UV mapping.
    /// This internal function handles the OpenGL setup to apply the transformation and color to a texture, 
    /// and then renders it using the provided texture ID and UV coordinates. 
    /// The transformation includes position, scale, and rotation, and the color is applied as a filter to the texture.
    /// Assumes UV coordinates are provided in `uv_buffer`, and the corresponding OpenGL buffers are set up for rendering.
    /// If the texture shape is missing, an error message is logged.
    fn draw_texture2di_internal<T: ITransform>(&mut self, transform : T, texture_id: u32, color: Vector4<f32>, uv_buffer : Vec<f32>) {
        let shape = self.render_shapes.get("texture_shape").copied();
        match shape {
            Some(shape) => {
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
            }
            None => {
                eprintln!("Texture shape not found!");
            }
        }
    }

    /// Renders a batch of 2D textured instances with unique transformations, colors, and UV coordinates.
    /// This function utilizes OpenGL instanced rendering for efficient rendering of multiple instances with a single draw call.
    /// Each instance can have different transformations (position, scale, rotation), color, and UV mapping.
    /// Requires a loaded texture and a properly initialized `Texture2DBatch` with buffers for model transformations, colors, and UVs.
    /// Ideal for rendering large quantities of the same object with different properties (e.g., sprites, particles).
    pub fn draw_texture2d_batch(&mut self, image_texture: &mut ImageTexture, instance_batch : &mut Texture2DBatch) {
        match image_texture {
            ImageTexture::Loaded { id, dimensions: _ } => {
                match instance_batch {
                    Texture2DBatch::Loaded { instances, mbo, cbo, uvto } => {
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

                                    //Bind the uv transform buffer
                                    gl::BindBuffer(gl::ARRAY_BUFFER, *uvto);
                                    gl::EnableVertexAttribArray(7);
                                    gl::VertexAttribPointer(7, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
                                    gl::VertexAttribDivisor(7, 1);

                                    //Draw the elements instanced
                                    gl::DrawElementsInstanced(gl::TRIANGLES, shape.index_count as i32, gl::UNSIGNED_INT, std::ptr::null(), instances.len() as i32);
                                    gl::BindVertexArray(0);
                                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                                }
                            }
                            None => {
                                eprintln!("Error: Render shape 'texture_batch_shape' not found. Ensure that the render_shapes map contains a valid entry for 'texture_batch_shape'.");
                            }
                        }
                    }
                    _ => {
                        eprintln!("Error: The provided InstanceBatch is not loaded. Ensure that the InstanceBatch is properly initialized and loaded before attempting to draw.");
                    }
                }
            }
            _ => {
                eprintln!("Error: The provided ImageTexture is not loaded. Ensure that the ImageTexture is properly initialized and loaded before attempting to draw.");
            }
        }
    }

    /// Renders a string of text at a given 2D position, applying optional scaling, color, and alignment.
    /// The function uses OpenGL with a font texture atlas to draw each character individually, adjusting position based on alignment.
    /// The text's size can be controlled by the scale parameter, and the color is applied via RGBA values.
    /// If a character is missing in the font, a warning is logged.
    /// Ideal for rendering UI elements or small text batches.
    pub fn draw_text2d(&mut self, position : Vector2<f32>, text : &str, scale : f32, color : Vector4<f32>, font : &mut Font, alignment : TextAlignment) {
        unsafe {
            let mut x = position.x;
            let y = position.y;
            let offset = font.get_offset(text, scale, alignment);

            gl::UniformMatrix4fv(self.get_uniform_location(self.shader_program, "p_mat"), 1, gl::FALSE, self.projection_matrix.as_ptr());
            gl::Uniform4f(self.get_uniform_location(self.shader_program, "vertexColor"), color.x, color.y, color.z, color.w);
            gl::BindVertexArray(font.vao);

            for c in text.chars() {
                let character = font.characters.get(&c);
                match character {
                    Some(character) => {
                        gl::ActiveTexture(gl::TEXTURE0);
                        gl::BindTexture(gl::TEXTURE_2D, character.texture_id);
                        gl::Uniform1i(self.get_uniform_location(self.shader_program, "textureSampler"), 0);

                        let xpos = (x + character.bearing.x as f32 * scale) + offset.x;
                        let ypos = (y - (character.size.y as f32 - character.bearing.y as f32) * scale) + offset.y;
                        let w = character.size.x as f32 * scale;
                        let h = character.size.y as f32 * scale;
                        
                        let vertices: [[f32;4]; 6] = [
                            [xpos, ypos + h, 0.0, 0.0],
                            [xpos, ypos, 0.0, 1.0],
                            [xpos + w, ypos, 1.0, 1.0],

                            [xpos, ypos + h, 0.0, 0.0],
                            [xpos + w, ypos, 1.0, 1.0],
                            [xpos + w, ypos + h, 1.0, 0.0]
                        ];

                        gl::BindBuffer(gl::ARRAY_BUFFER, font.vbo);
                        gl::BufferSubData(
                            gl::ARRAY_BUFFER, 
                            0 as isize, 
                            std::mem::size_of_val(&vertices) as isize, 
                            vertices.as_ptr() as *const _);
                        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        
                        gl::DrawArrays(gl::TRIANGLES, 0, 6);

                        x += ((character.advance as i32 >> 6) as f32) * scale;
                    }
                    None => {
                        println!("Character '{}' not found in this font", c);
                    }
                }
            }
            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    /// Renders a filled rectangle at a specified position with a given color, applying transformations (position, rotation, scale).
    /// Uses OpenGL to draw the rectangle with a `model-view-projection` matrix for position and scale adjustments.
    /// The function assumes the "rect_shape" is preloaded as a VAO and uses `gl::DrawElements` for rendering.
    /// Efficient for drawing background or UI elements with a single draw call.
    pub fn fill_rect<T: ITransform>(&mut self, transform : T, color : Vector4<f32>) {
        let shape = self.render_shapes.get("rect_shape").copied();
        match shape {
            Some(shape) => {
                unsafe {
                    gl::UniformMatrix4fv(self.get_uniform_location(self.shader_program, "p_mat"), 1, gl::FALSE, self.projection_matrix.as_ptr());
                    gl::UniformMatrix4fv(self.get_uniform_location(self.shader_program, "v_mat"), 1, gl::FALSE, self.view_matrix.as_ptr());
                    gl::UniformMatrix4fv(self.get_uniform_location(self.shader_program, "m_mat"), 1, gl::FALSE, transform.get_model_matrix().as_ptr());
                    gl::Uniform4f(self.get_uniform_location(self.shader_program, "vertexColor"), color.x, color.y, color.z, color.w);
                    gl::BindVertexArray(shape.vao);
                    gl::DrawElements(gl::TRIANGLES, shape.index_count as i32, gl::UNSIGNED_INT, std::ptr::null());
                    gl::BindVertexArray(0);
                }
            }
            None => {
                eprintln!("Rect shape not found!");
            }
        }
    }

    /// Renders a rectangle with a border at a given position, applying transformations (position, rotation, scale).
    /// The border width and aspect ratio are customizable for rendering across different screen resolutions.
    /// Assumes the rectangle shape (`rect_shape`) is preloaded as a VAO and uses `gl::DrawElements` for rendering.
    /// The aspect ratio is calculated and passed to the shader for proper rendering adjustments.
    /// Efficient for rendering rectangles with borders, useful for UI elements or graphical shapes.
    pub fn draw_rect<T: ITransform>(&mut self, transform : T, line_width : f32, color : Vector4<f32>) {
        let shape = self.render_shapes.get("rect_shape").copied();
        match shape {
            Some(shape) => {
                let aspect = transform.clone().get_aspect_ratio();                
                unsafe {
                    gl::UniformMatrix4fv(self.get_uniform_location(self.shader_program, "p_mat"), 1, gl::FALSE, self.projection_matrix.as_ptr());
                    gl::UniformMatrix4fv(self.get_uniform_location(self.shader_program, "v_mat"), 1, gl::FALSE, self.view_matrix.as_ptr());
                    gl::UniformMatrix4fv(self.get_uniform_location(self.shader_program, "m_mat"), 1, gl::FALSE, transform.get_model_matrix().as_ptr());
                    gl::Uniform4f(self.get_uniform_location(self.shader_program, "vertexColor"), color.x, color.y, color.z, color.w);
                    gl::Uniform1f(self.get_uniform_location(self.shader_program, "borderWidth"), line_width);
                    gl::Uniform1f(self.get_uniform_location(self.shader_program, "aspect"), aspect);
                    gl::BindVertexArray(shape.vao);
                    gl::DrawElements(gl::TRIANGLES, shape.index_count as i32, gl::UNSIGNED_INT, std::ptr::null());
                    gl::BindVertexArray(0);
                }
            }
            None => {
                eprintln!("Rect shape not found!");
            }
        }
    }

    /// Renders the texture from a `RenderTarget` as a fullscreen 2D quad.
    /// The texture is bound from the `RenderTarget` and drawn using the "framebuffer_shape" in `render_shapes` as a quad.
    /// Assumes the framebuffer shape is already loaded with the VAO for a fullscreen quad.
    /// Uses a single `gl::DrawElements` call to render the texture efficiently.
    /// Typically used for displaying textures from framebuffers to the screen.
    pub fn draw_render_target(&mut self, render_target : RenderTarget) {
        let shape = self.render_shapes.get("framebuffer_shape").copied();
        match shape {
           Some(shape) => {
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
           }
           None => {
                eprintln!("Framebuffer shape not found!");
           } 
        }
    }

    /// Disposes of all render shapes in `self.render_shapes`.
    /// This function will iterate over all the shapes in `self.render_shapes`
    /// and call `dispose_render_data` on each one to properly clean up the resources.
    pub fn dispose(&mut self) {
        let render_shapes = std::mem::take(&mut self.render_shapes);
        for (_key, mut value) in render_shapes {
            self.dispose_render_data(&mut value);
        }
    }

    /// Disposes of the resources associated with a specific `RenderData` object.
    /// This includes deleting OpenGL buffers (VBO, TBO, IBO, and VAO) if they are not zero.
    /// The function ensures that all buffers associated with the render data are safely deleted.
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

    /// Disposes of a loaded image texture by deleting the texture from OpenGL.
    /// If the texture is not loaded, it prints a message.
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

    /// Disposes of a font by deleting its VAO, VBO, and associated textures.
    /// This ensures that all resources associated with the font are cleaned up.
    pub fn dispose_font(&mut self, font : &mut Font) {
        unsafe {
            gl::DeleteVertexArrays(1, &font.vao);
            gl::DeleteBuffers(1, &font.vbo);
            for (_key, value) in &font.characters {
                gl::DeleteTextures(1, &value.texture_id);
            }
        }
    }

    /// Disposes of a render target by deleting its texture, renderbuffer, and framebuffer.
    /// This ensures that all resources associated with the render target are freed.
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

    /// Disposes of a mesh by deleting its render data and optionally disposing of its diffuse texture.
    /// This ensures that all resources associated with the mesh are cleaned up.
    pub fn dispose_mesh(&mut self, mesh : &mut Mesh, dispose_material : bool) {
        self.dispose_render_data(&mut mesh.render_data);
        if dispose_material == true {
            self.dispose_image_texture(&mut mesh.material.diffuse_texture);
        }
    }
   
    /// Disposes of a shader program by deleting its OpenGL program.
    /// This ensures that the shader program is properly cleaned up from the GPU.
    pub fn dispose_shader_program(&mut self, shader_program : &mut ShaderProgram) {
        match shader_program {
            ShaderProgram::Builded { program_id } => {
                unsafe {
                    gl::DeleteProgram(*program_id);
                    println!("Disposed shader programm {}", program_id);
                    *shader_program = ShaderProgram::Disposed{};
                }
            }
            ShaderProgram::Disposed { } => {
                panic!("You try to dispose an disposed shader!");
            }
            ShaderProgram::PreBuild { fragment_shader:_ , vertex_shader:_ } => {
                *shader_program = ShaderProgram::Disposed{};
            }
        }
    }

    /// Disposes of a texture2D batch by deleting the associated buffers (MBO, CBO, UVTO).
    /// This ensures that the batch resources are cleaned up properly.
    pub fn dispose_texture2d_batch(&mut self, instance_batch : &mut Texture2DBatch) {
        match instance_batch {
            Texture2DBatch::Loaded { instances, mbo, cbo, uvto } => {
                unsafe {
                    gl::DeleteBuffers(1, mbo);
                    gl::DeleteBuffers(1, cbo);
                    gl::DeleteBuffers(1, uvto);
                    *instance_batch = Texture2DBatch::Disposed { instances: instances.clone() }
                }
            }
            _ => {
                println!("The texture batch was not loaded!");
            }
        }
    }

    /// Retrieves the last OpenGL error code.
    /// This can be useful for debugging OpenGL calls and checking for errors.
    pub fn get_error(&self) -> u32 {
        unsafe { gl::GetError() }
    }
}