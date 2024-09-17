use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};

use crate::loader::{self, load};
use crate::renderer::Screen;
use crate::{camera::Camera, mat::*};
use core::panic;
use std::time::{Duration, Instant};

pub struct Game {
    pub renderer: Screen,
    pub camera: Camera,
}

const SPEED: f64 = 20.;
const GRAVITY: f64 = 80.;

impl Game {
    pub fn run(&mut self) {
        // load map files
        // generate map meshes

        let mesh = load(loader::MAP).0;

        // timer for fps
        let mut time = Instant::now();

        // device for input
        let device_state = DeviceState::new();

        loop {
            // reset timer
            let dt = time.elapsed().as_secs_f64();
            time = Instant::now();

            // get list off all vertices

            let fps = &format!("\r\nfps: {:.2?}", 1. / (dt));

            // render vertices
            self.renderer
                .render_mt(&self.camera, &mesh, &format!("{} dt: {}", fps, dt), true);

            // handle input
            let mouse = device_state.get_mouse();
            let keys = device_state.get_keys();

            if keys.contains(&Keycode::E) {
                let _ = crossterm::terminal::disable_raw_mode().unwrap();
                panic!("Exited app")
            }
            if keys.contains(&Keycode::Left) {
                self.camera.rotation.x -= 0.05;
            }
            if keys.contains(&Keycode::Right) {
                self.camera.rotation.x += 0.05;
            }
            if keys.contains(&Keycode::Up) {
                if self.camera.rotation.y < 1.5 {
                    self.camera.rotation.y += 0.05;
                }
            }
            if keys.contains(&Keycode::Down) {
                if self.camera.rotation.y > -1.5 {
                    self.camera.rotation.y -= 0.05;
                }
            }

            let mut v = Vec3 {
                x: 0.,
                y: self.camera.vel.y,
                z: 0.,
            };

            if keys.contains(&Keycode::W) {
                v = v + Vec3 {
                    x: 0.,
                    y: 0.,
                    z: SPEED,
                }
                .rotate_y(self.camera.rotation.x);
            }
            if keys.contains(&Keycode::A) {
                v = v + Vec3 {
                    x: -SPEED,
                    y: 0.,
                    z: 0.,
                }
                .rotate_y(self.camera.rotation.x);
            }
            if keys.contains(&Keycode::D) {
                v = v + Vec3 {
                    x: SPEED,
                    y: 0.,
                    z: 0.,
                }
                .rotate_y(self.camera.rotation.x);
            }
            if keys.contains(&Keycode::S) {
                v = v + Vec3 {
                    x: 0.,
                    y: 0.,
                    z: -SPEED,
                }
                .rotate_y(self.camera.rotation.x);
            }

            // jump, gravity & collision
            self.camera.vel = v;

            self.camera.vel.y += GRAVITY * dt;

            const PLAYER_WIDTH: f64 = 1.;
            if let Some(d) = collides(
                &mesh,
                self.camera.pos,
                Vec3 {
                    x: 0.,
                    y: self.camera.vel.y,
                    z: 0.,
                },
                PLAYER_WIDTH,
                dt,
            ) {
                self.camera.vel.y = self.camera.vel.y.signum() * d;

                if keys.contains(&Keycode::Space) && self.camera.vel.y >= 0. {
                    self.camera.vel.y = -40.
                }
            }

            if let Some(d) = collides(
                &mesh,
                self.camera.pos,
                Vec3 {
                    x: self.camera.vel.x,
                    y: 0.,
                    z: self.camera.vel.z,
                },
                PLAYER_WIDTH,
                dt,
            ) {
                let new_v = Vec3 {
                    x: self.camera.vel.x,
                    y: 0.,
                    z: self.camera.vel.z,
                }
                .norm()
                    * d;
                self.camera.vel.x = new_v.x;
                self.camera.vel.z = new_v.z;
            }

            self.camera.update_pos(dt);
        }
    }
}
