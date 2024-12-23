use nalgebra::{Matrix4, Point3, Vector3};

use super::viewport::Viewport;

pub trait ICamera {
    fn get_view_matrix(&mut self) -> Matrix4<f32>;
    fn get_projection_matrix(&mut self, viewport : Viewport, screen_correction : f32) -> Matrix4<f32>;
    fn set_camera_size(&mut self, vec : Vector3<f32>);
    fn sync_camera_size(&mut self, viewport : Viewport);
    fn set_camera_position(&mut self, vec : Vector3<f32>);
}

#[derive(Default)]
pub struct OrthographicCamera {
    pub position: Vector3<f32>,
    pub size: Vector3<f32>,
    pub near: f32,
    pub far: f32,
}

impl ICamera for OrthographicCamera {
    fn get_projection_matrix(&mut self, viewport : Viewport, screen_correction : f32) -> Matrix4<f32> {
        let half_width =  (viewport.size.x as f32 / 2.0) / screen_correction;
        let half_height = (viewport.size.y as f32 / 2.0) / screen_correction;
        let left = self.position.x - half_width;
        let right = self.position.x + half_width;
        let bottom = self.position.y - half_height;
        let top = self.position.y + half_height;

        let p_mat = Matrix4::new_orthographic(left, right, bottom, top, self.near, self.far);
        return p_mat;
    }

    fn get_view_matrix(&mut self) -> Matrix4<f32> {
        let v_mat = Matrix4::look_at_rh(
            &Point3::new(0.0, 0.0, 1.0),
            &Point3::new(0.0, 0.0, 0.0),
            &Vector3::new(0.0, 1.0, 0.0),
        );
        return v_mat;
    }

    fn set_camera_position(&mut self, vec : Vector3<f32>) {
        self.position = vec;
    }

    fn set_camera_size(&mut self, vec : Vector3<f32>) {
        self.size = vec;
    }

    fn sync_camera_size(&mut self, viewport : Viewport) {
        self.size = Vector3::new(viewport.size.x as f32, viewport.size.y as f32, 0.0);
    }

}

#[derive(Default)]
pub struct PerspectiveCamera {
    pub position: Vector3<f32>,
    pub rotation : Vector3<f32>,
    pub size: Vector3<f32>,
    pub near: f32,
    pub far: f32,
    pub fov: f32
}

impl PerspectiveCamera {
    
    pub fn new(position : Vector3<f32>, rotation : Vector3<f32>, size : Vector3<f32>, near : f32, far : f32, fov : f32) -> PerspectiveCamera {
        PerspectiveCamera{
            position: position,
            rotation: rotation,
            size: size,
            near: near,
            far: far,
            fov: fov
        }
    }

    pub fn translate(&mut self, vec : Vector3<f32>) {
        self.position = self.position + vec;
    }

    pub fn translate_xyz(&mut self, x : f32, y : f32, z : f32) {
        self.translate(Vector3::new(x, y, z));
    }

    pub fn get_camera_front3(&mut self) -> Vector3<f32> {
        let x = f32::cos(self.rotation.y.to_radians()) * f32::cos(self.rotation.x.to_radians());
        let y = f32::sin(self.rotation.x.to_radians());
        let z = f32::sin(self.rotation.y.to_radians()) * f32::cos(self.rotation.x.to_radians());
        
        Vector3::new(x, y, z).normalize()
    }

    pub fn get_camera_front2(&mut self) -> Vector3<f32> {
        let z = f32::cos(self.rotation.y.to_radians()) * f32::cos(self.rotation.x.to_radians());
        let y = f32::sin(self.rotation.x.to_radians());
        let x = f32::sin(self.rotation.y.to_radians()) * f32::cos(self.rotation.x.to_radians());
        
        Vector3::new(x, y, z).normalize()
    }

    pub fn get_camera_front(&mut self) -> Vector3<f32> {
        let z = -f32::cos(self.rotation.y.to_radians()) * f32::cos(self.rotation.x.to_radians()); // Negatives Z
        let y = f32::sin(self.rotation.x.to_radians());
        let x = -f32::sin(self.rotation.y.to_radians()) * f32::cos(self.rotation.x.to_radians()); // Negatives X
        
        Vector3::new(x, y, z).normalize()
    }

    pub fn get_target_point(&mut self) -> Point3<f32>{
        let target = self.position + self.get_camera_front();
        return Point3::new(target.x, target.y, target.z);
    }

    pub fn get_camera_front_p32(&mut self) -> Point3<f32> {
        let front = self.get_camera_front();
        return Point3::new(front.x, front.y, front.z);
    }

    pub fn get_aspect_ratio(&mut self) -> f32 {
        return self.size.x / self.size.y;
    }

}

impl ICamera for PerspectiveCamera {
    fn get_projection_matrix(&mut self, _viewport : Viewport, _screen_correction : f32) -> Matrix4<f32> {
        return Matrix4::new_perspective(self.get_aspect_ratio(), self.fov, self.near, self.far);
    }

    fn get_view_matrix(&mut self) -> Matrix4<f32> {
        let camera_position = Point3::new(self.position.x, self.position.y, self.position.z);
        return Matrix4::look_at_rh(&camera_position, &self.get_target_point(), &Vector3::new(0.0, 1.0, 0.0));
    }

    fn set_camera_position(&mut self, vec : Vector3<f32>) {
        self.position = vec;
    }

    fn set_camera_size(&mut self, vec : Vector3<f32>) {
        self.size = vec;
    }

    fn sync_camera_size(&mut self, viewport : Viewport) {
        self.size = Vector3::new(viewport.size.x as f32, viewport.size.y as f32, 0.0);
    }

}
