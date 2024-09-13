use futures::executor::block_on;
use std::mem::size_of_val;

use crate::{camera::Camera, mat::*};
use crossterm;
use rayon::prelude::*;
use wgpu::util::DeviceExt;

pub const RENDER_DIST: f64 = 45.;

pub struct Screen {
    pub w: usize,
    pub h: usize,
    pub buffer: Vec<Vec<Vec3>>,
    raw: (),
    context: WgpuContext,
}

struct WgpuContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    compute_pipeline: wgpu::ComputePipeline,
}
impl WgpuContext {
    async fn new() -> Self {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_defaults(),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .unwrap();

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "shader.wgsl"
            ))),
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &shader,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });

        Self {
            device,
            queue,
            compute_pipeline,
        }
    }
}

impl Screen {
    pub fn new() -> Self {
        let (w, h) = crossterm::terminal::size().unwrap();
        let raw = crossterm::terminal::enable_raw_mode().unwrap();
        let mut stdout = std::io::stdout();
        crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen).unwrap();
        let h = h - 2;
        // hide cursor
        print!("\x1b[?25l");

        //clear screen
        print!("\x1b[2J");

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

        let device_context = block_on(WgpuContext::new());

        Screen {
            w: w.into(),
            h: h.into(),
            buffer,
            raw,
            context: device_context,
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
                buffer += &format!(
                    "\x1b[{command};2;{};{};{}m{c}",
                    col.x as u8, col.y as u8, col.z as u8
                )
            }
            buffer += "\r\n"
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

    pub fn render_mt_gpu(&mut self, camera: &Camera, mesh: &Mesh, extra: &str, print: bool) {
        let mut pixels = vec![];
        let mut colors = vec![];
        let mut vertices = vec![];

        // TODO: I will need to prepare the pixelcoords and save them like this: [x1, y1, z1, x2, y2, z2...]
        // I will also need to save all the triangle vertices like this: [[x1, y1, z1], [x2,y2,z2], [x3,y3,z3], [x1, y1, z1] ...]
        // And the color like this: [[r,g,b][r,g,b], ...]

        for y in 0..self.h {
            for x in 0..self.w {
                let min_dim = self.w.min(self.h * 2) as f64 / 2.;
                let pixel_coords = Vec3 {
                    x: (x as f64 - self.w as f64 / 2.) / min_dim,
                    y: (y as f64 * 2. - self.h as f64 / 2.) / min_dim,
                    z: camera.focus_length,
                };
                let pixel_coords = pixel_coords.rotate(camera.rotation);
                pixels.push([
                    pixel_coords.x as f32,
                    pixel_coords.y as f32,
                    pixel_coords.z as f32,
                ]);
            }
        }

        for tri in mesh.tris() {
            let v0 = [tri.v0.x as f32, tri.v0.y as f32, tri.v0.z as f32];
            let v1 = [tri.v1.x as f32, tri.v1.y as f32, tri.v1.z as f32];
            let v2 = [tri.v2.x as f32, tri.v2.y as f32, tri.v2.z as f32];
            vertices.push(v0);
            vertices.push(v1);
            vertices.push(v2);
            let color = [tri.color.x as f32, tri.color.y as f32, tri.color.z as f32];
            colors.push(color);
        }

        let camera_position = [
            camera.pos.x as f32,
            camera.pos.y as f32,
            camera.pos.z as f32,
        ];

        let result = block_on(execute_gpu(
            &mut self.context,
            &pixels,
            &colors,
            &vertices,
            &camera_position,
        ))
        .unwrap();

        for (y, row) in self.buffer.iter_mut().enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                *pixel = Vec3 {
                    x: result[y * self.w + x][0] as f64,
                    y: result[y * self.w + x][1] as f64,
                    z: result[y * self.w + x][2] as f64,
                };
            }
        }

        if print {
            self.flush(false);
            self.print_info(camera, &format!("{extra}"))
        }
    }

    fn print_info(&self, camera: &Camera, extra: &str) {
        print!("\x1b[48;1;0m{}", extra)
    }

    pub fn render_map(&self, map: &str, position: Vec3, grid_width: f64) {
        let mut map: Vec<_> = map.split("\n").collect();
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

// let result = execute_gpu(&pixels, &colors, &vertices, &camera_position)
async fn execute_gpu(
    context: &mut WgpuContext,
    pixels: &[[f32; 3]],
    colors: &[[f32; 3]],
    vertices: &[[f32; 3]],
    camera_pos: &[f32; 3],
) -> Option<Vec<[f32; 3]>> {
    let queue = &context.queue;
    let device = &context.device;
    let shader = &context.compute_pipeline;

    execute_gpu_inner(
        &device, &queue, &shader, pixels, colors, vertices, camera_pos,
    )
    .await
}

async fn execute_gpu_inner(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    compute_pipeline: &wgpu::ComputePipeline,
    pixels: &[[f32; 3]],
    colors: &[[f32; 3]],
    vertices: &[[f32; 3]],
    camera_pos: &[f32; 3],
) -> Option<Vec<[f32; 3]>> {
    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(camera_pos),
        usage: wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::STORAGE,
    });

    // Buffers
    let pixels_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(pixels),
        usage: wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::STORAGE,
    });
    let colors_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(colors),
        usage: wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::STORAGE,
    });
    let vertices_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::STORAGE,
    });

    let size = size_of_val(pixels) as wgpu::BufferAddress;

    //Result CPU buffer
    let result_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: pixels_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: colors_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: vertices_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: camera_buffer.as_entire_binding(),
            },
        ],
    });

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("Compute pixel colors");
        cpass.dispatch_workgroups(pixels.len() as u32, 1, 1);
    }

    encoder.copy_buffer_to_buffer(&pixels_buffer, 0, &result_buffer, 0, size);

    queue.submit(Some(encoder.finish()));
    let buffer_slice = result_buffer.slice(..);

    let (sender, receiver) = flume::bounded(1);
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    device.poll(wgpu::Maintain::wait()).panic_on_timeout();

    if let Ok(Ok(())) = receiver.recv_async().await {
        let data = buffer_slice.get_mapped_range();

        let result = bytemuck::cast_slice(&data).to_vec();

        drop(data);
        result_buffer.unmap();

        Some(result)
    } else {
        panic!("failed to run compute on gpu!")
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

        const T: usize = 1;

        let now = Instant::now();
        for _ in 0..T {
            s.render_mt_gpu(&c, &m, e, false);
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
