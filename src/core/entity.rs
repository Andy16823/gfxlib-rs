use uuid::Uuid;

use crate::mesh::Mesh;

use super::transform::Transform3D;

pub trait Entity {
    fn set_transform(&mut self, transform : Transform3D);
    fn get_uuid(&mut self) -> Uuid;
    fn get_mesh(&mut self) -> &mut Mesh;
    fn get_transform(&mut self) -> &mut Transform3D;
}