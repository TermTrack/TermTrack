use crate::{camera::Camera, mat::*};
use crossterm;
use rayon::prelude::*;

pub const RENDER_DIST: f64 = 30.;

pub fn get_terminal_size() -> (usize, usize) {
    let (w, h) = crossterm::terminal::size().unwrap();
    let w = (w as usize).min(350);
    let h = (h as usize).min(150);
    (w, h)
}

pub struct Screen {
    pub w: usize,
    pub h: usize,
    pub buffer: Vec<Vec<Vec3>>,
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
                w.into()
            ];
            h.into()
        ];

        Screen {
            w: w.into(),
            h: h.into(),
            buffer,
        }
    }

    fn flush(&mut self, ascii: bool) {
        // Create new string buffer
        let mut buffer = String::new();
        let mut color = Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        // Iterate through buffer
        for y in 0..self.buffer.len() {
            for x in 0..self.buffer[y].len() {
                // Add ' ' (char if ascii is true) withrightcolor to string buffer
                let mut c = ' ';
                // set background
                let mut command = "48";
                let col = self.buffer[y][x];
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
                    buffer += &format!(
                        "\x1b[{command};2;{};{};{}m{c}",
                        col.x as u8, col.y as u8, col.z as u8
                    );
                    color = col;
                } else {
                    buffer += " ";
                }
            }
            buffer += "\x1b[48;2;0;0;0m\r\n";
        }

        //Move cursor home and print string buffer
        print!("\x1b[H{}", buffer);

        let (w, h) = get_terminal_size();
        if self.w != w as usize || self.h != h as usize - 1 {
            print!("\x1b[2J\r");
            self.w = w as usize;
            self.h = h as usize - 1;
        }

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
                *pixel = color;
            }
        });
        if print {
            // true for ascii art
            self.flush(false);
            self.print_info(camera, extra)
        }
    }

    pub fn render_pruned_mt(&mut self, camera: &Camera, mesh: &Mesh, extra: &str, print: bool) {
        let mut pruned_tris = Vec::with_capacity(mesh.tris().len());
        let forward = Vec3 {
            x: 0.,
            y: 0.,
            z: 1.,
        }
        .rotate(camera.rotation);
        for tri in mesh.tris() {
            let mut valid = false;
            for p in [tri.v0, tri.v1, tri.v2] {
                // vector from pos to vertex
                let v = p - camera.pos;
                if v.abs() < RENDER_DIST {
                    valid = true;
                    break;
                }
                if v.dot(forward) >= 0. {
                    valid = true;
                    break;
                }
            }
            if valid {
                pruned_tris.push(tri);
            }
        }
        let pruned_mesh = Mesh { tris: pruned_tris };
        self.render_mt(&camera, &pruned_mesh, extra, print);
    }

    pub fn render_mt(&mut self, camera: &Camera, mesh: &Mesh, extra: &str, print: bool) {
        let tris = mesh.tris();
        let buffer = &mut self.buffer;

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
                for (idx, tri) in tris.iter().enumerate() {
                    let (hit, distance) = tri.hit_mt(ray_o, ray_dir);
                    let distance = distance;
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
                *pixel = color;
            })
        });
        if print {
            // true for ascii art
            // self.amplify_edges();
            self.flush(false);
            self.print_info(camera, &format!("{extra}"))
        }
    }

    fn print_info(&self, camera: &Camera, extra: &str) {
        print!("\x1b[48;1;0m{:<1$}", extra, self.w);
    }

    pub fn render_map(&self, map: &str, position: Vec3, grid_width: f64) {
        let map = map_as_vec_of_floors(map);

        let mut map: Vec<&str> = map
            .get((-position.y.div_euclid(grid_width as f64)) as usize)
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
    let mut map: Vec<Vec<&str>> = map
        .split("sep\n")
        .map(|x| x.split("\n").collect())
        .collect();
    map
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

        const T: usize = 1000;

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

        const T: usize = 1000;

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
    #[test]
    fn benchmark_mt_gpu() {
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

        const T: usize = 100;

        let now = Instant::now();
        for _ in 0..T {
            // s.render_mt_gpu(&c, &m, e, true);
        }
        let _ = crossterm::terminal::disable_raw_mode().unwrap();
        let t = now.elapsed();
        println!(
            "\r\n GPU Total time: {:.2?}, average: {:.2?}",
            t,
            t / T as u32
        );
    }
}
