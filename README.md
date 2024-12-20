# gfxlib-rs
GFX-Lib is a lightweight library designed to simplify the creation of 2D and 3D games using OpenGL. Unlike a full-fledged game engine, GFX-Lib focuses on providing a set of functions and modules that allow developers to easily create graphics applications or games while still maintaining the freedom to implement their own abstractions. This makes GFX-Lib an ideal foundation for building custom game engines. It is comparable to other frameworks like Raylib, MonoGame, or LibGDX.

### Features
GFX-Lib offers the following modules:

- **Graphics**: Functions for rendering 2D graphics, including textures, shapes, and shader management.
- **Math**: Mathematical utilities for vectors and matrices, facilitating transformations and calculations for graphics and physics.
- **Core**: Basic utilities for creating windows, handling user input (keyboard, mouse), and managing events.
- **Shader**: A collection of predefined shaders and the ability to create custom shaders.
- **Physics**: Not implemented yet.
- **Audio**: Not implemented yet.

*Note: 3D rendering is currently under development and will be available in future versions.*

### Example
Here’s a simple example showing how to create a window, load textures, and handle basic user input:

```RUST
use std::time::Instant;
use gfx::core::transform::Transform2D;
use gfx::graphics::camera::Camera;
use gfx::graphics::game_window::Key;
use gfx::graphics::game_window::Window;
use gfx::graphics::image_texture::ImageTexture;
use gfx::graphics::TextAlignment;
use gfx::shader::prebuild_shader::FontShader;
use gfx::shader::prebuild_shader::Texture2DShader;
use gfx::math::Vector2;
use gfx::math::Vector3;

fn main() {
    // Create the main game window with dimensions 800x600 and the title "My Game".
    let mut window = Window::new(800, 600, "My Game", true);

    // Disable depth testing for this application.
    window.render_device.disable_depth_test();

    // Get the executable's path for resource loading.
    let binding = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
    let exe_path = binding.to_str().unwrap();
    println!("{}", exe_path);

    // Create a camera for rendering, with position and size set.
    let mut camera = Camera {
        size: Vector3::new(800.0, 600.0, 0.0),
        position: Vector3::new(0.0,0.0, 0.0),
        near: 1.0,
        far: -1.0
    };

    // Load a font to be used for rendering text.
    let mut font = window.render_device.load_font("C:/Users/andy1/Downloads/Gamer.ttf", 24);

    // Create render targets for different types of rendering operations.
    let mut animated_sprite_rt = window.render_device.create_render_target(800, 600);
    let mut sprite_rt = window.render_device.create_render_target(800, 600);
    let mut text_rt = window.render_device.create_render_target(800, 600);
    let mut rect_rt = window.render_device.create_render_target(800, 600);

    //Create a texture2d_shader for the texture rendering
    let mut texture2d_shader = Texture2DShader::build_shader_program();
    window.render_device.create_shader_program(&mut texture2d_shader);

    // Initialize various shader programs for rendering.
    let mut screen_shader = ScreenShader::build_shader_program();
    window.render_device.create_shader_program(&mut screen_shader);

    let mut font_shader = FontShader::build_shader_program();
    window.render_device.create_shader_program(&mut font_shader);

    // Load image textures for rendering sprites.
    let mut image = ImageTexture::load_from_file("C:/Users/andy1/Downloads/MainGuySpriteSheet.png");
    window.render_device.load_texture(&mut image);

    // Setup animation data for sprite frames.
    let mut last_frame = Instant::now();
    let mut current_frame_index = 0;
    let animation_frames = vec![
        utils::get_subimage(&mut image, 3, 4, 0, 0),
        utils::get_subimage(&mut image, 3, 4, 1, 0),
        utils::get_subimage(&mut image, 3, 4, 2, 0),
    ];
    let mut current_frame = animation_frames[0];

    // Initialize transforms for rendering objects.
    let mut player_transform = Transform2D::new(Vector2::new(0.0, 0.0), 0.0, Vector2::new(32.0, 32.0));

    // The main game loop runs until the window is closed.
    while !window.should_close() {
        // Process user input events.
        window.poll_events();

        // Update the animation frame based on time elapsed.
        if last_frame.elapsed().as_millis() > 100 {
            current_frame_index += 1;
            if current_frame_index == animation_frames.len() -1 {
                current_frame_index = 0;
            }
            current_frame = animation_frames[current_frame_index];
            last_frame = Instant::now();
        }

        // Handle keyboard input for player movement or exiting the game.
        if window.key_down(Key::Left) || window.key_down(Key::A) {
            player_transform.translate_xy(-5.0, 0.0);
        }
        else if window.key_down(Key::Right) || window.key_down(Key::D) { 
            player_transform.translate_xy(5.0, 0.0);
        }
        else if window.key_down(Key::Escape) {
            window.close_window();
        }  

        // Set the viewport and camera for rendering.
        window.render_device.set_viewport(window.get_viewport());
        window.render_device.set_camera(&mut camera);

        // Render the animated sprite to its render target.
        window.render_device.resize_render_target(&mut animated_sprite_rt, window.get_viewport().size.x, window.get_viewport().size.y);
        window.render_device.bind_render_target(animated_sprite_rt);
        window.render_device.clear_color(Vector4::new(0.0, 0.0, 0.0, 0.0));
        window.render_device.clear();
        window.render_device.bind_shader_program(&mut texture2d_shader); 
        window.render_device.draw_sub_texture2d(player_transform.clone(), Vector2::new(current_frame.x, current_frame.y), Vector2::new(current_frame.width, current_frame.height), &mut image, Vector4::new(1.0, 1.0, 1.0, 1.0)); // Draw the texture as a subtexture
        window.render_device.unbind_shader_program();
        window.render_device.unbind_render_target();

        // Combine the render targets onto the screen in a specific order.
        window.render_device.clear_color(Vector4::new(0.07, 0.0, 0.05, 1.0));
        window.render_device.clear();
        window.render_device.bind_shader_program(&mut screen_shader);
        window.render_device.draw_render_target(sprite_rt);
        window.render_device.draw_render_target(animated_sprite_rt);
        window.render_device.unbind_shader_program();

        // Swap the buffers and display the rendered result
        window.swap_buffers();
    }

    // Cleanup the shaders, images, and render targets
    window.render_device.dispose_shader_program(&mut texture2d_shader);
    window.render_device.dispose_shader_program(&mut screen_shader);
    window.render_device.dispose_render_target(&mut sprite_rt);
    window.render_device.dispose_render_target(&mut animated_sprite_rt);
    window.render_device.dispose_image_texture(&mut image);
    window.render_device.dispose_font(&mut font);
    window.render_device.dispose();
    println!("Finished with error {}", window.render_device.get_error());
}
```
*This example creates a simple 2D game window, loads a sprite sheet, handles user input, and renders animations and static images to the screen.*

### Dependencies
The project uses the following dependencies:
- beryllium: Version 0.13.3 | License: Apache-2.0
- freetype-rs: Version * (with bundled feature) | License: MIT
- gl: Version 0.14.0 | License: Apache-2.0
- glfw: Version 0.59.0 | License: Zlib
- nalgebra: Version 0.33.2 | License: Apache-2.0
- stb_image: Version 0.3.0 | License: MIT
- uuid: Version 1.11.0 (with serde and v4 features) | License: MIT

### Installation
To use GFX-Lib in your project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
gfxlib = { git = "https://github.com/Andy16823/gfxlib-rs" }
```

### Contributing
Feel free to open issues or submit pull requests. Contributions are always welcome!
