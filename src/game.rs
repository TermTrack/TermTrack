use device_query::{DeviceQuery, DeviceState, Keycode};
use rodio::OutputStreamHandle;

use rodio::{source::Source, OutputStream};

use crate::loader::{self};
use crate::renderer::{self, *};
use crate::GW;
use crate::{audio, LevelMap};
use crate::{camera::Camera, mat::*};
use crate::{screens, GH};
use core::panic;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Enemy {
    pos: Vec3,
    speed: f64,
    vel: Vec3,
    collider: BoxCollider,
    mesh: Mesh,
    vision: f64,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            pos: Vec3 {
                x: GW * 0.5,
                y: GH * 0.5,
                z: GW * 0.5,
            },
            speed: 20.,
            vel: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            collider: BoxCollider {
                max_x: GW * 0.1,
                max_y: GW * 0.1,
                max_z: GW * 0.1,
                min_x: -GW * 0.1,
                min_y: -GW * 0.1,
                min_z: -GW * 0.1,
                tag: Some("angry_pixel"),
            },
            mesh: Mesh::new(Vec::from([
                //left
                (-GW * 0.1, -GW * 0.1, -GW * 0.1),
                (-GW * 0.1, -GW * 0.1, GW * 0.1),
                (-GW * 0.1, GW * 0.1, GW * 0.1),
                (255., 0., 0.),
                (-GW * 0.1, -GW * 0.1, -GW * 0.1),
                (-GW * 0.1, GW * 0.1, GW * 0.1),
                (-GW * 0.1, GW * 0.1, -GW * 0.1),
                (255., 0., 0.),
                //front
                (-GW * 0.1, -GW * 0.1, -GW * 0.1),
                (GW * 0.1, -GW * 0.1, -GW * 0.1),
                (GW * 0.1, GW * 0.1, -GW * 0.1),
                (255., 0., 0.),
                (-GW * 0.1, -GW * 0.1, -GW * 0.1),
                (-GW * 0.1, GW * 0.1, -GW * 0.1),
                (GW * 0.1, GW * 0.1, -GW * 0.1),
                (255., 0., 0.),
                //right
                (GW * 0.1, -GW * 0.1, -GW * 0.1),
                (GW * 0.1, -GW * 0.1, GW * 0.1),
                (GW * 0.1, GW * 0.1, GW * 0.1),
                (255., 0., 0.),
                (GW * 0.1, -GW * 0.1, -GW * 0.1),
                (GW * 0.1, GW * 0.1, GW * 0.1),
                (GW * 0.1, GW * 0.1, -GW * 0.1),
                (255., 0., 0.),
                //back
                (-GW * 0.1, -GW * 0.1, GW * 0.1),
                (GW * 0.1, -GW * 0.1, GW * 0.1),
                (GW * 0.1, GW * 0.1, GW * 0.1),
                (255., 0., 0.),
                (-GW * 0.1, -GW * 0.1, GW * 0.1),
                (-GW * 0.1, GW * 0.1, GW * 0.1),
                (GW * 0.1, GW * 0.1, GW * 0.1),
                (255., 0., 0.),
                //top
                (-GW * 0.1, GW * 0.1, -GW * 0.1),
                (-GW * 0.1, GW * 0.1, GW * 0.1),
                (GW * 0.1, GW * 0.1, -GW * 0.1),
                (255., 0., 0.),
                (GW * 0.1, GW * 0.1, GW * 0.1),
                (-GW * 0.1, GW * 0.1, GW * 0.1),
                (GW * 0.1, GW * 0.1, -GW * 0.1),
                (255., 0., 0.),
                // bottom
                (-GW * 0.1, -GW * 0.1, -GW * 0.1),
                (-GW * 0.1, -GW * 0.1, GW * 0.1),
                (GW * 0.1, -GW * 0.1, -GW * 0.1),
                (255., 0., 0.),
                (GW * 0.1, -GW * 0.1, GW * 0.1),
                (-GW * 0.1, -GW * 0.1, GW * 0.1),
                (GW * 0.1, -GW * 0.1, -GW * 0.1),
                (255., 0., 0.),
            ])),
            vision: GW * 4.,
        }
    }
}

