use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use rodio::OutputStreamHandle;

use rodio::{source::Source, Decoder, OutputStream};

use crate::audio;
use crate::loader::{self, load};
use crate::renderer::{self, *};
use crate::GH;
use crate::GW;
use crate::{camera::Camera, mat::*};
use core::panic;
use std::thread;
use std::time::{Duration, Instant};

pub struct Game {
    pub renderer: Screen,
    pub camera: Camera,
}

const SPEED: f64 = 50.;
const ROTATION_SPEED: f64 = 1.5;
const GRAVITY: f64 = 140.;
const PLAYER_WIDTH: f64 = 1.;
const PLAYER_COLLIDER: ((f64, f64, f64), (f64, f64, f64)) = ((-1., 6., -1.), (1., -2., 1.));

impl Game {
    pub fn run(
        &mut self,
        map: (Mesh, Vec<BoxCollider>, (f64, f64, f64), String),
        audio_handle: &OutputStreamHandle,
    ) -> Result<f64, &str> {
        // load map files
        // generate map meshes

        let (mesh, colliders, start, map_string) = map;

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
        let (_stream, background_audio_handle) = OutputStream::try_default().unwrap();
        audio::play_audio(&background_audio_handle, "./sounds/background.mp3");

        loop {
            // reset timer
            let dt = time.elapsed().as_secs_f64();
            time = Instant::now();

            // get list off all vertices

            let fps_text = &format!("fps: {:.2?} ", 1. / (dt));
            let timer_text = &format!("time: {:.1?} ", level_timer.elapsed());
            let floor_text = &format!(
                "floor: {}/{}",
                (-self.camera.pos.y.div_euclid(GH)).clamp(0., floors as f64) as usize,
                floors
            );

            // render vertices
            self.renderer.render_mt(
                &self.camera,
                &mesh,
                &format!("{}{}{}", fps_text, timer_text, floor_text),
                true,
            );

            // handle input
            let mouse = device_state.get_mouse();
            let keys = device_state.get_keys();

            if keys.contains(&Keycode::E) {
                let _ = crossterm::terminal::disable_raw_mode().unwrap();

                let mut stdout = std::io::stdout();
                crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen).unwrap();
                panic!("Exited app")
            }
            if keys.contains(&Keycode::M) {
                let time1 = time.elapsed();
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
            }
            if keys.contains(&Keycode::Left) {
                self.camera.rotation.x -= ROTATION_SPEED * dt;
            }
            if keys.contains(&Keycode::Right) {
                self.camera.rotation.x += ROTATION_SPEED * dt;
            }
            if keys.contains(&Keycode::Up) {
                if self.camera.rotation.y < 1.5 {
                    self.camera.rotation.y += ROTATION_SPEED * dt;
                }
            }
            if keys.contains(&Keycode::Down) {
                if self.camera.rotation.y > -1.5 {
                    self.camera.rotation.y -= ROTATION_SPEED * dt;
                }
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

            // collision
            let mut current_pc = BoxCollider::new(PLAYER_COLLIDER.0, PLAYER_COLLIDER.1, None);
            let mut grounded = false;
            if let Some(tag) = check_collision(
                &mut current_pc,
                &mut self.camera.pos,
                &mut self.camera.vel,
                dt,
                &colliders,
                &mut grounded,
            ) {
                return match tag {
                    "goal" => Ok(level_timer.elapsed().as_secs_f64()),
                    "death" => Err("death"),
                    t => panic!("unkown collider-tag: {t}"),
                };
            };

            // jump
            if grounded && keys.contains(&Keycode::Space) {
                self.camera.vel.y = -40.;
                audio::play_audio(audio_handle, "./sounds/jump.mp3");
            }
            self.camera.update_pos(dt);
            if self.camera.pos.y > GW * (floors + 1) as f64 {
                return Err("death");
            }
        }
    }
}
