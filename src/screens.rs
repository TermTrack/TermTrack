use std::{ffi::OsStr, path::PathBuf, thread};

use crossterm;
use device_query::{DeviceQuery, DeviceState, Keycode};
use rodio::OutputStreamHandle;
use rodio::{source::Source, Decoder, OutputStream};

use crate::audio;

use crate::renderer;

const title_l: &str = r#"                   ___           ___           ___                       ___           ___           ___           ___     
      ___        /  /\         /  /\         /__/\          ___        /  /\         /  /\         /  /\         /__/|    
     /  /\      /  /:/_       /  /::\       |  |::\        /  /\      /  /::\       /  /::\       /  /:/        |  |:|    
    /  /:/     /  /:/ /\     /  /:/\:\      |  |:|:\      /  /:/     /  /:/\:\     /  /:/\:\     /  /:/         |  |:|    
   /  /:/     /  /:/ /:/_   /  /:/~/:/    __|__|:|\:\    /  /:/     /  /:/~/:/    /  /:/~/::\   /  /:/  ___   __|  |:|    
  /  /::\    /__/:/ /:/ /\ /__/:/ /:/___ /__/::::| \:\  /  /::\    /__/:/ /:/___ /__/:/ /:/\:\ /__/:/  /  /\ /__/\_|:|____
 /__/:/\:\   \  \:\/:/ /:/ \  \:\/:::::/ \  \:\~~\__\/ /__/:/\:\   \  \:\/:::::/ \  \:\/:/__\/ \  \:\ /  /:/ \  \:\/:::::/
 \__\/  \:\   \  \::/ /:/   \  \::/~~~~   \  \:\       \__\/  \:\   \  \::/~~~~   \  \::/       \  \:\  /:/   \  \::/~~~~ 
      \  \:\   \  \:\/:/     \  \:\        \  \:\           \  \:\   \  \:\        \  \:\        \  \:\/:/     \  \:\     
       \__\/    \  \::/       \  \:\        \  \:\           \__\/    \  \:\        \  \:\        \  \::/       \  \:\    
                 \__\/         \__\/         \__\/                     \__\/         \__\/         \__\/         \__\/    "#;

const title_s: &str = r#" _____ ______________  ______________  ___  _____  _   __
|_   _|  ___| ___ \  \/  |_   _| ___ \/ _ \/  __ \| | / /
  | | | |__ | |_/ / .  . | | | | |_/ / /_\ \ /  \/| |/ / 
  | | |  __||    /| |\/| | | | |    /|  _  | |    |    \ 
  | | | |___| |\ \| |  | | | | | |\ \| | | | \__/\| |\  \
  \_/ \____/\_| \_\_|  |_/ \_/ \_| \_\_| |_/\____/\_| \_/"#;

const title_xs: &str = r#"___ ____ ____ _  _ ___ ____ ____ ____ _  _ 
 |  |___ |__/ |\/|  |  |__/ |__| |    |_/  
 |  |___ |  \ |  |  |  |  \ |  | |___ | \_ "#;

const title_m: &str = r#" ________  ________  _______   __       __  ________  _______    ______    ______   __    __ 
/        |/        |/       \ /  \     /  |/        |/       \  /      \  /      \ /  |  /  |
$$$$$$$$/ $$$$$$$$/ $$$$$$$  |$$  \   /$$ |$$$$$$$$/ $$$$$$$  |/$$$$$$  |/$$$$$$  |$$ | /$$/ 
   $$ |   $$ |__    $$ |__$$ |$$$  \ /$$$ |   $$ |   $$ |__$$ |$$ |__$$ |$$ |  $$/ $$ |/$$/  
   $$ |   $$    |   $$    $$< $$$$  /$$$$ |   $$ |   $$    $$< $$    $$ |$$ |      $$  $$<   
   $$ |   $$$$$/    $$$$$$$  |$$ $$ $$/$$ |   $$ |   $$$$$$$  |$$$$$$$$ |$$ |   __ $$$$$  \  
   $$ |   $$ |_____ $$ |  $$ |$$ |$$$/ $$ |   $$ |   $$ |  $$ |$$ |  $$ |$$ \__/  |$$ |$$  \ 
   $$ |   $$       |$$ |  $$ |$$ | $/  $$ |   $$ |   $$ |  $$ |$$ |  $$ |$$    $$/ $$ | $$  |
   $$/    $$$$$$$$/ $$/   $$/ $$/      $$/    $$/    $$/   $$/ $$/   $$/  $$$$$$/  $$/   $$/ "#;

