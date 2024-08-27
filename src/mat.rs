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
}

//

// Triangle (Tri) Implementations

impl Tri {
    pub fn normal(self) -> Vec3 {
        let a = self.v1 - self.v0;
        let b = self.v2 - self.v0;
        a.cross(b).norm()
    }

    pub fn hit(self, ro: Vec3, rd: Vec3) -> bool {
        let rd = rd.norm();
        let n = self.normal();
        let d = -n.dot(self.v0);
        if let Some(t) = self.distance(ro, rd) {
            if t < 0. {
                return false;
            }
            let p = ro + rd * t;
            return self.check_point_inside(p);
        } else {
            return false;
        }
    }

    pub fn distance(self, ro: Vec3, rd: Vec3) -> Option<f64> {
        let rd = rd.norm();
        let n = self.normal();
        let d = -n.dot(self.v0);
        if n.dot(rd) == 0. {
            return None;
        }
        Some(-(n.dot(ro) + d) / n.dot(rd))
    }

    fn check_point_inside(&self, p: Vec3) -> bool {
        let e0 = self.v1 - self.v0;
        let e1 = self.v2 - self.v1;
        let e2 = self.v0 - self.v2;

        let n = self.normal();

        let c0 = p - self.v0;
        let c1 = p - self.v1;
        let c2 = p - self.v2;
        if n.dot(e0.cross(c0)) > 0. && n.dot(e1.cross(c1)) > 0. && n.dot(e2.cross(c2)) > 0. {
            return true;
        }
        false
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
