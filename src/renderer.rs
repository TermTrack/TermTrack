use crate::{camera::Camera, mat::*};
use crossterm;
use term_size::dimensions;

const RENDER_DIST: f64 = 70.;

pub struct Screen {
    pub w: usize,
    pub h: usize,
    pub buffer: String,
    raw: (),
}

impl Screen {
    pub fn new() -> Self {
        let (w, h) = dimensions().unwrap();
        let raw = crossterm::terminal::enable_raw_mode().unwrap();

        // hide cursor
        print!("\x1b[?25l");

        //clear screen
        print!("\x1b[2J");

        Screen {
            w,
            h,
            buffer: String::from("\x1b[H"),
            raw,
        }
    }

    fn push_color(&mut self, col: Vec3) {
        self.buffer += &format!(
            "\x1b[48;2;{};{};{}m ",
            col.x as u8, col.y as u8, col.z as u8
        );
    }

    fn nl(&mut self) {
        self.buffer += "\r\n"
    }

    fn flush(&mut self) {
        print!("{}", self.buffer);
        self.buffer = String::from("\x1b[H");
    }

    pub fn clear(&mut self) {
        print!("\x1b[2J")
    }

    pub fn render(&mut self, camera: &Camera, mesh: &Mesh) {
        let ray_origin = Vec3 {
            x: camera.pos.x + self.w as f64 / 2.,
            y: camera.pos.y + self.h as f64 / 2.,
            z: camera.pos.z - camera.fov,
        };
        let mut tris = mesh.tris();

        for y in 0..self.h {
            for x in 0..self.w {
                let mut min_dist = f64::MAX;
                let mut color = Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                };
                for tri in tris.iter() {
                    let ray_dir = Vec3 {
                        x: x as f64 + camera.pos.x,
                        y: y as f64 + camera.pos.y,
                        z: 0.,
                    } - ray_origin;
                    let ray_o = Vec3 {
                        x: x as f64 + camera.pos.x,
                        y: y as f64 + camera.pos.y,
                        z: 0.,
                    };
                    if tri.hit(ray_o, ray_dir) {
                        if tri.distance(ray_o, ray_dir).unwrap() < min_dist {
                            min_dist = tri.distance(ray_o, ray_dir).unwrap();

                            color = tri.color
                                * (1. - tri.distance(ray_o, ray_dir).unwrap() / RENDER_DIST);
                        }
                    }
                }
                self.push_color(color);
            }
            if y != self.h - 1 {
                self.nl();
            }
        }
        self.flush();
    }
}
