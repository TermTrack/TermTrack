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
                x: load(loader::MAP).1 .1,
                y: -0.5,
                z: load(loader::MAP).1 .0,
            },
            focus_length: 2.,
            rotation: Vec3 {
                x: 1.75,
                y: 0.0,
                z: 0.0,
            },
        },
    };
    game.run()
}
