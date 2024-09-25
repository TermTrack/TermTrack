use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Tri {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub color: Vec3,
}

#[derive(Clone)]
pub struct Mesh {
    pub tris: Vec<Tri>,
}

// Vec3 implementations

impl Vec3 {
    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn abs(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn norm(self) -> Self {
        self / self.abs()
    }

    pub fn rotate(self, rotation: Vec3) -> Vec3 {
        let ret = self.rotate_z(rotation.z);
        let ret = ret.rotate_x(rotation.y);
        return ret.rotate_y(rotation.x);
    }

    pub fn rotate_x(self, angle: f64) -> Vec3 {
        let y = self.y * angle.cos() - self.z * angle.sin();
        let z = self.y * angle.sin() + self.z * angle.cos();
        Vec3 { x: self.x, y, z }
    }
    pub fn rotate_y(self, angle: f64) -> Vec3 {
        let x = self.x * angle.cos() + self.z * angle.sin();
        let z = -self.x * angle.sin() + self.z * angle.cos();
        Vec3 { x, y: self.y, z }
    }
    pub fn rotate_z(self, angle: f64) -> Vec3 {
        let x = self.x * angle.cos() - self.y * angle.sin();
        let y = self.x * angle.sin() + self.y * angle.cos();
        Vec3 { x, y, z: self.z }
    }
}

//

// Triangle (Tri) Implementations

impl Tri {
    pub fn normal(self) -> Vec3 {
        let a = self.v1 - self.v0;
        let b = self.v2 - self.v0;
        a.cross(b).norm()
    }

    pub fn hit_geo(self, ro: Vec3, rd: Vec3) -> (bool, f64) {
        let rd = rd.norm();
        let n = self.normal();

        let d = -n.dot(self.v0);
        if n.dot(rd) == 0. {
            return (false, f64::INFINITY);
        }

        let distance = -(n.dot(ro) + d) / n.dot(rd);

        if distance < 0. {
            return (false, f64::INFINITY);
        }
        let p = ro + rd * distance;

        let e0 = self.v1 - self.v0;
        let e1 = self.v2 - self.v1;
        let e2 = self.v0 - self.v2;

        let c0 = p - self.v0;
        let c1 = p - self.v1;
        let c2 = p - self.v2;
        if n.dot(e0.cross(c0)) > 0. && n.dot(e1.cross(c1)) > 0. && n.dot(e2.cross(c2)) > 0. {
            return (true, distance);
        }
        (false, f64::INFINITY)
    }

    // möller trumbore algorithm: https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection.html
    pub fn hit_mt(self, ro: Vec3, rd: Vec3) -> (bool, f64) {
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;
        let p = rd.cross(e2);
        let det = p.dot(e1);
        if det.abs() < 0.001 {
            return (false, 0.);
        }
        let t = ro - self.v0;
        let inv_det = 1. / det;
        let u = t.dot(p) * inv_det;
        if u < 0. || u > 1. {
            return (false, f64::INFINITY);
        }
        let q = t.cross(e1);
        let v = rd.dot(q) * inv_det;
        if v < 0. || v + u > 1. {
            return (false, f64::INFINITY);
        }
        let t = e2.dot(q) * inv_det;
        if t < 0. {
            return (false, f64::INFINITY);
        }
        (true, t)
    }
}

// Mesh implementation

impl Mesh {
    pub fn new(vertices: Vec<(f64, f64, f64)>) -> Self {
        let mut i = 3;
        let mut tris = vec![];
        let l = vertices.len();
        while i < l {
            let tri = Tri {
                v0: Vec3 {
                    x: vertices[i - 1].0,
                    y: vertices[i - 1].1,
                    z: vertices[i - 1].2,
                },
                v1: Vec3 {
                    x: vertices[i - 2].0,
                    y: vertices[i - 2].1,
                    z: vertices[i - 2].2,
                },
                v2: Vec3 {
                    x: vertices[i - 3].0,
                    y: vertices[i - 3].1,
                    z: vertices[i - 3].2,
                },
                color: Vec3 {
                    x: vertices[i].0,
                    y: vertices[i].1,
                    z: vertices[i].2,
                },
            };
            tris.push(tri);
            i += 4;
        }
        Mesh { tris }
    }

    pub fn tris(&self) -> Vec<Tri> {
        self.tris.clone()
    }

    pub fn gpu_buffer_data(&self) -> (Vec<[f64; 3]>, Vec<[f64; 3]>) {
        let mut verts = vec![];
        let mut colors = vec![];
        for tri in self.tris() {
            verts.push([tri.v0.x, tri.v0.y, tri.v0.z]);
            verts.push([tri.v1.x, tri.v1.y, tri.v1.z]);
            verts.push([tri.v2.x, tri.v2.y, tri.v2.z]);
            colors.push([tri.color.x, tri.color.y, tri.color.z]);
        }
        return (verts, colors);
    }

