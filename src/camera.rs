use crate::mat::Vec3;

#[derive(Clone)]
pub struct Camera {
    pub pos: Vec3,
    pub focus_length: f64,
    pub rotation: Vec3,
    pub vel: Vec3,
}

impl Camera {
    pub fn update_pos(&mut self, dt: f64) {
        // check collision

        // update position
        self.pos = self.pos + self.vel * dt;
    }
}
