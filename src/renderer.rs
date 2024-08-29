use crate::{camera::Camera, mat::*};
use crossterm;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use term_size::dimensions;

const RENDER_DIST: f64 = 300.;

pub struct Screen {
    pub w: usize,
    pub h: usize,
    pub buffer: Vec<Vec<Vec3>>,
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
            buffer: vec![
                vec![
                    Vec3 {
                        x: 0.,
                        y: 0.,
                        z: 0.
                    };
                    w
                ];
                h
            ],
            raw,
        }
    }

    fn flush(&mut self) {
        // Create new string buffer
        let mut buffer = String::new();
        // Iterate through buffer
        for y in 0..self.buffer.len() {
            for x in 0..self.buffer[y].len() {
                // Add ' ' withrightcolor to string buffer
                buffer += &format!(
                    "\x1b[48;2;{};{};{}m ",
                    self.buffer[y][x].x as u8, self.buffer[y][x].y as u8, self.buffer[y][x].z as u8
                )
            }
            // Add newline if not on last line
            if y != self.buffer.len() - 1 {
                buffer += "\r\n";
            }
        }

        //Move cursor home and print string buffer
        print!("\x1b[H{}", buffer);
        // clear buffer
        self.buffer = vec![
            vec![
                Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.
                };
                self.w
            ];
            self.h
        ]
    }

    pub fn render(&mut self, camera: &Camera, mesh: &Mesh) {
        let tris = mesh.tris();
        let buffer = &mut self.buffer;

        buffer.par_iter_mut().enumerate().for_each(|(y, row)| {
            for (x, pixel) in row.iter_mut().enumerate() {
                let mut min_dist = f64::MAX;
                let mut color = Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                };
                for tri in &tris {
                    let pixel_coords = Vec3 {
                        x: x as f64 - (self.w as f64) / 2.,
                        y: y as f64 - (self.h as f64) / 2.,
                        z: camera.focus_length,
                    };
                    let pixel_coords = pixel_coords.rotate(camera.rotation);
                    let ray_dir = pixel_coords
                        - Vec3 {
                            x: 0.,
                            y: 0.,
                            z: 0.,
                        };
                    let ray_o = camera.pos + pixel_coords;
                    let (hit, distance) = tri.hit(ray_o, ray_dir);
                    if hit {
                        if distance < min_dist {
                            min_dist = distance;

                            color = tri.color;
                        }
                    }
                }
                color = color * (1. - min_dist / RENDER_DIST);
                *pixel = color;
            }
        });

        self.flush();
    }
}
