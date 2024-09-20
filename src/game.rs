use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};

use crate::loader::{self, load};
use crate::GW;
use crate::{camera::Camera, mat::*};
use crate::{
    renderer::{self, *},
    MAP,
};
use core::panic;
use std::thread;
use std::time::{Duration, Instant};

pub struct Game {
    pub renderer: Screen,
    pub camera: Camera,
}

const SPEED: f64 = 50.;
const ROTATION_SPEED: f64 = 1.5;
const GRAVITY: f64 = 80.;

impl Game {
    pub fn run(&mut self) {
        // load map files
        // generate map meshes

        let mesh = load(loader::MAP).0;

        let mut level_timer = Instant::now();

        // timer for fps
        let mut time = Instant::now();

        // device for input
        let device_state = DeviceState::new();

        let floors = renderer::map_as_vec_of_floors(MAP).len();

        loop {
            // reset timer
            let dt = time.elapsed().as_secs_f64();
            time = Instant::now();

            // get list off all vertices

            let fps_text = &format!("fps: {:.2?} ", 1. / (dt));
            let timer_text = &format!("time: {:.1?} ", level_timer.elapsed());
            let floor_text = &format!(
                "floor: {}/{}",
                (-self.camera.pos.y.div_euclid(GW)).clamp(0., floors as f64) as usize,
                floors
            );

            // render vertices
            self.renderer.render_mt(
                &self.camera,
                &mesh,
                &format!("{}{}{}", fps_text, timer_text, floor_text),
                true,
            );

            // handle input
            let mouse = device_state.get_mouse();
            let keys = device_state.get_keys();

            if keys.contains(&Keycode::E) {
                let _ = crossterm::terminal::disable_raw_mode().unwrap();

                let mut stdout = std::io::stdout();
                crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen).unwrap();
                panic!("Exited app")
            }
            if keys.contains(&Keycode::M) {
                let time1 = time.elapsed();
                time = Instant::now();
                self.renderer
                    .render_map(loader::MAP, self.camera.pos, loader::GW);
                loop {
                    if device_state.get_keys().contains(&Keycode::M) {
                        if time.elapsed() < Duration::from_millis(150) {
                            continue;
                        }
                        thread::sleep_ms(150);
                        break;
                    }
                }
                time = Instant::now().checked_sub(time1).unwrap();
            }
            if keys.contains(&Keycode::Left) {
                self.camera.rotation.x -= ROTATION_SPEED * dt;
            }
            if keys.contains(&Keycode::Right) {
                self.camera.rotation.x += ROTATION_SPEED * dt;
            }
            if keys.contains(&Keycode::Up) {
                if self.camera.rotation.y < 1.5 {
                    self.camera.rotation.y += ROTATION_SPEED * dt;
                }
            }
            if keys.contains(&Keycode::Down) {
                if self.camera.rotation.y > -1.5 {
                    self.camera.rotation.y -= ROTATION_SPEED * dt;
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

            // replace with collision statement

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
