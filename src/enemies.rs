use crate::loader::*;
use crate::mat::*;

#[derive(Clone)]
pub struct Enemy {
    pos: Vec3,
    speed: f64,
    vel: Vec3,
    collider: BoxCollider,
    mesh: Mesh,
    vision: f64,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            pos: Vec3 {
                x: GW * 0.5,
                y: GH * 0.5,
                z: GW * 0.5,
            },
            speed: 23.,
            vel: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            collider: BoxCollider {
                max_x: GW * 0.1,
                max_y: GW * 0.1,
                max_z: GW * 0.1,
                min_x: -GW * 0.1,
                min_y: -GW * 0.1,
                min_z: -GW * 0.1,
                tag: Some("angry_pixel"),
            },
            mesh: Mesh::new(Vec::from([
                //left
                (-GW * 0.1, -GW * 0.1, -GW * 0.1),
                (-GW * 0.1, -GW * 0.1, GW * 0.1),
                (-GW * 0.1, GW * 0.1, GW * 0.1),
                (255., 0., 0.),
                (-GW * 0.1, -GW * 0.1, -GW * 0.1),
                (-GW * 0.1, GW * 0.1, GW * 0.1),
                (-GW * 0.1, GW * 0.1, -GW * 0.1),
                (255., 0., 0.),
                //front
                (-GW * 0.1, -GW * 0.1, -GW * 0.1),
                (GW * 0.1, -GW * 0.1, -GW * 0.1),
                (GW * 0.1, GW * 0.1, -GW * 0.1),
                (255., 0., 0.),
                (-GW * 0.1, -GW * 0.1, -GW * 0.1),
                (-GW * 0.1, GW * 0.1, -GW * 0.1),
                (GW * 0.1, GW * 0.1, -GW * 0.1),
                (255., 0., 0.),
                //right
                (GW * 0.1, -GW * 0.1, -GW * 0.1),
                (GW * 0.1, -GW * 0.1, GW * 0.1),
                (GW * 0.1, GW * 0.1, GW * 0.1),
                (255., 0., 0.),
                (GW * 0.1, -GW * 0.1, -GW * 0.1),
                (GW * 0.1, GW * 0.1, GW * 0.1),
                (GW * 0.1, GW * 0.1, -GW * 0.1),
                (255., 0., 0.),
                //back
                (-GW * 0.1, -GW * 0.1, GW * 0.1),
                (GW * 0.1, -GW * 0.1, GW * 0.1),
                (GW * 0.1, GW * 0.1, GW * 0.1),
                (255., 0., 0.),
                (-GW * 0.1, -GW * 0.1, GW * 0.1),
                (-GW * 0.1, GW * 0.1, GW * 0.1),
                (GW * 0.1, GW * 0.1, GW * 0.1),
                (255., 0., 0.),
                //top
                (-GW * 0.1, GW * 0.1, -GW * 0.1),
                (-GW * 0.1, GW * 0.1, GW * 0.1),
                (GW * 0.1, GW * 0.1, -GW * 0.1),
                (255., 0., 0.),
                (GW * 0.1, GW * 0.1, GW * 0.1),
                (-GW * 0.1, GW * 0.1, GW * 0.1),
                (GW * 0.1, GW * 0.1, -GW * 0.1),
                (255., 0., 0.),
                // bottom
                (-GW * 0.1, -GW * 0.1, -GW * 0.1),
                (-GW * 0.1, -GW * 0.1, GW * 0.1),
                (GW * 0.1, -GW * 0.1, -GW * 0.1),
                (255., 0., 0.),
                (GW * 0.1, -GW * 0.1, GW * 0.1),
                (-GW * 0.1, -GW * 0.1, GW * 0.1),
                (GW * 0.1, -GW * 0.1, -GW * 0.1),
                (255., 0., 0.),
            ])),
            vision: GW * 4.,
        }
    }
}

impl Enemy {
    pub fn update(&mut self, dt: f64, player_positon: Vec3, colliders: &Vec<BoxCollider>) {
        let vec_to_player = player_positon - self.pos;
        if (vec_to_player).abs() < self.vision {
            self.vel = vec_to_player.norm() * self.speed;
            let mut col = self.collider.clone();
            check_collision(
                &mut col,
                &mut self.pos,
                &mut self.vel,
                dt,
                colliders,
                &mut false,
            );
            self.pos = self.pos + self.vel * dt;
        }
    }
    pub fn get_collider(&self) -> BoxCollider {
        let mut col = self.collider.clone();
        col.translate(self.pos);
        col
    }

    pub fn translate(mut self, to: Vec3) -> Self {
        self.pos = self.pos + to;
        self
    }

    pub fn get_mesh(&self) -> Mesh {
        let mut mesh = self.mesh.clone();
        for tri in mesh.mut_tris() {
            tri.v0 = tri.v0
                + Vec3 {
                    x: self.pos.x,
                    z: self.pos.z,
                    y: self.pos.y,
                };
            tri.v1 = tri.v1
                + Vec3 {
                    x: self.pos.x,
                    z: self.pos.z,
                    y: self.pos.y,
                };
            tri.v2 = tri.v2
                + Vec3 {
                    x: self.pos.x,
                    z: self.pos.z,
                    y: self.pos.y,
                };
        }
        mesh
    }
}
