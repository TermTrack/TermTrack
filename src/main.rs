use device_query::{self, DeviceQuery, DeviceState, Keycode};
use loader::*;
use mat::Vec3;
use renderer::Screen;
use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::thread;
use std::{fs, io};

mod camera;
mod game;
mod loader;
mod mat;
mod renderer;
mod screens;

fn main() {
    let level_dir = env::args().collect::<Vec<String>>()[1].clone();
    let entries = fs::read_dir(level_dir).unwrap();
    let levels: Vec<PathBuf> = entries.map(|e| e.unwrap().path()).collect();
    loop {
        let chosen_level = screens::menu(levels.clone());
        let map = loader::load(&levels[chosen_level]);

        let mut game = game::Game {
            renderer: Screen::new(),
            camera: camera::Camera {
                pos: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                focus_length: 2., //2
                rotation: Vec3 {
                    x: 1.75,
                    y: 0.0,
                    z: 0.0,
                },
                vel: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
            },
        };
        loop {
            match game.run(map.clone()) {
                Ok(time) => {
                    if screens::finish(time) {
                        continue;
                    }
                }
                Err(e) => match e {
                    "death" => {
                        if screens::game_over("You died!") {
                            continue;
                        }
                    }
                    _ => {
                        if screens::game_over("You failed!") {
                            continue;
                        }
                    }
                },
            }
            break; // break loop if not try again
        }
    }
}