impl Enemy {
    pub fn update(&mut self, dt: f64, player_positon: Vec3, colliders: &Vec<BoxCollider>) {
        let vec_to_player = player_positon - self.pos;
        if (vec_to_player).abs() < self.vision {
            self.vel = vec_to_player.norm() * self.speed;
            let mut col = self.collider.clone();
            check_collision(
                &mut col,
                &mut self.pos,
                &mut self.vel,
                dt,
                colliders,
                &mut false,
            );
            self.pos = self.pos + self.vel * dt;
        }
    }
    pub fn get_collider(&self) -> BoxCollider {
        let mut col = self.collider.clone();
        col.translate(self.pos);
        col
    }

    pub fn translate(mut self, to: Vec3) -> Self {
        self.pos = self.pos + to;
        self
    }

    pub fn get_mesh(&self) -> Mesh {
        let mut mesh = self.mesh.clone();
        for tri in mesh.mut_tris() {
            tri.v0 = tri.v0
                + Vec3 {
                    x: self.pos.x,
                    z: self.pos.z,
                    y: self.pos.y,
                };
            tri.v1 = tri.v1
                + Vec3 {
                    x: self.pos.x,
                    z: self.pos.z,
                    y: self.pos.y,
                };
            tri.v2 = tri.v2
                + Vec3 {
                    x: self.pos.x,
                    z: self.pos.z,
                    y: self.pos.y,
                };
        }
        mesh
    }
}

#[derive(Clone)]
pub struct Game {
    pub renderer: Screen,
    pub camera: Camera,
}

const SPEED: f64 = 27.;
const JUMP_SPEED: f64 = 47.5;
const ROTATION_SPEED: f64 = 2.5;
const GRAVITY: f64 = 105.;
const PLAYER_COLLIDER: ((f64, f64, f64), (f64, f64, f64)) = ((-1., 4.5, -1.), (1., -1., 1.));

