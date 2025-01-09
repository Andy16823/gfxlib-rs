use nalgebra::Vector3;

pub trait ILight {
    fn get_light_intensity(&mut self) -> f32;
    fn get_light_color(&mut self) -> Vector3<f32>;
    fn get_light_pos(&mut self) -> Vector3<f32>;
}

pub struct PointLight3D {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
    pub intensity: f32,
}

impl ILight for PointLight3D {
    fn get_light_intensity(&mut self) -> f32 {
        return self.intensity;
    }

    fn get_light_color(&mut self) -> Vector3<f32> {
        return self.color;
    }

    fn get_light_pos(&mut self) -> Vector3<f32> {
        return self.position;
    }
}

impl PointLight3D {
    pub fn new(position: Vector3<f32>, color: Vector3<f32>, intensity: f32) -> PointLight3D {
        PointLight3D {
            position,
            color,
            intensity
        }
    }
}
