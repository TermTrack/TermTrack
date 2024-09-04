use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};

use crate::loader::{self, load};
use crate::renderer::Screen;
use crate::{camera::Camera, mat::*};
use core::panic;
use std::any::Any;
use std::time::{Duration, Instant};

pub struct Game {
    pub renderer: Screen,
    pub camera: Camera,
}

const SPEED: f64 = 2.;
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

        // velocity vector
        let mut v = Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };

        loop {
            // gravity
            // if self.camera.pos.y < -0.5 {
            //     // change this statement to depend on collision instead
            //     self.camera.pos = self.camera.pos
            //         + Vec3 {
            //             x: 0.,
            //             y: v.y * time.elapsed().as_secs_f64()
            //                 + (GRAVITY * time.elapsed().as_secs_f64().powi(2)) / 2.,
            //             z: 0.,
            //         };
            //     v.y += GRAVITY * time.elapsed().as_secs_f64();
            // } else {
            //     v.y = 0.;
            // }

            // get list off all vertices

            let fps = &format!(
                "\r\nfps: {:.2?}",
                1. / (time.elapsed().as_micros() as f64 / 1_000_000.)
            );

            // reset timer
            time = Instant::now();

            // render vertices
            self.renderer
                .render_mt(&self.camera, &mesh, &format!("{}", fps), true);

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
            if keys.contains(&Keycode::W) {
                self.camera.pos = self.camera.pos
                    + Vec3 {
                        x: 0.,
                        y: 0.,
                        z: SPEED,
                    }
                    .rotate_y(self.camera.rotation.x);
            }
            if keys.contains(&Keycode::A) {
                self.camera.pos = self.camera.pos
                    + Vec3 {
                        x: -SPEED,
                        y: 0.,
                        z: 0.,
                    }
                    .rotate_y(self.camera.rotation.x);
            }
            if keys.contains(&Keycode::D) {
                self.camera.pos = self.camera.pos
                    + Vec3 {
                        x: SPEED,
                        y: 0.,
                        z: 0.,
                    }
                    .rotate_y(self.camera.rotation.x);
            }
            if keys.contains(&Keycode::S) {
                self.camera.pos = self.camera.pos
                    + Vec3 {
                        x: 0.,
                        y: 0.,
                        z: -SPEED,
                    }
                    .rotate_y(self.camera.rotation.x);
            }
            // if keys.contains(&Keycode::Space) {
            //     v = v + Vec3 {
            //         x: 0.,
            //         y: -20.,
            //         z: 0.,
            //     }
            // }

            // jump, gravity & collision

            // replace with collision statement
            if self.camera.pos.y > -0.5 {
                v.y = 0.;
                if keys.contains(&Keycode::Space) {
                    self.camera.pos.y = -0.6;
                    v = v + Vec3 {
                        x: 0.,
                        y: -40.,
                        z: 0.,
                    }
                }
            } else {
                self.camera.pos = self.camera.pos
                    + Vec3 {
                        x: 0.,
                        y: v.y * time.elapsed().as_secs_f64()
                            + (GRAVITY * time.elapsed().as_secs_f64().powi(2)) / 2.,
                        z: 0.,
                    };
                v.y += GRAVITY * time.elapsed().as_secs_f64();
            }

            // if poll(Duration::from_millis(5)).unwrap() {
            //     match read().unwrap() {
            //         Event::Key(event) => match event {
            //             KeyEvent {
            //                 code,
            //                 modifiers: _,
            //                 kind: _,
            //                 state: _,
            //             } => match code {
            //                 KeyCode::Char('e') => {
            //                     let _ = crossterm::terminal::disable_raw_mode().unwrap();
            //                     panic!("Exited app")
            //                 }
            //                 KeyCode::Right => {
            //                     self.camera.pos = self.camera.pos
            //                         + Vec3 {
            //                             x: SPEED,
            //                             y: 0.,
            //                             z: 0.,
            //                         }
            //                         .rotate_y(self.camera.rotation.x);
            //                 }
            //                 KeyCode::Left => {
            //                     self.camera.pos = self.camera.pos
            //                         - Vec3 {
            //                             x: SPEED,
            //                             y: 0.,
            //                             z: 0.,
            //                         }
            //                         .rotate_y(self.camera.rotation.x);
            //                 }
            //                 KeyCode::Up => {
            //                     self.camera.pos = self.camera.pos
            //                         + Vec3 {
            //                             x: 0.,
            //                             y: 0.,
            //                             z: SPEED,
            //                         }
            //                         .rotate_y(self.camera.rotation.x);
            //                 }
            //                 KeyCode::Down => {
            //                     self.camera.pos = self.camera.pos
            //                         - Vec3 {
            //                             x: 0.,
            //                             y: 0.,
            //                             z: SPEED,
            //                         }
            //                         .rotate_y(self.camera.rotation.x);
            //                 }
            //                 KeyCode::Char('w') => {
            //                     self.camera.rotation.y += 0.05;
            //                 }
            //                 KeyCode::Char('a') => {
            //                     self.camera.rotation.x -= 0.05;
            //                 }
            //                 KeyCode::Char('s') => {
            //                     self.camera.rotation.y -= 0.05;
            //                 }
            //                 KeyCode::Char('d') => {
            //                     self.camera.rotation.x += 0.05;
            //                 }
            //                 _ => (),
            //             },
            //         },
            //         _ => (),
            //     }
            // }

            // perform game logic
        }
    }
}
