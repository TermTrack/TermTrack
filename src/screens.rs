use std::io::{stdin, stdout, Read};
use std::sync::{Arc, Mutex};
use std::{ffi::OsStr, fs, path::PathBuf, thread};

use crossterm::event::PopKeyboardEnhancementFlags;
use crossterm::execute;
use rodio::OutputStream;
use rodio::OutputStreamHandle;
use serde_json::{json, Value};

use crate::{audio, mat, network, screens, Keys};

use crate::renderer;

const KEYS_KEYCODE: [(char, char); 37] = [
    ('a', 'A'),
    ('b', 'B'),
    ('c', 'C'),
    ('d', 'D'),
    ('e', 'E'),
    ('f', 'F'),
    ('g', 'G'),
    ('h', 'H'),
    ('i', 'I'),
    ('j', 'J'),
    ('k', 'K'),
    ('l', 'L'),
    ('m', 'M'),
    ('n', 'N'),
    ('o', 'O'),
    ('p', 'P'),
    ('q', 'Q'),
    ('r', 'R'),
    ('s', 'S'),
    ('t', 'T'),
    ('u', 'U'),
    ('v', 'V'),
    ('w', 'W'),
    ('x', 'X'),
    ('y', 'Y'),
    ('z', 'Z'),
    ('1', '!'),
    ('2', '\"'),
    ('3', '#'),
    ('4', '¤'),
    ('5', '%'),
    ('6', '&'),
    ('7', '/'),
    ('8', '('),
    ('9', ')'),
    ('0', '='),
    (' ', ' '),
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
        " ",
        "Press |L| to show leaderboard for chosen level",
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

pub fn menu(
    levels: Vec<PathBuf>,
    audio_handle: &OutputStreamHandle,
    focused: Arc<Mutex<bool>>,
    keys: Arc<Mutex<Keys>>
) -> usize {
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
        " ",
        "Press |L| to show leaderboard for chosen level",
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

        //chosen level id
        let level_map = fs::read(&levels[chosen_level as usize]).unwrap();

        let level_id = level_names[chosen_level as usize]
            .to_str()
            .unwrap()
            .to_string()
            + &crc32fast::hash(&level_map).to_string();

        // println!(
        //     "{esc}[{};{}Hid: {}",
        //     y + box_height + 2,
        //     x,
        //     level_id,
        //     esc = 27 as char
        // );

        thread::sleep_ms(200);

        //match input
        loop {
            let mut key_list = if *focused.lock().unwrap() {keys.lock().unwrap().clone_reset_enter()} else {Keys::default()};

            if key_list.down && chosen_level != level_names.len() as u16 - 1 {
                chosen_level += 1;
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }
            if key_list.up && chosen_level != 0 {
                chosen_level = chosen_level.saturating_sub(1);
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }
            if key_list.enter {
                // audio::play_audio(&audio_handle, "./sounds/enter.mp3");
                // thread::sleep_ms(800);

                return chosen_level as usize;
            }
            if key_list.l {
                if !(leaderboard(
                    level_id,
                    level_names[chosen_level as usize]
                        .to_str()
                        .unwrap()
                        .to_string(),
                    focused.clone(),
                    keys.clone(),
                )) {
                    exit_app();
                };
                menu_print();
                break;
            }
            if key_list.e {
                if screens::exit(focused.clone(), keys.clone()) {
                    exit_app();
                };
                menu_print();
                break;
            }
        }
    }
}

pub fn leaderboard(level_id: String, level_name: String, focused: Arc<Mutex<bool>>, keys: Arc<Mutex<Keys>>) -> bool {
    let (screen_width, screen_height) = renderer::get_terminal_size();
    let screen_width = screen_width as u16;
    let screen_height = screen_height as u16;
    let box_width = 50;
    let mut y = 0;
    let x = screen_width / 2 - box_width / 2;
    let margin = 3;
    let mut scroll: usize = 0;

    let mut leader_board = network::get_leader_board(&level_id);
    let leader_vec = leader_board.as_array_mut().expect("leaderboard error");
    let take = leader_vec.len().min(screen_height as usize - 10);

    leader_vec.sort_by_key(|val| {
        (val.get("time")
            .expect("leaderboard format wrong")
            .as_f64()
            .expect("leaderboard format wrong")
            * 1000.) as usize
    });

    // print background image
    print!("{esc}[H{esc}[48;2;0;0;0m", esc = 27 as char);
    for _row in 0..=screen_height {
        println!("{}\r", " ".repeat(screen_width as usize),)
    }

    let level_name = level_name.to_uppercase();

    y += 3;
    println!(
        "{esc}[{};{}H{:^3$}",
        y,
        x,
        format!("* LEADERBOARD FOR: {level_name} *"),
        (box_width) as usize,
        esc = 27 as char
    );
    y += 1;
    println!(
        "{esc}[{};{}H{:-^3$}",
        y,
        x,
        "",
        (box_width) as usize,
        esc = 27 as char
    );
    y += 1;

    loop {
        // print leaderboard

        for (i, result) in leader_vec.iter().skip(scroll).take(take).enumerate() {
            let mut name = result.get("name").unwrap().as_str().unwrap().to_string();
            let time = result.get("time").unwrap().as_f64().unwrap();
            let max_width = box_width
                - 2 * margin
                - format!("{:.2}", time).len() as u16
                - 7
                - i.to_string().len() as u16;
            if name.len() > max_width as usize {
                name = name[0..(max_width as usize - 3)].to_string() + "...";
            }

            println!(
                "{esc}[{};{}H{:<3$}",
                y + i as u16,
                x + margin,
                format!("{}. {} - {:.2}s", i + 1 + scroll, name, time,),
                (box_width - 4) as usize,
                esc = 27 as char
            );
        }

        // print lines

        println!(
            "{esc}[{};{}H{:-^3$}",
            y + take as u16,
            x,
            "",
            (box_width) as usize,
            esc = 27 as char
        );
        println!(
            "{esc}[{};{}H{}",
            y + take as u16 + 1,
            x,
            "Use |\u{1F845} | and |\u{1F847} | to scroll",
            esc = 27 as char
        );
        println!(
            "{esc}[{};{}H{}",
            y + take as u16 + 2,
            x,
            "Press |E| to go back",
            esc = 27 as char
        );

        thread::sleep_ms(200);

        //match input
        loop {
            let keys = if *focused.lock().unwrap() {
                keys.lock().unwrap().clone_reset_enter()
            }
            else {
                Keys::default()
            };

            if keys.up {
                scroll = scroll.saturating_sub(1);
                break;
            }
            if keys.down {
                if scroll < leader_vec.len() - take {
                    scroll += 1;
                }

                break;
            }
            if keys.e {
                return true;
            }
        }
    }
}