    pub fn mut_tris(&mut self) -> &mut Vec<Tri> {
        return &mut self.tris;
    }
}

impl Add<Mesh> for Mesh {
    type Output = Mesh;

    fn add(self, rhs: Mesh) -> Self::Output {
        let mut tris1 = self.tris();
        let mut tris2 = rhs.tris();

        tris1.append(&mut tris2);
        Mesh { tris: tris1 }
    }
}

// Operator implementation

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1. / rhs)
    }
}

//

#[derive(Clone)]
pub struct BoxCollider {
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub tag: Option<&'static str>,
}

impl BoxCollider {
    pub fn new(p1: (f64, f64, f64), p2: (f64, f64, f64), tag: Option<&'static str>) -> BoxCollider {
        BoxCollider {
            min_x: p1.0,
            min_y: p2.1,
            min_z: p1.2,
            max_x: p2.0,
            max_y: p1.1,
            max_z: p2.2,
            tag,
        }
    }

    pub fn intersects(&self, other: &BoxCollider) -> bool {
        let margin: f64 = 1. / 100.;
        self.min_x < other.max_x - margin
            && self.min_y < other.max_y - margin
            && self.min_z < other.max_z - margin
            && self.max_x > other.min_x + margin
            && self.max_y > other.min_y + margin
            && self.max_z > other.min_z + margin
    }

    pub fn translate(&mut self, pos: Vec3) {
        self.min_x += pos.x;
        self.max_x += pos.x;
        self.min_y += pos.y;
        self.max_y += pos.y;
        self.min_z += pos.z;
        self.max_z += pos.z;
    }
}

pub fn check_collision(
    pcollider: &mut BoxCollider,
    pos: &mut Vec3,
    vel: &mut Vec3,
    dt: f64,
    colliders: &Vec<BoxCollider>,
    grounded: &mut bool,
) -> Option<&'static str> {
    let dir = Vec3 {
        x: vel.x.signum(),
        y: vel.y.signum(),
        z: vel.z.signum(),
    };

    let mut next_pc = pcollider.clone();
    next_pc.translate(pos.clone());
    next_pc.translate(Vec3 {
        x: vel.x * dt,
        y: 0.,
        z: 0.,
    });

    let mut t = None;

    for collider in colliders.iter() {
        // checking for collision in x and fixing position
        if next_pc.intersects(collider) {
            if let Some(tag) = collider.tag {
                t = Some(tag);
                println!("{tag}");
            }
            // calculate collided distance, set position to not colliding & delete velocity in x direction
            if dir.x < 0. {
                pos.x += (collider.max_x - next_pc.min_x) + vel.x * dt;
            } else if dir.x > 0. {
                pos.x += (collider.min_x - next_pc.max_x) + vel.x * dt;
            }

            vel.x = 0.;
            next_pc = pcollider.clone();
            next_pc.translate(pos.clone());
        }
    }
    next_pc.translate(Vec3 {
        x: 0.,
        y: vel.y * dt,
        z: 0.,
    });
    for collider in colliders.iter() {
        // checking for collision in x and fixing position
        if next_pc.intersects(collider) {
            if let Some(tag) = collider.tag {
                t = Some(tag);
                println!("{tag}");
            }
            // calculate collided distance, set position to not colliding & delete velocity in x direction
            if dir.y < 0. {
                pos.y += (collider.max_y - next_pc.min_y) + vel.y * dt;
            } else if dir.y > 0. {
                pos.y += (collider.min_y - next_pc.max_y) + vel.y * dt;
                *grounded = true;
            }

            vel.y = 0.;
            next_pc = pcollider.clone();
            next_pc.translate(pos.clone());
        }
    }
    next_pc.translate(Vec3 {
        x: 0.,
        y: 0.,
        z: vel.z * dt,
    });
    for collider in colliders.iter() {
        // checking for collision in x and fixing position
        if next_pc.intersects(collider) {
            if let Some(tag) = collider.tag {
                t = Some(tag);
                println!("{tag}");
            }
            // calculate collided distance, set position to not colliding & delete velocity in x direction
            if dir.z < 0. {
                pos.z += (collider.max_z - next_pc.min_z) + vel.z * dt;
            } else if dir.z > 0. {
                pos.z += (collider.min_z - next_pc.max_z) + vel.z * dt;
            }

            vel.z = 0.;
            next_pc = pcollider.clone();
            next_pc.translate(pos.clone());
        }
    }

    t
}
