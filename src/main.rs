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

fn menu(levels: Vec<PathBuf>) -> usize {
    let device_state = DeviceState::new();
    let mut chosen_level = 0;
    let level_names: Vec<&OsStr> = levels
        .iter()
        .map(|path| path.file_stem().unwrap())
        .collect();

    loop {
        // clear screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        // print background image
        print!("{esc}[48;2;105;105;105m", esc = 27 as char);
        for row in 0..=crossterm::terminal::size().unwrap().1 {
            print!(
                "{}",
                std::iter::repeat(" ")
                    .take(crossterm::terminal::size().unwrap().0 as usize)
                    .collect::<String>()
            )
        }

        // print menu
        let lowest = chosen_level.min(3);
        let highest = (chosen_level + 6 - lowest).min(level_names.len() - 1);
        for i in chosen_level - lowest..=highest {
            println!("{:?}", level_names[i]);
        }

        thread::sleep_ms(200);

        //match input
        loop {
            let keys = device_state.get_keys();

            if keys.contains(&Keycode::Down) {
                chosen_level += 1;
                chosen_level = chosen_level.min(level_names.len() - 1);
                break;
            }
            if keys.contains(&Keycode::Up) {
                chosen_level -= 1;
                chosen_level = chosen_level.max(0);
                break;
            }
            if keys.contains(&Keycode::Enter) {
                return chosen_level;
            }
        }
    }
}

fn main() {
    let level_dir = env::args().collect::<Vec<String>>()[1].clone();
    let entries = fs::read_dir(level_dir).unwrap();
    let levels: Vec<PathBuf> = entries.map(|e| e.unwrap().path()).collect();
    loop {
        let chosen_level = menu(levels.clone());
        let map = loader::load("");
    }

    let mut game = game::Game {
        renderer: Screen::new(),
        camera: camera::Camera {
            pos: Vec3 {
                x: load(loader::MAP).2 .1,
                y: -20.,
                z: load(loader::MAP).2 .0,
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
    game.run()
}
