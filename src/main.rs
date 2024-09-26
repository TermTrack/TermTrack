use device_query::{self, DeviceQuery, DeviceState, Keycode};
use loader::*;
use mat::Vec3;
use renderer::Screen;
use rodio::{source::Source, Decoder, OutputStream};
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::thread;
use std::{fs, io};

mod audio;
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
    let (screen_width, screen_height) = crossterm::terminal::size().unwrap();
    let box_width: u16 = 30;
    let mut box_height = 7;
    if levels.len() < 7 {
        box_height = level_names.len() as u16;
    }

    let start_x = screen_width / 2 - box_width / 2;
    let start_y = screen_height / 2 - box_height / 2;

    loop {
        // print background image
        print!("{esc}[48;2;105;105;105m", esc = 27 as char);
        for row in 0..=screen_height {
            print!(
                "{}",
                std::iter::repeat(" ")
                    .take(screen_width as usize)
                    .collect::<String>()
            )
        }

        // print menu
        print!("{esc}[48;2;0;0;0m", esc = 27 as char);
        println!(
            "{esc}[{};{}H*{:-^3$}*",
            start_y,
            start_x,
            "",
            (box_width - 2) as usize,
            esc = 27 as char
        );
        let mut lowest = 0;
        let mut highest = 0;
        if chosen_level < box_height.div_ceil(2) {
            highest = box_height;
        } else if level_names.len() as u16 - 1 - chosen_level < box_height.div_euclid(2) {
            highest = level_names.len() as u16;
            lowest = highest - box_height;
        } else {
            highest = chosen_level + box_height.div_euclid(2) + 1;
            lowest = chosen_level - box_height.div_euclid(2);
        }

        for i in lowest..highest {
            if i == chosen_level {
                print!("{esc}[48;2;255;0;0m", esc = 27 as char);
            }
            println!(
                "{esc}[{};{}H|{:^3$}|",
                start_y + 1 + (i - lowest) as u16,
                start_x,
                level_names[i as usize].to_str().unwrap(),
                (box_width - 2) as usize,
                esc = 27 as char
            );
            print!("{esc}[48;2;0;0;0m", esc = 27 as char);
        }
        println!(
            "{esc}[{};{}H*{:-^3$}*",
            start_y + box_height + 1,
            start_x,
            "",
            (box_width - 2) as usize,
            esc = 27 as char
        );

        thread::sleep_ms(200);

        //match input
        loop {
            let keys = device_state.get_keys();

            if keys.contains(&Keycode::Down) {
                chosen_level += 1;
                chosen_level = chosen_level.min(level_names.len() as u16 - 1);
                break;
            }
            if keys.contains(&Keycode::Up) {
                chosen_level = chosen_level.checked_sub(1).unwrap_or(0);
                break;
            }
            if keys.contains(&Keycode::Enter) {
                return chosen_level as usize;
            }
        }
    }
}

fn main() {
    let level_dir = env::args().collect::<Vec<String>>()[1].clone();
    let entries = fs::read_dir(level_dir).unwrap();
    let levels: Vec<PathBuf> = entries.map(|e| e.unwrap().path()).collect();
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    loop {
        let chosen_level = menu(levels.clone());
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
        match game.run(map, &stream_handle) {
            Ok(time) => finish(time),
            Err(e) => match e {
                "death" => game_over("You died! try again"),
                _ => game_over("you failed!"),
            },
        }
    }
}

fn game_over(arg: &str) {
    todo!()
}

fn finish(time: f64) {
    todo!()
}
