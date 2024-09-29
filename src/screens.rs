use std::io::{stdin, Read};
use std::{ffi::OsStr, fs, path::PathBuf, thread};

use device_query::{DeviceQuery, DeviceState, Keycode};
use rodio::OutputStream;
use rodio::OutputStreamHandle;
use serde_json::{json, Value};

use crate::{audio, screens};

use crate::renderer;

const KEYS_KEYCODE: [(Keycode, &str, &str); 37] = [
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

const TITLE_L: &str = r#"                  ___           ___           ___                       ___           ___           ___           ___     
      ___        /  /\         /  /\         /__/\          ___        /  /\         /  /\         /  /\         /__/|    
     /  /\      /  /:/_       /  /::\       |  |::\        /  /\      /  /::\       /  /::\       /  /:/        |  |:|    
    /  /:/     /  /:/ /\     /  /:/\:\      |  |:|:\      /  /:/     /  /:/\:\     /  /:/\:\     /  /:/         |  |:|    
   /  /:/     /  /:/ /:/_   /  /:/~/:/    __|__|:|\:\    /  /:/     /  /:/~/:/    /  /:/~/::\   /  /:/  ___   __|  |:|    
  /  /::\    /__/:/ /:/ /\ /__/:/ /:/___ /__/::::| \:\  /  /::\    /__/:/ /:/___ /__/:/ /:/\:\ /__/:/  /  /\ /__/\_|:|____
 /__/:/\:\   \  \:\/:/ /:/ \  \:\/:::::/ \  \:\~~\__\/ /__/:/\:\   \  \:\/:::::/ \  \:\/:/__\/ \  \:\ /  /:/ \  \:\/:::::/
 \__\/  \:\   \  \::/ /:/   \  \::/~~~~   \  \:\       \__\/  \:\   \  \::/~~~~   \  \::/       \  \:\  /:/   \  \::/~~~~ 
      \  \:\   \  \:\/:/     \  \:\        \  \:\           \  \:\   \  \:\        \  \:\        \  \:\/:/     \  \:\     
       \__\/    \  \::/       \  \:\        \  \:\           \__\/    \  \:\        \  \:\        \  \::/       \  \:\    
                 \__\/         \__\/         \__\/                     \__\/         \__\/         \__\/         \__\/    "#; // min 140

const TITLE_S: &str = r#" _____ ______________  ______________  ___  _____  _   __
|_   _|  ___| ___ \  \/  |_   _| ___ \/ _ \/  __ \| | / /
  | | | |__ | |_/ / .  . | | | | |_/ / /_\ \ /  \/| |/ / 
  | | |  __||    /| |\/| | | | |    /|  _  | |    |    \ 
  | | | |___| |\ \| |  | | | | | |\ \| | | | \__/\| |\  \
  \_/ \____/\_| \_\_|  |_/ \_/ \_| \_\_| |_/\____/\_| \_/"#; // min 100

const TITLE_XS: &str = r#"___ ____ ____ _  _ ___ ____ ____ ____ _  _ 
 |  |___ |__/ |\/|  |  |__/ |__| |    |_/  
 |  |___ |  \ |  |  |  |  \ |  | |___ | \_ "#; // min 70

const TITLE_M: &str = r#" ________  ________  _______   __       __  ________  _______    ______    ______   __    __ 
/        |/        |/       \ /  \     /  |/        |/       \  /      \  /      \ /  |  /  |
$$$$$$$$/ $$$$$$$$/ $$$$$$$  |$$  \   /$$ |$$$$$$$$/ $$$$$$$  |/$$$$$$  |/$$$$$$  |$$ | /$$/ 
   $$ |   $$ |__    $$ |__$$ |$$$  \ /$$$ |   $$ |   $$ |__$$ |$$ |__$$ |$$ |  $$/ $$ |/$$/  
   $$ |   $$    |   $$    $$< $$$$  /$$$$ |   $$ |   $$    $$< $$    $$ |$$ |      $$  $$<   
   $$ |   $$$$$/    $$$$$$$  |$$ $$ $$/$$ |   $$ |   $$$$$$$  |$$$$$$$$ |$$ |   __ $$$$$  \  
   $$ |   $$ |_____ $$ |  $$ |$$ |$$$/ $$ |   $$ |   $$ |  $$ |$$ |  $$ |$$ \__/  |$$ |$$  \ 
   $$ |   $$       |$$ |  $$ |$$ | $/  $$ |   $$ |   $$ |  $$ |$$ |  $$ |$$    $$/ $$ | $$  |
   $$/    $$$$$$$$/ $$/   $$/ $$/      $$/    $$/    $$/   $$/ $$/   $$/  $$$$$$/  $$/   $$/ "#; // min 130

pub fn menu_print() {
    let (screen_width, screen_height) = renderer::get_terminal_size();
    let screen_width = screen_width as u16;
    let screen_height = screen_height as u16;
    let message = vec![
        "Use |\u{1F845} | and |\u{1F847} | to navigate menu.",
        "Press enter to play.\n",
        " ",
        "How to win:",
        "Find the end.",
    ];
    // print background image
    print!("{esc}[H{esc}[48;2;0;0;0m", esc = 27 as char);
    for _row in 0..=screen_height {
        println!("{}\r", " ".repeat(screen_width as usize),)
    }

    let (title, gap) = match screen_width {
        125..=u16::MAX => (TITLE_L, 2),
        95..125 => (TITLE_M, 2),
        59..95 => (TITLE_S, 1),
        45..59 => (TITLE_XS, 1),
        0..45 => ("", 0),
    };

    if title.is_empty() {
        panic!("please expand your terminal and try again.");
    };

    // print title
    let lines: Vec<&str> = title.split("\n").collect();
    let menu_width = lines[1].len() as u16;
    let mut y: u16 = 0;
    let x = screen_width / 2 - menu_width / 2;
    y += gap;
    for line in &lines {
        println!(
            "{esc}[{};{}H{}",
            y,
            screen_width / 2 - (line.len() as u16) / 2,
            line,
            esc = 27 as char
        );
        y += 1;
    }

    // print controls

    y += gap;
    println!(
        "{esc}[{};{}H{:-^3$}",
        y,
        screen_width / 2 - menu_width / 2,
        "",
        menu_width as usize,
        esc = 27 as char
    );
    y += 1;

    if title == TITLE_L {
        println!(
        "{esc}[{};{}H{:^3$}",
        y,
        screen_width / 2 - menu_width / 2,
        "|W| |A| |S| |D| - move   |\u{1F844} | |\u{1F845} | |\u{1F847} | |\u{1F846} | - rotate camera   | [SPACEBAR] | - jump   |M| - view map   |E| - exit",
        menu_width as usize,
        esc = 27 as char
    );
    } else if title == TITLE_M {
        println!(
        "{esc}[{};{}H{:^3$}",
        y,
        screen_width / 2 - menu_width / 2,
        "|W| |A| |S| |D| - move   |\u{1F844} | |\u{1F845} | |\u{1F847} | |\u{1F846} | - rotate camera   | [SPACEBAR] | - jump",
        menu_width as usize,
        esc = 27 as char
    );
        y += 1;
        println!(
            "{esc}[{};{}H{:^3$}",
            y,
            screen_width / 2 - menu_width / 2,
            "|M| - view map   |E| - exit",
            menu_width as usize,
            esc = 27 as char
        );
    } else if title == TITLE_S {
        println!(
            "{esc}[{};{}H{:^3$}",
            y,
            screen_width / 2 - menu_width / 2,
            "WASD - move | \u{1F844} \u{1F845} \u{1F847} \u{1F846}  - rotate camera | [SPACE] - jump",
            menu_width as usize,
            esc = 27 as char
        );
        y += 1;
        println!(
            "{esc}[{};{}H{:^3$}",
            y,
            screen_width / 2 - menu_width / 2,
            "M - map | E - exit",
            menu_width as usize,
            esc = 27 as char
        );
    } else if title == TITLE_XS {
        println!(
            "{esc}[{};{}H{:^3$}",
            y,
            screen_width / 2 - menu_width / 2,
            "WASD - move | \u{1F844} \u{1F845} \u{1F847} \u{1F846}  - rotate camera",
            menu_width as usize,
            esc = 27 as char
        );
        y += 1;
        println!(
            "{esc}[{};{}H{:^3$}",
            y,
            screen_width / 2 - menu_width / 2,
            "[SPACE] - jump | M - map | E - exit",
            menu_width as usize,
            esc = 27 as char
        );
    }

    y += 1;
    println!(
        "{esc}[{};{}H{:-^3$}",
        y,
        screen_width / 2 - menu_width / 2,
        "",
        menu_width as usize,
        esc = 27 as char
    );

    // print instructions
    y += gap;
    let mut chopped_message = vec![];
    for line in message {
        let mut chopped_line = line
            .chars()
            .collect::<Vec<char>>()
            .chunks(menu_width as usize / 2 - gap as usize)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();

        chopped_message.append(&mut chopped_line);
    }

    for (n, line) in chopped_message.iter().enumerate() {
        println!(
            "{esc}[{};{}H{}",
            y + n as u16,
            screen_width / 2 + gap,
            line,
            esc = 27 as char
        );
    }

    let box_width: u16 = menu_width / 2 - 2;
}

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
    let mut box_height = 7;
    if levels.len() < 7 {
        box_height = level_names.len() as u16;
    }
    let message = vec![
        "Use |\u{1F845} | and |\u{1F847} | to navigate menu.",
        "Press enter to play.\n",
        " ",
        "How to win:",
        "Find the end.",
    ];
    let (_stream, audio_handle) = OutputStream::try_default().unwrap();
    audio::audio_loop(&audio_handle, "./sounds/menu.mp3");

    // print background image
    print!("{esc}[H{esc}[48;2;0;0;0m", esc = 27 as char);
    for _row in 0..=screen_height {
        println!("{}\r", " ".repeat(screen_width as usize),)
    }

    // print title

    let (title, gap) = match screen_width {
        125..=u16::MAX => (TITLE_L, 2),
        95..125 => (TITLE_M, 2),
        59..95 => (TITLE_S, 1),
        45..59 => (TITLE_XS, 1),
        0..45 => ("", 0),
    };

    if title.is_empty() {
        panic!("please expand your terminal and try again.");
    };

    let lines: Vec<&str> = title.split("\n").collect();
    let menu_width = lines[1].len() as u16;
    let mut y: u16 = 0;
    let x = screen_width / 2 - menu_width / 2;
    y += gap;
    for line in &lines {
        println!(
            "{esc}[{};{}H{}",
            y,
            screen_width / 2 - (line.len() as u16) / 2,
            line,
            esc = 27 as char
        );
        y += 1;
    }

    // print controls

    y += gap;
    println!(
        "{esc}[{};{}H{:-^3$}",
        y,
        screen_width / 2 - menu_width / 2,
        "",
        menu_width as usize,
        esc = 27 as char
    );
    y += 1;

    if title == TITLE_L {
        println!(
        "{esc}[{};{}H{:^3$}",
        y,
        screen_width / 2 - menu_width / 2,
        "|W| |A| |S| |D| - move   |\u{1F844} | |\u{1F845} | |\u{1F847} | |\u{1F846} | - rotate camera   | [SPACEBAR] | - jump   |M| - view map   |E| - exit",
        menu_width as usize,
        esc = 27 as char
    );
    } else if title == TITLE_M {
        println!(
        "{esc}[{};{}H{:^3$}",
        y,
        screen_width / 2 - menu_width / 2,
        "|W| |A| |S| |D| - move   |\u{1F844} | |\u{1F845} | |\u{1F847} | |\u{1F846} | - rotate camera   | [SPACEBAR] | - jump",
        menu_width as usize,
        esc = 27 as char
    );
        y += 1;
        println!(
            "{esc}[{};{}H{:^3$}",
            y,
            screen_width / 2 - menu_width / 2,
            "|M| - view map   |E| - exit",
            menu_width as usize,
            esc = 27 as char
        );
    } else if title == TITLE_S {
        println!(
            "{esc}[{};{}H{:^3$}",
            y,
            screen_width / 2 - menu_width / 2,
            "WASD - move | \u{1F844} \u{1F845} \u{1F847} \u{1F846}  - rotate camera | [SPACE] - jump",
            menu_width as usize,
            esc = 27 as char
        );
        y += 1;
        println!(
            "{esc}[{};{}H{:^3$}",
            y,
            screen_width / 2 - menu_width / 2,
            "M - map | E - exit",
            menu_width as usize,
            esc = 27 as char
        );
    } else if title == TITLE_XS {
        println!(
            "{esc}[{};{}H{:^3$}",
            y,
            screen_width / 2 - menu_width / 2,
            "WASD - move | \u{1F844} \u{1F845} \u{1F847} \u{1F846}  - rotate camera",
            menu_width as usize,
            esc = 27 as char
        );
        y += 1;
        println!(
            "{esc}[{};{}H{:^3$}",
            y,
            screen_width / 2 - menu_width / 2,
            "[SPACE] - jump | M - map | E - exit",
            menu_width as usize,
            esc = 27 as char
        );
    }

    y += 1;
    println!(
        "{esc}[{};{}H{:-^3$}",
        y,
        screen_width / 2 - menu_width / 2,
        "",
        menu_width as usize,
        esc = 27 as char
    );

    // print instructions
    y += gap;
    let mut chopped_message = vec![];
    for line in message {
        let mut chopped_line = line
            .chars()
            .collect::<Vec<char>>()
            .chunks(menu_width as usize / 2 - gap as usize)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();

        chopped_message.append(&mut chopped_line);
    }

    for (n, line) in chopped_message.iter().enumerate() {
        println!(
            "{esc}[{};{}H{}",
            y + n as u16,
            screen_width / 2 + gap,
            line,
            esc = 27 as char
        );
    }

    let box_width: u16 = menu_width / 2 - 2;

    loop {
        // PRINT BOX
        // box upper line
        println!(
            "{esc}[{};{}H*{:-^3$}*",
            y,
            x,
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
                print!("{esc}[48;2;46;46;46m", esc = 27 as char);
            }

            let mut level_name = level_names[i as usize].to_str().unwrap().to_string();
            if level_name.len() > box_width as usize - 2 {
                level_name = level_name[0..(box_width as usize - 5)].to_string() + "...";
            }
            println!(
                "{esc}[{};{}H|{:^3$}|",
                y + 1 + (i - lowest),
                x,
                level_name,
                (box_width - 2) as usize,
                esc = 27 as char
            );
            print!("{esc}[48;2;0;0;0m", esc = 27 as char);
        }

        // box lower line
        println!(
            "{esc}[{};{}H*{:-^3$}*",
            y + box_height + 1,
            x,
            "",
            (box_width - 2) as usize,
            esc = 27 as char
        );

        thread::sleep_ms(200);

        //match input
        loop {
            let keys = device_state.get_keys();

            if keys.contains(&Keycode::Down) && chosen_level != level_names.len() as u16 - 1 {
                chosen_level += 1;
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }
            if keys.contains(&Keycode::Up) && chosen_level != 0 {
                chosen_level = chosen_level.saturating_sub(1);
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }
            if keys.contains(&Keycode::Enter) {
                // audio::play_audio(&audio_handle, "./sounds/enter.mp3");
                // thread::sleep_ms(800);

                return chosen_level as usize;
            }
            if keys.contains(&Keycode::E) {
                screens::exit();
                menu_print();
                break;
            }
        }
    }
}

