use crate::{camera::Camera, mat::*};
use crossterm;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use term_size::dimensions;

use std::time::Instant;

const RENDER_DIST: f64 = 100.;

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
        let mut buffer = String::new();
        for y in 0..self.buffer.len() {
            for x in 0..self.buffer[y].len() {
                buffer += &format!(
                    "\x1b[48;2;{};{};{}m ",
                    self.buffer[y][x].x as u8, self.buffer[y][x].y as u8, self.buffer[y][x].z as u8
                )
            }
            if y != self.buffer.len() - 1 {
                buffer += "\r\n";
            }
        }
        print!("\x1b[H{}", buffer);
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

    pub fn clear(&mut self) {
        print!("\x1b[2J")
    }

    pub fn render(&mut self, camera: &Camera, mesh: Mesh) {
        let ray_origin = Vec3 {
            x: camera.pos.x + self.w as f64 / 2.,
            y: camera.pos.y + self.h as f64 / 2.,
            z: camera.pos.z - camera.fov,
        };
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
