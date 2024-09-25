use loader::*;
use mat::Vec3;
use renderer::Screen;

mod camera;
mod game;
mod loader;
mod mat;
mod renderer;

fn main() {
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
    match game.run() {
        Ok(time) => println!("You won!!!, your time is: {time}"),
        Err(e) => match e {
            "death" => println!("You died! try again"),
            _ => println!("you failed!"),
        },
    }
}
