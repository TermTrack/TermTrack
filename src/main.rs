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
