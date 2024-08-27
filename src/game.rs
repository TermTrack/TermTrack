use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};

use crate::loader::{self, load};
use crate::renderer::Screen;
use crate::{camera::Camera, mat::*};
use core::panic;
use std::time::{Duration, Instant};

pub struct Game {
    pub renderer: Screen,
    pub camera: Camera,
}

impl Game {
    pub fn run(&mut self) {
        // load map files
        // generate map meshes
        let frame_duration = Duration::from_millis(10);
        let mut start = Instant::now();

        let mesh = load(loader::MAP);

        loop {
            // get list off all vertices

            if Instant::now() - start >= frame_duration {
                self.renderer.render(&self.camera, &mesh);
                start = Instant::now();
            }
            // render vertices
            // handle input

            if poll(Duration::from_millis(5)).unwrap() {
                match read().unwrap() {
                    Event::Key(event) => match event {
                        KeyEvent {
                            code,
                            modifiers: _,
                            kind: _,
                            state: _,
                        } => match code {
                            KeyCode::Char('e') => {
                                let _ = crossterm::terminal::disable_raw_mode().unwrap();
                                panic!("Exited app")
                            }
                            KeyCode::Right => {
                                self.camera.pos.x += 1.;
                            }
                            KeyCode::Left => {
                                self.camera.pos.x -= 1.;
                            }
                            KeyCode::Up => {
                                self.camera.pos.z += 1.;
                            }
                            KeyCode::Down => {
                                self.camera.pos.z -= 1.;
                            }
                            _ => (),
                        },
                    },
                    _ => (),
                }
            }

            // perform game logic
        }
    }
}