pub fn game_over(arg: &str, focused: Arc<Mutex<bool>>, keys: Arc<Mutex<Keys>>) -> bool {
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

            let mut key_list = if *focused.lock().unwrap() {
                keys.lock().unwrap().clone_reset_enter()
            }
            else {
                Keys::default()
            };
            
            if key_list.down {
                try_again = !try_again;
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }
            if key_list.up {
                try_again = !try_again;
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }
            if key_list.enter {
                return try_again;
            }
            if key_list.e {
                if exit(focused.clone(), keys.clone()) {
                    exit_app();
                }
                break;
            }
        }
    }
}

pub fn finish(time: f64, level_name: &str, level_map: &str, focused: Arc<Mutex<bool>>, keys: Arc<Mutex<Keys>>) -> u8 {
    // get device state for input

    // get terminal size
    let (screen_width, screen_height) = renderer::get_terminal_size();
    let screen_width = screen_width as u16;
    let screen_height = screen_height as u16;

    //for audio
    let (_stream, audio_handle) = OutputStream::try_default().unwrap();

    //name + crc32(map) == id
    let id = level_name.to_string() + &crc32fast::hash(level_map.as_bytes()).to_string();

    // get the leaderboard
    let mut leader_board = network::get_leader_board(&id);
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
            let mut name = result.get("name").unwrap().as_str().unwrap().to_string();
            let time = result.get("time").unwrap().as_f64().unwrap();
            let max_width = box_width - 10 - format!("{:.2}", time).len() as u16 - 1;
            if name.len() > max_width as usize {
                name = name[0..(max_width as usize - 3)].to_string() + "...";
            }

            println!(
                "{esc}[{};{}H| {:<3$} |",
                start_y + 6 + i as u16,
                start_x,
                format!("{}. {} - {:.2}s", i + 1, name, time,),
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
        // println!(
        //     "{esc}[{};{}Hid: {}",
        //     start_y + box_height + 2,
        //     start_x,
        //     id,
        //     esc = 27 as char
        // );

        thread::sleep_ms(200);

        //match input
        'input_loop: loop {
            let mut key_list = if *focused.lock().unwrap() {
                keys.lock().unwrap().clone_reset_enter()
            } else {
                Keys::default()
            };

            for key in KEYS_KEYCODE {
                if key_list.other.contains(&key.0) && chosen == 0 {
                    name.push(key.0);
                    break 'input_loop;
                }
            }

            if key_list.delete && chosen == 0 {
                name.pop();
                break;
            }

            if key_list.down && chosen != 2 {
                chosen += 1;
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }
            if key_list.up && chosen != 0 {
                chosen -= 1;
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }
            if key_list.enter && chosen != 0 {
                if !name.is_empty() {
                    network::log_result(&id, &name, time);
                }
                return chosen;
            }
            if key_list.enter && chosen == 0 {
                chosen += 1;
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }
            if key_list.e && chosen != 0 {
                if exit(focused.clone(), keys.clone()) {
                    exit_app();
                }
                break;
            }
        }
    }
}

pub fn exit(focused: Arc<Mutex<bool>>, keys: Arc<Mutex<Keys>>) -> bool {
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
 let key_list =            if *focused.lock().unwrap() {keys.lock().unwrap().clone_reset_enter()} else {Keys::default()};

            if key_list.down || key_list.up {
                exit = !exit;
                audio::play_audio(&audio_handle, "./sounds/pop.mp3");
                break;
            }

            if key_list.enter {
                return exit;
            }
        }
    }
}

fn exit_app() {
    let _ = crossterm::terminal::disable_raw_mode();
    let mut stdout = stdout(); 
    execute!(stdout, PopKeyboardEnhancementFlags);

    println!("\x1b[2J\x1b[H\x1b[48;2;0;0;0mGame closing\r");
    let _ = thread::spawn(|| {
        for x in stdin().bytes() {
            let _ = x;
        }
    });
    thread::sleep_ms(100);
    println!("\x1b[2J\x1b[H\x1b[48;2;0;0;0mGame Closed\r");

    std::process::exit(0);
}