pub fn menu(levels: Vec<PathBuf>, audio_handle: &OutputStreamHandle) -> usize {
    let device_state = DeviceState::new();
    let mut chosen_level = 0;
    let level_names: Vec<&OsStr> = levels
        .iter()
        .map(|path| path.file_stem().unwrap())
        .collect();
    let (screen_width, screen_height) = renderer::get_terminal_size();
    let screen_width = screen_width as u16;
    let screen_height = screen_height as u16;
    let box_width: u16 = 30;
    let mut box_height = 7;
    if levels.len() < 7 {
        box_height = level_names.len() as u16;
    }

    let start_x = screen_width / 2 - box_width / 2;
    let start_y = screen_height / 2 - box_height / 2;

    let (_stream, audio_handle) = OutputStream::try_default().unwrap();
    audio::play_audio(&audio_handle, "./sounds/menu.mp3");

    loop {
        // print background image
        print!("{esc}[H{esc}[48;2;0;0;0m", esc = 27 as char);
        for row in 0..=screen_height {
            print!(
                "{}",
                std::iter::repeat(" ")
                    .take(screen_width as usize)
                    .collect::<String>()
            )
        }

        // print menu

        // box upper line
        println!(
            "{esc}[{};{}H*{:-^3$}*",
            start_y,
            start_x,
            "",
            (box_width - 2) as usize,
            esc = 27 as char
        );

        // box content
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

        // box lower line
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
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }
            if keys.contains(&Keycode::Up) {
                chosen_level = chosen_level.checked_sub(1).unwrap_or(0);
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }
            if keys.contains(&Keycode::Enter) {
                return chosen_level as usize;
            }
            if keys.contains(&Keycode::E) {
                panic!("exited app");
            }
        }
    }
}

pub fn game_over(arg: &str) -> bool {
    let device_state = DeviceState::new();
    let (screen_width, screen_height) = renderer::get_terminal_size();
    let screen_width = screen_width as u16;
    let screen_height = screen_height as u16;

    let box_width: u16 = 30.max(arg.len() as u16);
    let mut box_height = 5;

    let start_x = screen_width / 2 - box_width / 2;
    let start_y = screen_height / 2 - box_height / 2;
    let mut try_again = true;

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
        println!(
            "{esc}[{};{}H|{: ^3$}|",
            start_y + 1,
            start_x,
            arg,
            (box_width - 2) as usize,
            esc = 27 as char
        );
        println!(
            "{esc}[{};{}H|{: ^3$}|",
            start_y + 2,
            start_x,
            "try again?",
            (box_width - 2) as usize,
            esc = 27 as char
        );
        if try_again {
            print!("{esc}[48;2;255;0;0m", esc = 27 as char);
        }
        println!(
            "{esc}[{};{}H|{: ^3$}|",
            start_y + 3,
            start_x,
            "YES",
            (box_width - 2) as usize,
            esc = 27 as char
        );
        print!("{esc}[48;2;0;0;0m", esc = 27 as char);
        if !try_again {
            print!("{esc}[48;2;255;0;0m", esc = 27 as char);
        }
        println!(
            "{esc}[{};{}H|{: ^3$}|",
            start_y + 4,
            start_x,
            "NO",
            (box_width - 2) as usize,
            esc = 27 as char
        );

        print!("{esc}[48;2;0;0;0m", esc = 27 as char);
        println!(
            "{esc}[{};{}H*{:-^3$}*",
            start_y + 5,
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
                try_again = !try_again;
                break;
            }
            if keys.contains(&Keycode::Up) {
                try_again = !try_again;
                break;
            }
            if keys.contains(&Keycode::Enter) {
                return try_again;
            }
        }
    }
}

pub fn finish(time: f64) -> bool {
    let device_state = DeviceState::new();
    let (screen_width, screen_height) = renderer::get_terminal_size();
    let screen_width = screen_width as u16;
    let screen_height = screen_height as u16;
    let mut box_height = 5;
    let box_width = 30;

    let start_x = screen_width / 2 - box_width / 2;
    let start_y = screen_height / 2 - box_height / 2;
    let mut try_again = true;

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
        println!(
            "{esc}[{};{}H|{:^3$}|",
            start_y + 1,
            start_x,
            "You Won!",
            (box_width - 2) as usize,
            esc = 27 as char
        );
        println!(
            "{esc}[{};{}H|{:^3$}|",
            start_y + 2,
            start_x,
            format!("time: {:.2}s", time),
            (box_width - 2) as usize,
            esc = 27 as char
        );
        println!(
            "{esc}[{};{}H|{: ^3$}|",
            start_y + 3,
            start_x,
            "try again?",
            (box_width - 2) as usize,
            esc = 27 as char
        );
        if try_again {
            print!("{esc}[48;2;255;0;0m", esc = 27 as char);
        }
        println!(
            "{esc}[{};{}H|{: ^3$}|",
            start_y + 4,
            start_x,
            "YES",
            (box_width - 2) as usize,
            esc = 27 as char
        );
        print!("{esc}[48;2;0;0;0m", esc = 27 as char);
        if !try_again {
            print!("{esc}[48;2;255;0;0m", esc = 27 as char);
        }
        println!(
            "{esc}[{};{}H|{: ^3$}|",
            start_y + 5,
            start_x,
            "NO",
            (box_width - 2) as usize,
            esc = 27 as char
        );

        print!("{esc}[48;2;0;0;0m", esc = 27 as char);
        println!(
            "{esc}[{};{}H*{:-^3$}*",
            start_y + 6,
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
                try_again = !try_again;
                break;
            }
            if keys.contains(&Keycode::Up) {
                try_again = !try_again;
                break;
            }
            if keys.contains(&Keycode::Enter) {
                return try_again;
            }
        }
    }
}
