use nalgebra::{Matrix4, Vector3};

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>
}

impl Transform {

    pub fn translate(&mut self, vec : Vector3<f32>) {
        self.position = self.position + vec;
    }

    pub fn translate_xyz(&mut self, x : f32, y : f32, z : f32) {
        self.position = self.position + Vector3::new(x,y,z);
    }

    pub fn translate_xy(&mut self, x : f32, y : f32) {
        self.position = self.position + Vector3::new(x, y, 0.0);
    }

    pub fn get_model_matrix(&mut self) -> Matrix4<f32> {
        let translation = Matrix4::new_translation(&self.position);
        let rotation = Matrix4::new_rotation(self.rotation);
        let scale = Matrix4::new_nonuniform_scaling(&self.scale);

        return translation * rotation * scale;
    }
}
