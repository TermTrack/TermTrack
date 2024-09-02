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

impl Game {
    pub fn run(&mut self) {
        // load map files
        // generate map meshes

        let mesh = load(loader::MAP);

        // timer for fps
        let mut time = Instant::now();

        // device for input
        let device_state = DeviceState::new();

        loop {
            // get list off all vertices

            let fps = &format!(
                "\r\nfps: {:.2?}",
                1. / (time.elapsed().as_micros() as f64 / 1_000_000.)
            );
            time = Instant::now();
            // render vertices
            self.renderer
                .render_mt(&self.camera, &mesh, &format!("{}", fps), true);

            // reset timer

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
                self.camera.rotation.y += 0.05;
            }
            if keys.contains(&Keycode::Down) {
                self.camera.rotation.y -= 0.05;
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
