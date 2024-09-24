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
const PLAYER_WIDTH: f64 = 1.;
const PLAYER_COLLIDER: ((f64, f64, f64), (f64, f64, f64)) = ((-1., 6., -1.), (1., -2., 1.));

impl Game {
    pub fn run(&mut self) {
        // load map files
        // generate map meshes

        let (mesh, colliders, start) = load(loader::MAP);

        self.camera.pos = Vec3 {
            x: start.0,
            y: start.1,
            z: start.2,
        };

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

            // add gravity
            self.camera.vel = v;
            self.camera.vel.y += GRAVITY * dt;

            // collision
            let mut current_pc = BoxCollider::new(PLAYER_COLLIDER.0, PLAYER_COLLIDER.1, None);
            let mut grounded = false;
            check_collision(
                &mut current_pc,
                &mut self.camera.pos,
                &mut self.camera.vel,
                dt,
                &colliders,
                &mut grounded,
            );

            if grounded && keys.contains(&Keycode::Space) {
                self.camera.vel.y = -40.;
            }
            // // get current player collider & translate it to position + velocity vector
            // current_pc.translate(self.camera.pos);

            // for collider in colliders.iter() {
            //     // temporary variable for imagining next position
            //     let mut next_pc = current_pc.clone();

            //     // adding x distance to next_pc
            //     next_pc.translate(Vec3 {
            //         x: self.camera.vel.x * dt,
            //         y: 0.,
            //         z: 0.,
            //     });

            //     // adding z distance to next_pc
            //     next_pc.translate(Vec3 {
            //         x: 0.,
            //         y: 0.,
            //         z: self.camera.vel.z * dt,
            //     });

            //     // checking for collision in z and fixing position
            //     if next_pc.intersects(collider) {
            //         // calculate collided distance, set position to not colliding & delete velocity in z direction
            //         if dir.z < 0. {
            //             self.camera.pos.z +=
            //                 (collider.max_z - next_pc.min_z) + self.camera.vel.z * dt;
            //         } else if dir.z > 0. {
            //             self.camera.pos.z +=
            //                 (collider.min_z - next_pc.max_z) + self.camera.vel.z * dt;
            //         }
            //         self.camera.vel.z = 0.;
            //         current_pc = BoxCollider::new(PLAYER_COLLIDER.0, PLAYER_COLLIDER.1);
            //         current_pc.translate(self.camera.pos);
            //         continue;
            //     }

            //     // adding y distance to next_pc
            // next_pc.translate(Vec3 {
            //     x: 0.,
            //     y: self.camera.vel.y * dt,
            //     z: 0.,
            // });

            // checking for collision in y and fixing position
            // if next_pc.intersects(collider) {
            //     // calculate collided distance, set position to not colliding & delete velocity in y direction
            //     if dir.y < 0. {
            //         self.camera.pos.y +=
            //             (collider.max_y - next_pc.min_y) + self.camera.vel.y * dt;
            //         self.camera.vel.y = 0.;
            //     } else if dir.y > 0. {
            //         self.camera.pos.y +=
            //             (collider.min_y - next_pc.max_y) + self.camera.vel.y * dt;
            //         self.camera.vel.y = 0.;
            // if keys.contains(&Keycode::Space) && self.camera.vel.y >= 0. {
            //     self.camera.vel.y = -40.;
            //             }
            //         }
            //         current_pc = BoxCollider::new(PLAYER_COLLIDER.0, PLAYER_COLLIDER.1);
            //         current_pc.translate(self.camera.pos);
            //         continue;
            //     }
            // }

            self.camera.update_pos(dt);
        }
    }
}
