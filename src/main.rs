use mat::Vec3;
use renderer::Screen;

mod camera;
mod game;
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
            fov: 1.,
        },
    };
    game.run()
}