impl Game {
    pub fn run(
        &mut self,
        map: loader::LevelMap,
        audio_handle: &OutputStreamHandle,
    ) -> Result<f64, &str> {
        // load map files
        // generate map meshes

        let LevelMap {
            mesh,
            colliders,
            start_pos: start,
            map_string,
            level_name,
            mut enemies,
        } = map;

        self.camera.pos = Vec3 {
            x: start.0,
            y: start.1,
            z: start.2,
        };

        let mut level_timer = Instant::now();

        // timer for fps
        let mut time = Instant::now();

        // device for input
        let device_state = DeviceState::new();

        let floors = renderer::map_as_vec_of_floors(&map_string).len();

        // Get an output stream handle to the default physical sound device
        let (_stream, level_audio_handle) = OutputStream::try_default().unwrap();
        audio::audio_loop(&level_audio_handle, "./sounds/background.mp3");
        let walk = audio::create_infinite_sink(&level_audio_handle, "./sounds/walk.mp3");
        walk.set_volume(30.);
        walk.pause();

        loop {
            // reset timer
            let dt = time.elapsed().as_secs_f64();
            time = Instant::now();

            let fps_text = format!("fps: {:.2?} ", 1. / (dt));
            let timer_text = format!("time: {:.1?} ", level_timer.elapsed());
            let floor_text = format!(
                "floor: {}/{}",
                (-self.camera.pos.y.div_euclid(GH)).clamp(0., floors as f64) as usize,
                floors
            );

            let dt = dt.min(0.2);

            //update enemies
            let mut render_mesh = mesh.clone();
            let mut cols = colliders.clone();
            for enemy in enemies.iter_mut() {
                enemy.update(dt, self.camera.pos, &colliders);
                cols.push(enemy.get_collider());
                render_mesh = render_mesh + enemy.get_mesh();
            }

            // render vertices in parallel thread.
            let cam = self.camera.clone();
            let renderer = self.renderer.clone();
            let render_thread =
                thread::spawn(move || renderer.render_pruned_mt(&cam, &render_mesh));

            // handle input
            let mouse = device_state.get_mouse();
            let keys = device_state.get_keys();

            if keys.contains(&Keycode::E) {
                let time1 = time.elapsed();
                let timer_1 = level_timer.elapsed();

                if screens::exit() {
                    return Err("menu");
                };
                time = Instant::now().checked_sub(time1).unwrap();
                level_timer = Instant::now().checked_sub(timer_1).unwrap();
            }
            if keys.contains(&Keycode::M) {
                let time1 = time.elapsed();
                let timer_1 = level_timer.elapsed();
                time = Instant::now();
                self.renderer
                    .render_map(&map_string, self.camera.pos, loader::GW);
                loop {
                    if device_state.get_keys().contains(&Keycode::M) {
                        if time.elapsed() < Duration::from_millis(150) {
                            continue;
                        }
                        thread::sleep_ms(150);
                        break;
                    }
                }
                time = Instant::now().checked_sub(time1).unwrap();
                level_timer = Instant::now().checked_sub(timer_1).unwrap();
            }
            if keys.contains(&Keycode::Left) {
                self.camera.rotation.x -= ROTATION_SPEED * dt;
            }
            if keys.contains(&Keycode::Right) {
                self.camera.rotation.x += ROTATION_SPEED * dt;
            }
            if keys.contains(&Keycode::Up) && self.camera.rotation.y < 1.5 {
                self.camera.rotation.y += ROTATION_SPEED * dt;
            }
            if keys.contains(&Keycode::Down) && self.camera.rotation.y > -1.5 {
                self.camera.rotation.y -= ROTATION_SPEED * dt;
            }

            let mut v = Vec3 {
                x: 0.,
                y: self.camera.vel.y,
                z: 0.,
            };

            if keys.contains(&Keycode::W) {
                v = v + Vec3 {
                    x: 0.,
                    y: 0.,
                    z: SPEED,
                }
                .rotate_y(self.camera.rotation.x);
            }
            if keys.contains(&Keycode::A) {
                v = v + Vec3 {
                    x: -SPEED,
                    y: 0.,
                    z: 0.,
                }
                .rotate_y(self.camera.rotation.x);
            }
            if keys.contains(&Keycode::D) {
                v = v + Vec3 {
                    x: SPEED,
                    y: 0.,
                    z: 0.,
                }
                .rotate_y(self.camera.rotation.x);
            }
            if keys.contains(&Keycode::S) {
                v = v + Vec3 {
                    x: 0.,
                    y: 0.,
                    z: -SPEED,
                }
                .rotate_y(self.camera.rotation.x);
            }

            // add gravity
            self.camera.vel = v;
            self.camera.vel.y += GRAVITY * dt;

            //update enemies

            // collision
            let mut current_pc = BoxCollider::new(PLAYER_COLLIDER.0, PLAYER_COLLIDER.1, None);
            let mut grounded = false;
            if let Some(tag) = check_collision(
                &mut current_pc,
                &mut self.camera.pos,
                &mut self.camera.vel,
                dt,
                &cols,
                &mut grounded,
            ) {
                return match tag {
                    "goal" => Ok(level_timer.elapsed().as_secs_f64()),
                    "angry_pixel" => Err("angry_pixel"),
                    "spike" => Err("spike"),
                    t => panic!("unkown collider-tag: {t}"),
                };
            };
            if (self.camera.vel.x != 0. || self.camera.vel.z != 0.) && grounded {
                if walk.is_paused() {
                    walk.play();
                }
            } else if !walk.is_paused() {
                walk.pause();
            }

            self.camera.update_pos(dt);

            //print to when rendering is finished screen
            if let Ok(buffer) = render_thread.join() {
                self.renderer.flush(
                    &buffer,
                    false,
                    &format!("{}{}{}", &fps_text, &timer_text, &floor_text),
                );
            }

            // jump
            if grounded && keys.contains(&Keycode::Space) {
                self.camera.vel.y = -JUMP_SPEED;
                audio::play_audio(audio_handle, "./sounds/jump.mp3");
            }

            if self.camera.pos.y > GW * (floors + 10) as f64 {
                return Err("void");
            }
        }
    }
}
