use crossterm::event::{poll, read, Event};
use crossterm::{self};
use device_query::{self};
use loader::*;
use mat::Vec3;
use renderer::Screen;
use rodio::OutputStream;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

mod audio;
mod camera;
mod enemies;
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
    let focused = Arc::new(Mutex::new(true));
    let focused_clone = Arc::clone(&focused);

    let _focus_thread = thread::spawn(move || loop {
        match read().expect("couldn't read event's") {
            Event::FocusLost => *focused_clone.lock().unwrap() = false,
            Event::FocusGained => *focused_clone.lock().unwrap() = true,
            _ => (),
        }
    });

    loop {
        let chosen_level = screens::menu(levels.clone(), &stream_handle, focused.clone());
        let map = loader::load(&levels[chosen_level]);

        loop {
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
            match game.run(map.clone(), &stream_handle, focused.clone()) {
                Ok(time) => {
                    if screens::finish(time, &map.level_name, &map.map_string, focused.clone()) == 1
                    {
                        continue;
                    }
                }
                Err(e) => match e {
                    "void" => {
                        if screens::game_over("You fell into the void!", focused.clone()) {
                            continue;
                        }
                    }
                    "angry_pixel" => {
                        if screens::game_over("Angry pixel killed you!", focused.clone()) {
                            continue;
                        }
                    }
                    "spike" => {
                        if screens::game_over("You died of spike!", focused.clone()) {
                            continue;
                        }
                    }
                    "menu" => (),
                    "retry" => continue,
                    _ => {
                        if screens::game_over("You failed!", focused.clone()) {
                            continue;
                        }
                    }
                },
            }
            break; // break loop if not try again
        }
    }
}
