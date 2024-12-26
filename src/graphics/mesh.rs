use nalgebra::{Matrix4, Quaternion, UnitQuaternion, Vector3, Vector4};
use super::{material::Material, RenderData};

#[derive(Default)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub uv_cords: Vec<f32>,
    pub indicies : Vec<u32>,
    pub normals : Vec<f32>,
    pub material: Material,
    pub render_data : RenderData,
    pub local_translation : Vector3<f32>,
    pub local_rotation : Vector4<f32>,
    pub local_scale : Vector3<f32> 
}

impl Mesh {
    pub fn get_local_matrix(&mut self) -> Matrix4<f32> {
        let translation = Matrix4::new_translation(&self.local_translation);
        let rotation_quat = Quaternion::new(
            self.local_rotation.w,
            self.local_rotation.x,
            self.local_rotation.y,
            self.local_rotation.z,
        );
        let rotation = UnitQuaternion::from_quaternion(rotation_quat).to_homogeneous();
        let scale = Matrix4::new_nonuniform_scaling(&self.local_scale);

        return translation * rotation * scale;
    }
}