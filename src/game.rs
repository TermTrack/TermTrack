use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use device_query::{device_state, DeviceQuery, DeviceState, Keycode, MouseState};

use crate::loader::{self, load};
use crate::renderer::*;
use crate::{camera::Camera, mat::*};
use core::panic;
use std::any::Any;
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

        let mut pause = false;
        let mut show_map = false;

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

            let dt = time.elapsed().as_secs_f64();
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
            if keys.contains(&Keycode::W) {
                self.camera.pos = self.camera.pos
                    + Vec3 {
                        x: 0.,
                        y: 0.,
                        z: SPEED * dt,
                    }
                    .rotate_y(self.camera.rotation.x);
            }
            if keys.contains(&Keycode::A) {
                self.camera.pos = self.camera.pos
                    + Vec3 {
                        x: -SPEED * dt,
                        y: 0.,
                        z: 0.,
                    }
                    .rotate_y(self.camera.rotation.x);
            }
            if keys.contains(&Keycode::D) {
                self.camera.pos = self.camera.pos
                    + Vec3 {
                        x: SPEED * dt,
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
                        z: -SPEED * dt,
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

            // start for tests
            if false {
                if self.camera.pos.y >= 1.5 {
                    v.y = 0.;
                    self.camera.pos.y = 1.5;
                    if keys.contains(&Keycode::Space) {
                        self.camera.pos.y = 1.4;
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
            }

            if keys.contains(&Keycode::Space) {
                self.camera.pos.y -= SPEED * time.elapsed().as_secs_f64();
            }
            if keys.contains(&Keycode::LShift) {
                self.camera.pos.y += SPEED * time.elapsed().as_secs_f64();
            }
            // end for tests

            let fps = &format!(
                "\r\nfps: {:.2?}",
                1. / (time.elapsed().as_micros() as f64 / 1_000_000.)
            );

            // reset timer
            time = Instant::now();

            // render vertices
            self.renderer
                .render_mt(&self.camera, &mesh, &format!("{}", fps), true);
        }
    }
}
