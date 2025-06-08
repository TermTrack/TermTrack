use crossterm::cursor::Hide;
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
use crossterm::{self, execute};
use loader::*;
use mat::Vec3;
use renderer::Screen;
use rodio::OutputStream;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::stdout;
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
mod network;
mod renderer;
mod screens;

#[derive(Default,Clone)]
struct Keys {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub space: bool,
    pub up: bool,
    pub left: bool,
    pub down: bool,
    pub right: bool,
    pub enter: bool,
    pub e: bool,
    pub m: bool,
    pub r: bool,
    pub l: bool,
    pub delete: bool,
    pub other: HashSet<char>,
}

impl Keys {
    fn clone_reset_enter(&mut self) -> Self{
        let clone = self.clone();
        self.enter = false;
        clone
    }
}

fn main() {

    let mut stdout = stdout();

execute!(
    stdout,
    PushKeyboardEnhancementFlags(
        KeyboardEnhancementFlags::REPORT_EVENT_TYPES
    )
    , Hide
);


    
    let level_dir = env::args().collect::<Vec<String>>()[1].clone();
    let entries = fs::read_dir(level_dir).unwrap();
    let levels: Vec<PathBuf> = entries.map(|e| e.unwrap().path()).collect();
    let (_stream, stream_handle) = OutputStream::try_default().expect("couldnt get sound handle!");
    crossterm::terminal::enable_raw_mode().unwrap();
    let focused = Arc::new(Mutex::new(true));
    let focused_clone = Arc::clone(&focused);

    let keys = Arc::new(Mutex::new(Keys::default()));
    let keys_clone = Arc::clone(&keys);
    let key_check_thread = thread::spawn(move || {
        let mut last_enter = std::time::Instant::now();
        loop {
            
            match read().unwrap() {
                Event::FocusLost => *focused_clone.lock().unwrap() = false,
                Event::FocusGained => *focused_clone.lock().unwrap() = true,
                Event::Key(event) => {
                    
                    if event.is_press() {
                        match event.code {
                            KeyCode::Char('w') => keys_clone.lock().unwrap().w = true, 
                            KeyCode::Char('a') => keys_clone.lock().unwrap().a = true, 
                            KeyCode::Char('s') => keys_clone.lock().unwrap().s = true, 
                            KeyCode::Char('d') => keys_clone.lock().unwrap().d = true, 
                            KeyCode::Char('e') => keys_clone.lock().unwrap().e = true, 
                            KeyCode::Char('l') => keys_clone.lock().unwrap().l = true, 
                            KeyCode::Char('r') => keys_clone.lock().unwrap().r = true, 
                            KeyCode::Char('m') => keys_clone.lock().unwrap().m = true, 
                            KeyCode::Backspace => keys_clone.lock().unwrap().delete = true,
                            KeyCode::Enter =>  {
                                // enter is special because its often used to switch menus we want to give the user some times before pressing enter again
                                if std::time::Instant::now() - last_enter > std::time::Duration::from_millis(300) {
                                keys_clone.lock().unwrap().enter = true;
                                last_enter = std::time::Instant::now();
                                
                            }
                        },
                            KeyCode::Left => keys_clone.lock().unwrap().left = true,
                            KeyCode::Right => keys_clone.lock().unwrap().right = true,
                            KeyCode::Up => keys_clone.lock().unwrap().up = true,
                            KeyCode::Down => keys_clone.lock().unwrap().down = true,
                            KeyCode::Char(' ') => keys_clone.lock().unwrap().space = true,
                            _ => ()
                            
                            
                        }   
                    }
                    if event.is_release() {
                        match event.code {
                            KeyCode::Char('w') => keys_clone.lock().unwrap().w = false, 
                            KeyCode::Char('a') => keys_clone.lock().unwrap().a = false, 
                            KeyCode::Char('s') => keys_clone.lock().unwrap().s = false, 
                            KeyCode::Char('d') => keys_clone.lock().unwrap().d = false, 
                            KeyCode::Char('e') => keys_clone.lock().unwrap().e = false, 
                            KeyCode::Char('l') => keys_clone.lock().unwrap().l = false, 
                            KeyCode::Char('r') => keys_clone.lock().unwrap().r = false, 
                            KeyCode::Char('m') => keys_clone.lock().unwrap().m = false, 
                            KeyCode::Backspace => keys_clone.lock().unwrap().delete = false,
                            KeyCode::Left => keys_clone.lock().unwrap().left = false,
                            KeyCode::Right => keys_clone.lock().unwrap().right = false,
                            KeyCode::Up => keys_clone.lock().unwrap().up = false,
                            KeyCode::Down => keys_clone.lock().unwrap().down = false,
                            KeyCode::Char(' ') => keys_clone.lock().unwrap().space = false,
                            _ => ()
                            
                            
                        }   
                        
                    }
                },
                _ => (),
        }
    };
    });

    loop {
        let chosen_level = screens::menu(levels.clone(), &stream_handle, focused.clone(), keys.clone());
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
            match game.run(map.clone(), &stream_handle, focused.clone(), keys.clone()) {
                Ok(time) => {
                    if screens::finish(time, &map.level_name, &map.map_string, focused.clone(), keys.clone()) == 1
                    {
                        continue;
                    }
                }
                Err(e) => match e {
                    "void" => {
                        if screens::game_over("You fell into the void!", focused.clone(), keys.clone()) {
                            continue;
                        }
                    }
                    "angry_pixel" => {
                        if screens::game_over("Angry pixel killed you!", focused.clone(), keys.clone()) {
                            continue;
                        }
                    }
                    "spike" => {
                        if screens::game_over("You died of spike!", focused.clone(), keys.clone()) {
                            continue;
                        }
                    }
                    "menu" => (),
                    "retry" => continue,
                    _ => {
                        if screens::game_over("You failed!", focused.clone(), keys.clone()) {
                            continue;
                        }
                    }
                },
            }
            break; // break loop if not try again
        }
    }
}
