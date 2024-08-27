use crate::mat::Vec3;

#[derive(Clone)]
pub struct Camera {
    pub pos: Vec3,
    pub fov: f64,
}
