use std::time::Instant;

use crate::{camera::Camera, mat::*};
use crossterm;
use rayon::prelude::*;

pub const RENDER_DIST: f64 = 50.;

pub struct Screen {
    pub w: usize,
    pub h: usize,
    pub buffer: Vec<Vec<(Vec3, f64)>>,
    raw: (),
}

impl Screen {
    pub fn new() -> Self {
        let (w, h) = crossterm::terminal::size().unwrap();
        let raw = crossterm::terminal::enable_raw_mode().unwrap();
        let h = h - 2;
        // hide cursor
        print!("\x1b[?25l");

        //clear screen
        print!("\x1b[2J");

        Screen {
            w: w.into(),
            h: h.into(),
            buffer: vec![
                vec![
                    (
                        Vec3 {
                            x: 0.,
                            y: 0.,
                            z: 0.
                        },
                        0.
                    );
                    w.into()
                ];
                h.into()
            ],
            raw,
        }
    }

    fn flush(&mut self, ascii: bool) {
        // Create new string buffer
        let mut buffer = String::new();
        // Iterate through buffer
        for y in 0..self.buffer.len() {
            for x in 0..self.buffer[y].len() {
                // Add ' ' (char if ascii is true) withrightcolor to string buffer
                let mut c = ' ';
                // set background
                let mut command = "48";
                let col = self.buffer[y][x].0;
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
                buffer += &format!(
                    "\x1b[{command};2;{};{};{}m{c}",
                    col.x as u8, col.y as u8, col.z as u8
                )
            }
        }

        //Move cursor home and print string buffer
        print!("\x1b[H{}", buffer);

        // clear buffer
        self.buffer = vec![
            vec![
                (
                    Vec3 {
                        x: 0.,
                        y: 0.,
                        z: 0.
                    },
                    0.
                );
                self.w
            ];
            self.h
        ]
    }

    // Not used, left for benchmark against new version
    pub fn render_geometric(&mut self, camera: &Camera, mesh: &Mesh, extra: &str, print: bool) {
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
                let min_dim = self.w.min(self.h * 2) as f64 / 2.;
                let pixel_coords = Vec3 {
                    x: (x as f64 - self.w as f64 / 2.) / min_dim,
                    y: (y as f64 * 2. - self.h as f64 / 2.) / min_dim,
                    z: camera.focus_length,
                };
                let pixel_coords = pixel_coords.rotate(camera.rotation);
                let ray_dir = pixel_coords;
                let ray_o = camera.pos + pixel_coords;
                for tri in &tris {
                    let (hit, distance) = tri.hit_geo(ray_o, ray_dir);
                    if hit {
                        if distance < min_dist {
                            min_dist = distance;

                            color = tri.color;
                        }
                    }
                }
                color = color * (1. - min_dist / RENDER_DIST);
                *pixel = (color, min_dist);
            }
        });
        if print {
            // true for ascii art
            self.flush(false);
            self.print_info(camera, extra)
        }
    }

    pub fn render_mt(&mut self, camera: &Camera, mesh: &Mesh, extra: &str, print: bool) {
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
                let min_dim = self.w.min(self.h * 2) as f64 / 2.;
                let pixel_coords = Vec3 {
                    x: (x as f64 - self.w as f64 / 2.) / min_dim,
                    y: (y as f64 * 2. - self.h as f64 / 2.) / min_dim,
                    z: camera.focus_length,
                };
                let pixel_coords = pixel_coords.rotate(camera.rotation);
                let pixel_distance = pixel_coords.abs();
                let ray_dir = pixel_coords;
                let ray_o = camera.pos + pixel_coords;
                let mut closet_idx = None;
                for (idx, tri) in tris.iter().enumerate() {
                    let (hit, distance) = tri.hit_mt(ray_o, ray_dir);
                    let distance = distance - pixel_distance;
                    if hit {
                        if distance < min_dist {
                            min_dist = distance;

                            closet_idx = Some(idx);
                        }
                    }
                }
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
                *pixel = (color, min_dist);
            }
        });
        if print {
            // true for ascii art
            // self.amplify_edges();
            self.flush(false);
            self.print_info(camera, &format!("{extra}"))
        }
    }

    fn print_info(&self, camera: &Camera, extra: &str) {
        print!(
            "\x1b[48;1;0mposition: {:.2?}, rotation: {:.2?}, extra: {}",
            camera.pos, camera.rotation, extra
        )
    }

    fn amplify_edges(&mut self) {
        const LOWER_THRESHOLD: f64 = 0.5;
        const HIGHER_THRESHOLD: f64 = 1_000_000_000.;
        for x in 1..self.buffer.len() - 1 {
            for y in 1..self.buffer[x].len() - 1 {
                let gy1 = self.buffer[x][y].1 - self.buffer[x][y - 1].1;
                if gy1 > HIGHER_THRESHOLD {
                    continue;
                }
                let gy2 = self.buffer[x][y + 1].1 - self.buffer[x][y].1;
                if gy2 > HIGHER_THRESHOLD {
                    continue;
                }
                let gx1 = self.buffer[x][y].1 - self.buffer[x - 1][y].1;
                if gx1 > HIGHER_THRESHOLD {
                    continue;
                }
                let gx2 = self.buffer[x + 1][y].1 - self.buffer[x][y].1;
                if gx2 > HIGHER_THRESHOLD {
                    continue;
                }
                if (gy1 - gy2).abs() > LOWER_THRESHOLD || (gx1 - gx2).abs() > LOWER_THRESHOLD {
                    self.buffer[x][y].0 = Vec3 {
                        x: 0.,
                        y: 0.,
                        z: 0.,
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn benchmark_geometric() {
        let mut s = Screen::new();
        let c = Camera {
            pos: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            rotation: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            focus_length: 0.5,
            vel: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
        };
        let e = "tom";

        let m = Mesh::new(vec![
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
        ]);

        const T: usize = 10_000;

        let now = Instant::now();
        for _ in 0..T {
            s.render_geometric(&c, &m, e, false);
        }
        let _ = crossterm::terminal::disable_raw_mode().unwrap();
        let t = now.elapsed();
        println!(
            "\r\n GEO Total time: {:.2?}, average: {:.2?}",
            t,
            t / T as u32
        );
    }

    #[test]
    fn benchmark_mt() {
        let mut s = Screen::new();
        let c = Camera {
            pos: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            rotation: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            focus_length: 0.5,
            vel: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
        };
        let e = "tom";
        let m = Mesh::new(vec![
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
            (0., 0., 1.),
            (1., 1., 1.),
            (1., 0., 1.),
            (200., 200., 200.),
        ]);

        const T: usize = 10_000;

        let now = Instant::now();
        for _ in 0..T {
            s.render_mt(&c, &m, e, false);
        }
        let _ = crossterm::terminal::disable_raw_mode().unwrap();
        let t = now.elapsed();
        println!(
            "\r\n MT Total time: {:.2?}, average: {:.2?}",
            t,
            t / T as u32
        );
    }
}
