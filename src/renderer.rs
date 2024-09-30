use std::sync::{Arc, Mutex};

use crate::{camera::Camera, mat::*};
use rayon::prelude::*;

pub const RENDER_DIST: f64 = 30.;

pub fn get_terminal_size() -> (usize, usize) {
    let (w, h) = crossterm::terminal::size().unwrap();
    let w = (w as usize).min(220);
    let h = (h as usize).min(65);
    (w, h)
}

#[derive(Clone)]
pub struct Screen {
    pub w: usize,
    pub h: usize,
}

impl Screen {
    pub fn new() -> Self {
        let (w, h) = get_terminal_size();
        let mut stdout = std::io::stdout();
        crossterm::execute!(stdout, crossterm::cursor::Hide).unwrap();
        let h = h - 1;

        //clear screen
        print!("\x1b[2J\x1b[H\r");

        let buffer = vec![
            vec![
                Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.
                };
                w
            ];
            h
        ];

        Screen { w, h }
    }

    pub fn flush(&mut self, buffer: &[Vec<Vec3>], ascii: bool, extra: &str) {
        // Create new string buffer
        let mut pix_buffer = String::new();
        let mut color = Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        // Iterate through buffer
        for y in 0..buffer.len() {
            for x in 0..buffer[y].len() {
                // Add ' ' (char if ascii is true) withrightcolor to string buffer
                let mut c = ' ';
                // set background
                let mut command = "48";
                let col = buffer[y][x];
                if ascii {
                    // set foreground
                    command = "38";
                    let chars = [
                        ' ', '.', '-', ':', '_', '~', '/', 'c', 'r', 'x', '*', '%', '#', '8', '@',
                    ];

                    let s = col.x + col.y + col.z;
                    let s = s * chars.len() as f64 / (255 * 3) as f64;
                    let s = (s as usize).clamp(0, chars.len() - 1);
                    c = chars[s];
                }
                if col != color {
                    pix_buffer += &format!(
                        "\x1b[{command};2;{};{};{}m{c}",
                        col.x as u8, col.y as u8, col.z as u8
                    );
                    color = col;
                } else {
                    pix_buffer += " ";
                }
            }
            pix_buffer += "\x1b[48;2;0;0;0m\r\n";
        }
        pix_buffer += &format!("{:<1$}", extra, self.w);

        //Move cursor home and print string buffer
        print!("\x1b[H{}", pix_buffer);

        let (w, h) = get_terminal_size();
        if self.w != w || self.h != h - 1 {
            print!("\x1b[2J\r");
            self.w = w;
            self.h = h - 1;
        }
    }

    // Not used, left for benchmark against new version

    pub fn render_pruned_mt(&self, camera: &Camera, mesh: &Mesh) -> Vec<Vec<Vec3>> {
        let mut pruned_tris = Vec::with_capacity(mesh.tris().len());
        let forward = Vec3 {
            x: 0.,
            y: 0.,
            z: 1.,
        };
        let min_dim = self.w.min(self.h * 2) as f64 / 2.;
        let pixel_coords = Vec3 {
            x: -(self.w as f64 / 2.) / min_dim,
            y: -(self.h as f64 / 2.) / min_dim,
            z: camera.focus_length,
        };
        let angle = (forward.dot(pixel_coords) / pixel_coords.abs())
            .acos()
            .abs()
            + 5.;
        let forward = forward.rotate(camera.rotation);
        for tri in mesh.tris() {
            let mut valid = false;
            for p in [tri.v0, tri.v1, tri.v2] {
                // vector from pos to vertex
                let v = p - camera.pos;
                let v_abs = v.abs();
                if v_abs < RENDER_DIST {
                    valid = true;
                    break;
                }
                if (v.dot(forward) / v_abs).acos() <= angle {
                    valid = true;
                    break;
                }
            }
            if valid {
                pruned_tris.push(tri);
            }
        }
        self.render_mt(camera, &pruned_tris)
    }

    pub fn render_mt(&self, camera: &Camera, tris: &[Tri]) -> Vec<Vec<Vec3>> {
        let mut buffer = vec![
            vec![
                Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.
                };
                self.w
            ];
            self.h
        ];

        buffer.par_iter_mut().enumerate().for_each(|(y, row)| {
            row.par_iter_mut().enumerate().for_each(|(x, pixel)| {
                let mut min_dist = f64::MAX;
                let mut color = Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                };
                let min_dim = self.w.min(self.h * 2) as f64 / 2.;
                let pixel_coords = Vec3 {
                    x: (x as f64 - self.w as f64 / 2.) / min_dim,
                    y: (y as f64 * 2. - self.h as f64 / 2.) / min_dim,
                    z: camera.focus_length,
                };
                let pixel_coords = pixel_coords.rotate(camera.rotation);
                let ray_dir = pixel_coords;
                let ray_o = camera.pos;
                let mut closet_idx = None;
                tris.iter().enumerate().for_each(|(idx, tri)| {
                    let (hit, distance) = tri.hit_mt(ray_o, ray_dir);
                    if hit && distance < min_dist {
                        min_dist = distance;

                        closet_idx = Some(idx);
                    }
                });
                if let Some(idx) = closet_idx {
                    let tri = tris[idx];
                    color = tri.color;
                    let n = tri.normal();
                    color = color
                        * (n.dot(ray_dir * (-1.)) / (ray_dir.abs() * n.abs()))
                            .abs()
                            .clamp(0.5, 1.);
                }
                color = color * (1. - min_dist / RENDER_DIST);
                *pixel = color;
            });
        });
        buffer
    }

    fn print_info(&self, camera: &Camera, extra: &str) {
        print!("\x1b[48;1;0m{:<1$}", extra, self.w);
    }

    pub fn render_map(&self, map: &str, position: Vec3, grid_width: f64) {
        let map = map_as_vec_of_floors(map);

        let mut map: Vec<&str> = map
            .get((-position.y.div_euclid(grid_width)) as usize)
            .unwrap_or(map.last().unwrap_or(&vec![]))
            .clone();

        let height = map.len();
        let mut width = 0;
        let [pos_x, pos_y] = [
            (position.x / grid_width) as usize,
            (position.z / grid_width) as usize,
        ];

        let mut player_string = String::new();

        for (i, row) in map.iter_mut().enumerate() {
            width = width.max(row.len());
            if i == pos_y {
                let (p1, p2) = row.split_at(pos_x);
                let (p2, p3) = p2.split_at(1);
                player_string = format!("{p1}\x1b[48;2;50;255;50m{p2}\x1b[48;2;000;000;000m{p3}");
                *row = &player_string;
                break;
            }
        }

        print!("\x1b[48;2;000;000;000m\x1b[\r");

        let x_start = self.w / 2 - width / 2 - 2;
        let y_start = self.h / 2 - height / 2 - 3;

        println!("\x1b[{};{}H*{:-^3$}*\r", y_start, x_start, "-", width + 4);
        println!(
            "\x1b[{};{}H|{:^3$}|\r",
            y_start + 1,
            x_start,
            " ",
            width + 4
        );
        let y_start = y_start + 2;
        for (i, row) in map.iter().enumerate() {
            //Move cursor
            print!("\x1b[{};{}H", y_start + i, x_start);
            let mut extra = 4;

            if i == pos_y {
                extra += 36;
            }
            //println center aligned string
            println!("|{:^1$}|\r", row, width + extra);
        }
        let y_start = y_start + height;

        println!(
            "\x1b[{};{}H*{:-^3$}*\r",
            y_start + 1,
            x_start,
            "-",
            width + 4
        );
        println!("\x1b[{};{}H|{:^3$}|\r", y_start, x_start, " ", width + 4);
    }
}

pub fn map_as_vec_of_floors(map: &str) -> Vec<Vec<&str>> {
    let map: Vec<Vec<&str>> = map
        .split("sep\n")
        .map(|x| x.split("\n").collect())
        .collect();
    map
}
