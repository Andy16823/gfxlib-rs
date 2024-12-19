use nalgebra::{Matrix4, Rotation, Vector2, Vector3};

pub trait ITransform: Clone {
    fn get_model_matrix(self) -> Matrix4<f32>;
    fn get_aspect_ratio(self) -> f32;
}

#[derive(Clone, Copy)]
pub struct Transform2D {
    pub position : Vector2<f32>,
    pub rotation : f32,
    pub scale : Vector2<f32>
}

impl ITransform for Transform2D {
    fn get_model_matrix(self) -> Matrix4<f32> {
        let pos = Vector3::<f32>::new(self.position.x, self.position.y, 0.0);
        let scale = Vector3::<f32>::new(self.scale.x, self.scale.y, 0.0);

        let translation = Matrix4::new_translation(&pos);
        let rotation = Rotation::from_euler_angles(0.0, 0.0, self.rotation.to_radians()).to_homogeneous();
        let scale = Matrix4::new_nonuniform_scaling(&scale);

        return translation * rotation * scale;
    }

    fn get_aspect_ratio(self) -> f32 {
        return self.scale.x / self.scale.y;
    }
}

impl Transform2D {

    pub fn new(position : Vector2<f32>, rotation :f32, scale : Vector2<f32>) -> Transform2D {
        Transform2D {
            position: position,
            rotation: rotation,
            scale: scale
        }
    }
    
    pub fn translate(&mut self, vec : Vector2<f32>) {
        self.position = self.position + vec;
    }

    pub fn translate_xy(&mut self, x : f32, y : f32) {
        self.position = self.position + Vector2::new(x, y);
    }

    pub fn set_position(&mut self, position : Vector2<f32>) {
        self.position = position;
    }

    pub fn set_position_xy(&mut self, x : f32, y : f32) {
        self.position = Vector2::new(x, y);
    }

    pub fn turn(&mut self, value : f32) {
        self.rotation = self.rotation + value;
    }

    pub fn set_rotation(&mut self, rotation : f32) {
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, scale : Vector2<f32>) {
        self.scale = scale;
    }

    pub fn set_scale_xy(&mut self, x : f32, y : f32) {
        self.scale = Vector2::new(x, y);
    }

    pub fn set_scale_x(&mut self, x : f32) {
        self.scale = Vector2::new(x, 0.0);
    }

    pub fn set_scale_y(&mut self, y : f32) {
        self.scale = Vector2::new(0.0, y);
    }
}

#[derive(Clone, Copy)]
pub struct Transform3D {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>
}

impl ITransform for Transform3D {
    fn get_model_matrix(self) -> Matrix4<f32> {
        let translation = Matrix4::new_translation(&self.position);
        let rotation = Rotation::from_euler_angles(self.rotation.x.to_radians(), self.rotation.y.to_radians(), self.rotation.z.to_radians()).to_homogeneous();
        let scale = Matrix4::new_nonuniform_scaling(&self.scale);

        return translation * rotation * scale;
    }

    fn get_aspect_ratio(self) -> f32 {
        return self.scale.x / self.scale.y;
    }
}

impl Transform3D {

    pub fn new(position : Vector3<f32>, rotation : Vector3<f32>, scale : Vector3<f32>) -> Transform3D {
        Transform3D {
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

}
