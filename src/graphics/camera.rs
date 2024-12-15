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
pub struct Camera {
    pub position: Vector3<f32>,
    pub size: Vector3<f32>,
    pub near: f32,
    pub far: f32,
}

impl ICamera for Camera {
    fn get_projection_matrix(&mut self, viewport : Viewport, screen_correction : f32) -> Matrix4<f32> {
        let half_width =  (viewport.size.x as f32 / 2.0) / screen_correction;
        let half_height = (viewport.size.y as f32 / 2.0) / screen_correction;
        let left = self.position.x - half_width;
        let right = self.position.x + half_width;
        let bottom = self.position.y - half_height;
        let top = self.position.y + half_height;

        println!("vp width {} vp height {}", left, right);

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