pub fn game_over(arg: &str) -> bool {
    let device_state = DeviceState::new();
    let (screen_width, screen_height) = renderer::get_terminal_size();
    let screen_width = screen_width as u16;
    let screen_height = screen_height as u16;
    let (_stream, audio_handle) = OutputStream::try_default().unwrap();

    let box_width: u16 = 30.max(arg.len() as u16);
    let box_height = 5;

    let start_x = screen_width / 2 - box_width / 2;
    let start_y = screen_height / 2 - box_height / 2;
    let mut try_again = true;

    loop {
        // print background image
        print!("{esc}[H{esc}[48;2;0;0;0m", esc = 27 as char);
        for _row in 0..=screen_height {
            println!("{}\r", " ".repeat(screen_width as usize),)
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
            print!("{esc}[48;2;46;46;46m", esc = 27 as char);
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
            print!("{esc}[48;2;46;46;46m", esc = 27 as char);
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
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }
            if keys.contains(&Keycode::Up) {
                try_again = !try_again;
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }
            if keys.contains(&Keycode::Enter) {
                return try_again;
            }
            if keys.contains(&Keycode::E) {
                exit()
            }
        }
    }
}

pub fn finish(time: f64, level_name: &str) -> u8 {
    // get device state for input
    let device_state = DeviceState::new();

    // get terminal size
    let (screen_width, screen_height) = renderer::get_terminal_size();
    let screen_width = screen_width as u16;
    let screen_height = screen_height as u16;

    //for audio
    let (_stream, audio_handle) = OutputStream::try_default().unwrap();

    // get the leaderboard
    let leader_boards = fs::read_to_string("./leaderboards.json").unwrap_or(String::from("{}"));
    let mut leader_boards: Value = serde_json::from_str(&leader_boards).unwrap_or(json!({}));
    let leader_board = match leader_boards.get_mut(level_name) {
        Some(x) => x,
        None => {
            leader_boards[&level_name] = json!([]);
            leader_boards.get_mut(level_name).unwrap()
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

    let mut chosen = 0;

    loop {
        // print background image
        print!("{esc}[H{esc}[48;2;0;0;0m", esc = 27 as char);
        for _row in 0..=screen_height {
            println!("{}\r", " ".repeat(screen_width as usize),)
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
            "You won!",
            (box_width - 2) as usize,
            esc = 27 as char
        );
        println!(
            "{esc}[{};{}H|{:^3$}|",
            start_y + 2,
            start_x,
            format!("Time: {:.2}s", time),
            (box_width - 2) as usize,
            esc = 27 as char
        );
        println!(
            "{esc}[{};{}H|{:^3$}|",
            start_y + 3,
            start_x,
            "Choose name to save result:",
            (box_width - 2) as usize,
            esc = 27 as char
        );
        if chosen == 0 {
            print!("{esc}[48;2;46;46;46m", esc = 27 as char);
        }
        println!(
            "{esc}[{};{}H|{:^3$}|",
            start_y + 4,
            start_x,
            &name,
            (box_width - 2) as usize,
            esc = 27 as char
        );
        print!("{esc}[48;2;0;0;0m", esc = 27 as char);
        println!(
            "{esc}[{};{}H|{:^3$}|",
            start_y + 5,
            start_x,
            "---LEADERS---",
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
            "Try again?",
            (box_width - 2) as usize,
            esc = 27 as char
        );
        if chosen == 1 {
            print!("{esc}[48;2;46;46;46m", esc = 27 as char);
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
        if chosen == 2 {
            print!("{esc}[48;2;46;46;46m", esc = 27 as char);
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

            for key in KEYS_KEYCODE {
                if keys.contains(&key.0) && chosen == 0 {
                    if keys.contains(&Keycode::LShift) {
                        name += key.2;
                    } else {
                        name += key.1;
                    }
                    break 'input_loop;
                }
            }

            if keys.contains(&Keycode::Backspace) && chosen == 0 {
                name.pop();
                break;
            }

            if keys.contains(&Keycode::Down) && chosen != 2 {
                chosen += 1;
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }
            if keys.contains(&Keycode::Up) && chosen != 0 {
                chosen -= 1;
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }
            if keys.contains(&Keycode::Enter) && chosen != 0 {
                if !name.is_empty() {
                    leader_vec.push(json!({"name": name, "time": time}));
                    fs::write("./leaderboards.json", leader_boards.to_string())
                        .expect("couldn't write json");
                }
                return chosen;
            }
            if keys.contains(&Keycode::Enter) && chosen == 0 {
                chosen += 1;
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }
            if keys.contains(&Keycode::E) && chosen != 0 {
                exit();
                break;
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
    let box_height = 5;

    let (_stream, audio_handle) = OutputStream::try_default().unwrap();

    let start_x = screen_width / 2 - box_width / 2;
    let start_y = screen_height / 2 - box_height / 2;
    let mut exit = true;

    loop {
        // print background image
        print!("{esc}[H{esc}[48;2;0;0;0m", esc = 27 as char);
        for _row in 0..=screen_height {
            println!("{}\r", " ".repeat(screen_width as usize),)
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
            print!("{esc}[48;2;46;46;46m", esc = 27 as char);
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
            print!("{esc}[48;2;46;46;46m", esc = 27 as char);
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

            if keys.contains(&Keycode::Down) || keys.contains(&Keycode::Up) {
                exit = !exit;
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
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
                    thread::sleep_ms(100);
                    return;
                }
            }
        }
    }
}
