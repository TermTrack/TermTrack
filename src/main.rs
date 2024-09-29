use crossterm::{self};
use device_query::{self};
use loader::*;
use mat::Vec3;
use renderer::Screen;
use rodio::OutputStream;
use std::env;
use std::path::PathBuf;
use std::fs;

mod audio;
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
    let (_stream, stream_handle) = OutputStream::try_default().expect("couldnt get sound handle!");
    crossterm::terminal::enable_raw_mode().unwrap();
    loop {
        let chosen_level = screens::menu(levels.clone(), &stream_handle);
        let map = loader::load(&levels[chosen_level]);

        let mut game = game::Game {
            renderer: Screen::new(),
            camera: camera::Camera {
                pos: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                focus_length: 1.5, //2
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
            match game.run(map.clone(), &stream_handle) {
                Ok(time) => {
                    if screens::finish(time, &map.level_name) == 1 {
                        continue;
                    }
                }
                Err(e) => match e {
                    "void" => {
                        if screens::game_over("You fell into the void!") {
                            continue;
                        }
                    }
                    "angry_pixel" => {
                        if screens::game_over("Angry pixel killed you!") {
                            continue;
                        }
                    }
                    "spike" => {
                        if screens::game_over("You died of spike!") {
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
