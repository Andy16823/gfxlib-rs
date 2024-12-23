use std::path::Path;
use gltf::{material::NormalTexture, mesh::util::{ReadIndices, ReadTexCoords}, texture::Info};
use nalgebra::Vector4;

use crate::{image_texture::ImageTexture, mesh::Mesh};

pub struct AssetLoader;

impl AssetLoader {
    pub fn load_gltf(file: &str) -> Vec<Mesh> {
        let path = Path::new(file);
        let parent_path = path.parent().unwrap();

        let (gltf, buffers, _) = gltf::import(file).unwrap();
        let mut meshes = Vec::<Mesh>::new();

        for scene in gltf.scenes() {
            for node in scene.nodes() {
                if let Some(mesh) = node.mesh() {
                    for primitive in mesh.primitives() {
                        let mut gfx_mesh = Mesh::default();

                        //Load Basecolor
                        let color_factor = primitive.material().pbr_metallic_roughness().base_color_factor();
                        gfx_mesh.material.base_color_texture = AssetLoader::extract_texture(
                            primitive
                                .material()
                                .pbr_metallic_roughness()
                                .base_color_texture(),
                            parent_path.to_str().unwrap(),
                        );
                        gfx_mesh.material.base_color_friction = Vector4::new(color_factor[0], color_factor[1], color_factor[2], color_factor[3]);

                        //Load Metallic_Roughness
                        gfx_mesh.material.metallic_roughness_texture = AssetLoader::extract_texture(
                            primitive
                                .material()
                                .pbr_metallic_roughness()
                                .metallic_roughness_texture(),
                            parent_path.to_str().unwrap(),
                        );
                        gfx_mesh.material.metallic_factor = primitive.material().pbr_metallic_roughness().metallic_factor();
                        gfx_mesh.material.roughness_factor = primitive.material().pbr_metallic_roughness().roughness_factor();

                        //Load Normal Map
                        gfx_mesh.material.normal_map = AssetLoader::extract_normal_map(
                            primitive.material().normal_texture(),
                            parent_path.to_str().unwrap(),
                        );

                        let r = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                        if let Some(ReadIndices::U16(gltf::accessor::Iter::Standard(iter))) =
                            r.read_indices()
                        {
                            for v in iter {
                                gfx_mesh.indicies.push(v as u32);
                            }
                        }
                        if let Some(iter) = r.read_positions() {
                            for v in iter {
                                gfx_mesh.vertices.extend_from_slice(&v);
                            }
                        }
                        if let Some(ReadTexCoords::F32(gltf::accessor::Iter::Standard(iter))) =
                            r.read_tex_coords(0)
                        {
                            for v in iter {
                                gfx_mesh.uv_cords.extend_from_slice(&v);
                            }
                        }

                        if let Some(iter) = r.read_normals() {
                            for v in iter {
                                gfx_mesh.normals.extend_from_slice(&v);
                            }
                        }
                        meshes.push(gfx_mesh);                     
                    }
                }
            }
        }
        return meshes;
    }

    pub fn extract_texture(texture_info: Option<Info>, base_path: &str) -> Option<ImageTexture> {
        if let Some(texture_info) = texture_info {
            let texture = texture_info.texture();
            let source = texture.source().source();
            match source {
                gltf::image::Source::Uri { uri, mime_type: _ } => {
                    let texture_path = base_path.to_owned() + "/" + uri;
                    return Some(ImageTexture::load_from_file(&texture_path, false));
                }
                _ => {
                    eprintln!("Not Supportet yet!");
                }
            };
        }
        return None;
    }

    pub fn extract_normal_map(
        normal_map: Option<NormalTexture>,
        base_path: &str,
    ) -> Option<ImageTexture> {
        if let Some(normal) = normal_map {
            let texture = normal.texture();
            let source = texture.source().source();
            match source {
                gltf::image::Source::Uri { uri, mime_type: _ } => {
                    let texture_path = base_path.to_owned() + "/" + uri;
                    return Some(ImageTexture::load_from_file(&texture_path, false));
                }
                _ => {
                    eprintln!("Not Supportet yet!");
                }
            }
        }
        return None;
    }
}
