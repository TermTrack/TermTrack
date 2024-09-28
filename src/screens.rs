use std::io::{self, stdin, BufReader, Read};
use std::{ffi::OsStr, fs, path::PathBuf, thread};

use crossterm::{self, execute};
use device_query::{DeviceQuery, DeviceState, Keycode};
use rodio::OutputStreamHandle;
use rodio::{source::Source, Decoder, OutputStream};
use serde_json::{json, Value};

use crate::{audio, screens};

use crate::renderer;

const keys_keycode: [(Keycode, &str, &str); 37] = [
    (Keycode::A, "a", "A"),
    (Keycode::B, "b", "B"),
    (Keycode::C, "c", "C"),
    (Keycode::D, "d", "D"),
    (Keycode::E, "e", "E"),
    (Keycode::F, "f", "F"),
    (Keycode::G, "g", "G"),
    (Keycode::H, "h", "H"),
    (Keycode::I, "i", "I"),
    (Keycode::J, "j", "J"),
    (Keycode::K, "k", "K"),
    (Keycode::L, "l", "L"),
    (Keycode::M, "m", "M"),
    (Keycode::N, "n", "N"),
    (Keycode::O, "o", "O"),
    (Keycode::P, "p", "P"),
    (Keycode::Q, "q", "Q"),
    (Keycode::R, "r", "R"),
    (Keycode::S, "s", "S"),
    (Keycode::T, "t", "T"),
    (Keycode::U, "u", "U"),
    (Keycode::V, "v", "V"),
    (Keycode::W, "w", "W"),
    (Keycode::X, "x", "X"),
    (Keycode::Y, "y", "Y"),
    (Keycode::Z, "z", "Z"),
    (Keycode::Key1, "1", "!"),
    (Keycode::Key2, "2", "\""),
    (Keycode::Key3, "3", "#"),
    (Keycode::Key4, "4", "¤"),
    (Keycode::Key5, "5", "%"),
    (Keycode::Key6, "6", "&"),
    (Keycode::Key7, "7", "/"),
    (Keycode::Key8, "8", "("),
    (Keycode::Key9, "9", ")"),
    (Keycode::Key0, "0", "="),
    (Keycode::Space, " ", " "),
];

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

    let (_stream, background_audio_handle) = OutputStream::try_default().unwrap();
    audio::play_audio(&background_audio_handle, "./sounds/menu.mp3");

    loop {
        // print background image
        print!("{esc}[H{esc}[48;2;105;105;105m", esc = 27 as char);
        for row in 0..=screen_height {
            println!(
                "{}\r",
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
            if keys.contains(&Keycode::E) {
                screens::exit();
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
        print!("{esc}[H{esc}[48;2;105;105;105m", esc = 27 as char);
        for row in 0..=screen_height {
            println!(
                "{}\r",
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

pub fn finish(time: f64, level_name: &str) -> bool {
    // get device state for input
    let device_state = DeviceState::new();

    // get terminal size
    let (screen_width, screen_height) = renderer::get_terminal_size();
    let screen_width = screen_width as u16;
    let screen_height = screen_height as u16;

    // get the leaderboard
    let leader_boards = fs::read_to_string("./leaderboards.json").unwrap_or(String::from("{}"));
    let mut leader_boards: Value = serde_json::from_str(&leader_boards).unwrap_or(json!({}));
    let leader_board = match leader_boards.get_mut(&level_name) {
        Some(x) => x,
        None => {
            leader_boards[&level_name] = json!([]);
            leader_boards.get_mut(&level_name).unwrap()
        }
    };
    println!("{:?}", leader_board);
    let leader_vec = leader_board.as_array_mut().expect("leaderboard error");
    leader_vec.sort_by_key(|val| {
        (val.get("time")
            .expect("leaderboard format wrong")
            .as_f64()
            .expect("leaderboard format wrong")
            * 1000.) as usize
    });
    let mut name = String::new();

    let take = leader_vec.len().min(5);

    // set the size of the textbox
    let box_height = 9 + take as u16;
    let box_width = 35;

    let start_x = screen_width / 2 - box_width / 2;
    let start_y = screen_height / 2 - box_height / 2;
    let mut try_again = true;

    loop {
        // print background image
        print!("{esc}[H{esc}[48;2;105;105;105m", esc = 27 as char);
        for row in 0..=screen_height {
            println!(
                "{}\r",
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
            "{esc}[{};{}H|{:^3$}|",
            start_y + 3,
            start_x,
            "Choose Name",
            (box_width - 2) as usize,
            esc = 27 as char
        );
        println!(
            "{esc}[{};{}H|{:^3$}|",
            start_y + 4,
            start_x,
            &name,
            (box_width - 2) as usize,
            esc = 27 as char
        );
        println!(
            "{esc}[{};{}H|{:^3$}|",
            start_y + 5,
            start_x,
            "---leaders---",
            (box_width - 2) as usize,
            esc = 27 as char
        );
        for (i, result) in leader_vec.iter().take(take).enumerate() {
            println!(
                "{esc}[{};{}H| {:<3$} |",
                start_y + 6 + i as u16,
                start_x,
                format!(
                    "{}. {} - {:.2}s",
                    i + 1,
                    result.get("name").unwrap().as_str().unwrap(),
                    result.get("time").unwrap().as_f64().unwrap(),
                ),
                (box_width - 4) as usize,
                esc = 27 as char
            );
        }
        println!(
            "{esc}[{};{}H|{:^3$}|",
            start_y + 6 + take as u16,
            start_x,
            "-------------",
            (box_width - 2) as usize,
            esc = 27 as char
        );

        println!(
            "{esc}[{};{}H|{: ^3$}|",
            start_y + box_height - 2,
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
            start_y + box_height - 1,
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
            start_y + box_height,
            start_x,
            "NO",
            (box_width - 2) as usize,
            esc = 27 as char
        );

        print!("{esc}[48;2;0;0;0m", esc = 27 as char);
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
        'input_loop: loop {
            let keys = device_state.get_keys();

            for key in keys_keycode {
                if keys.contains(&key.0) {
                    if keys.contains(&Keycode::LShift) {
                        name += &key.2;
                    } else {
                        name += &key.1;
                    }
                    break 'input_loop;
                }
            }

            if keys.contains(&Keycode::Backspace) {
                name.pop();
                break;
            }

            if keys.contains(&Keycode::Down) {
                try_again = !try_again;
                break;
            }
            if keys.contains(&Keycode::Up) {
                try_again = !try_again;
                break;
            }
            if keys.contains(&Keycode::Enter) {
                if !name.is_empty() {
                    leader_vec.push(json!({"name": name, "time": time}));
                    fs::write("./leaderboards.json", leader_boards.to_string())
                        .expect("couldn't write json");
                }
                return try_again;
            }
        }
    }
}

pub fn exit() {
    let device_state = DeviceState::new();
    let (screen_width, screen_height) = renderer::get_terminal_size();
    let screen_width = screen_width as u16;
    let screen_height = screen_height as u16;

    let box_width: u16 = 30;
    let mut box_height = 5;

    let start_x = screen_width / 2 - box_width / 2;
    let start_y = screen_height / 2 - box_height / 2;
    let mut exit = true;

    loop {
        // print background image
        print!("{esc}[H{esc}[48;2;105;105;105m", esc = 27 as char);
        for row in 0..=screen_height {
            println!(
                "{}\r",
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
            "Are you sure you",
            (box_width - 2) as usize,
            esc = 27 as char
        );
        println!(
            "{esc}[{};{}H|{: ^3$}|",
            start_y + 2,
            start_x,
            "want to exit?",
            (box_width - 2) as usize,
            esc = 27 as char
        );
        if exit {
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
        if !exit {
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
                exit = !exit;
                break;
            }
            if keys.contains(&Keycode::Up) {
                exit = !exit;
                break;
            }
            if keys.contains(&Keycode::Enter) {
                if exit {
                    let _ = crossterm::terminal::disable_raw_mode();

                    println!("\x1b[2J\x1b[H\x1b[48;2;0;0;0mGame closing\r");
                    let _ = thread::spawn(|| {
                        for x in stdin().bytes() {
                            let _ = x;
                        }
                    });
                    thread::sleep_ms(100);
                    println!("\x1b[2J\x1b[H\x1b[48;2;0;0;0mGame Closed\r");

                    std::process::exit(0);
                } else {
                    return;
                }
            }
        }
    }
}
