use nalgebra::{Matrix4, Rotation, Vector3};

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>
}

impl Transform {

    pub fn new(position : Vector3<f32>, rotation : Vector3<f32>, scale : Vector3<f32>) -> Transform {
        Transform {
            position: position,
            rotation: rotation,
            scale: scale
        }
    }
    
    pub fn translate(&mut self, vec : Vector3<f32>) {
        self.position = self.position + vec;
    }

    pub fn translate_xyz(&mut self, x : f32, y : f32, z : f32) {
        self.position = self.position + Vector3::new(x,y,z);
    }

    pub fn translate_xy(&mut self, x : f32, y : f32) {
        self.position = self.position + Vector3::new(x, y, 0.0);
    }

    pub fn set_position(&mut self, position : Vector3<f32>) {
        self.position = position;
    }

    pub fn set_position_xy(&mut self, x : f32, y : f32) {
        self.position = Vector3::new(x, y, 0.0);
    }

    pub fn set_position_xyz(&mut self, x : f32, y : f32, z : f32) {
        self.position = Vector3::new(x, y, z);
    }

    pub fn turn_xyz(&mut self, x : f32, y : f32, z : f32) {
        self.rotation = self.rotation + Vector3::new(x, y, z);
    }

    pub fn turn_xy(&mut self, x : f32, y : f32) {
        self.rotation = self.rotation + Vector3::new(x, y, 0.0);
    }

    pub fn turn_x(&mut self, x : f32) {
        self.rotation = self.rotation + Vector3::new(x, 0.0, 0.0);
    }

    pub fn turn_y(&mut self, y : f32) {
        self.rotation = self.rotation + Vector3::new(0.0, y, 0.0);
    }

    pub fn turn_z(&mut self, z : f32) {
        self.rotation = self.rotation + Vector3::new(0.0, 0.0, z);
    }

    pub fn set_rotation(&mut self, rotation : Vector3<f32>) {
        self.rotation = rotation;
    }

    pub fn set_rotation_xyz(&mut self, x : f32, y : f32, z : f32) {
        self.rotation = Vector3::new(x, y, z);
    }

    pub fn set_rotation_xy(&mut self, x : f32, y : f32) {
        self.rotation = Vector3::new(x, y, 0.0);
    }

    pub fn set_rotation_x(&mut self, x : f32 ) {
        self.rotation = Vector3::new(x, 0.0, 0.0);
    }

    pub fn set_rotation_y(&mut self, y : f32 ) {
        self.rotation = Vector3::new(0.0, y, 0.0);
    }

    pub fn set_rotation_z(&mut self, z : f32 ) {
        self.rotation = Vector3::new(0.0, 0.0, z);
    }

    pub fn set_scale(&mut self, scale : Vector3<f32>) {
        self.scale = scale;
    }

    pub fn set_scale_xyz(&mut self, x : f32, y : f32, z : f32) {
        self.scale = Vector3::new(x, y, z);
    }

    pub fn set_scale_xy(&mut self, x : f32, y : f32) {
        self.scale = Vector3::new(x, y, 0.0);
    }

    pub fn set_scale_x(&mut self, x : f32) {
        self.scale = Vector3::new(x, 0.0, 0.0);
    }

    pub fn set_scale_y(&mut self, y : f32) {
        self.scale = Vector3::new(0.0, y, 0.0);
    }

    pub fn set_scale_z(&mut self, z : f32) {
        self.scale = Vector3::new(0.0, 0.0, z);
    }

    pub fn get_model_matrix(self) -> Matrix4<f32> {
        let translation = Matrix4::new_translation(&self.position);
        let rotation = Rotation::from_euler_angles(self.rotation.x.to_radians(), self.rotation.y.to_radians(), self.rotation.z.to_radians()).to_homogeneous();
        let scale = Matrix4::new_nonuniform_scaling(&self.scale);

        return translation * rotation * scale;
    }

}
