# gfxlib-rs
GFX-Lib is a lightweight library designed to simplify the creation of 2D and 3D games using OpenGL. Unlike a full-fledged game engine, GFX-Lib focuses on providing a set of functions and modules that allow developers to easily create graphics applications or games while still maintaining the freedom to implement their own abstractions. This makes GFX-Lib an ideal foundation for building custom game engines. It is comparable to other frameworks like Raylib, MonoGame, or LibGDX.

## Features
GFX-Lib offers the following modules:

- **Graphics**: Functions for rendering 2D graphics, including textures, shapes, and shader management.
- **Math**: Mathematical utilities for vectors and matrices, facilitating transformations and calculations for graphics and physics.
- **Core**: Basic utilities for creating windows, handling user input (keyboard, mouse), and managing events.
- **Shader**: A collection of predefined shaders and the ability to create custom shaders.
- **Physics**: Not implemented yet.
- **Audio**: Not implemented yet.

*Note: 3D rendering is currently under development and will be available in future versions.*

## Example
See the Github wiki for many examples of how you can use GFX-Lib. 
https://github.com/Andy16823/gfxlib-rs/wiki/Examples

## Dependencies
The project uses the following dependencies:
- [beryllium](https://crates.io/crates/beryllium) | License: Apache-2.0
- [freetype-rs](https://crates.io/crates/freetype-rs) | License: MIT
- [gl](https://crates.io/crates/gl) | License: Apache-2.0
- [glfw](https://crates.io/crates/glfw) | License: Zlib
- [nalgebra](https://crates.io/crates/nalgebra) | License: Apache-2.0
- [stb_image](https://crates.io/crates/stb_image) | License: MIT
- [uuid](https://crates.io/crates/uuid) | License: MIT
- [gltf](https://github.com/gltf-rs/gltf) | License: MIT

## Installation
To use GFX-Lib in your project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
gfx = { git = "https://github.com/Andy16823/gfxlib-rs" }
```

## Contributing
Feel free to open issues or submit pull requests. Contributions are always welcome! Also make sure to join the gfx discord channel https://discord.com/invite/qZRgRKedBs
