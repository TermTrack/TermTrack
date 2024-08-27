use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};

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
        loop {
            // get list off all vertices
            let verts = vec![
                (0., 0., 0.),
                (0., 50., 0.),
                (30., 50., 30.),
                (255., 255., 255.),
            ];

            let mesh = Mesh::new(verts);

            self.renderer.render(&self.camera, mesh);
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
