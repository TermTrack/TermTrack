use crate::mat::Vec3;

#[derive(Clone)]
pub struct Camera {
    pub pos: Vec3,
    pub focus_length: f64,
    pub rotation: Vec3,
}
