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
                x: 9.,
                y: -0.5,
                z: 10.,
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
