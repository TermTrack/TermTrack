use crate::mat::*;

const GW: f64 = 30.;

pub const MAP: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXX
XS...........X...........X
X............X...........X
XXXXXX.......X...........X
X............X...........X
XXXX       XXX...........X
X........................X
X........................X
X.......................EX
XXXXXXXXXXXXXXXXXXXXXXXXXX";

const WALL: [[(f64, f64, f64); 8]; 4] = [
    // front face
    [
        (0., 0., 0.),
        (GW, 0., 0.),
        (GW, GW, 0.),
        (235., 52., 189.),
        (0., 0., 0.),
        (GW, GW, 0.),
        (0., GW, 0.),
        (235., 52., 189.),
    ],
    // back face
    [
        (0., 0., GW),
        (GW, 0., GW),
        (GW, GW, GW),
        (235., 52., 189.),
        (0., 0., GW),
        (GW, GW, GW),
        (0., GW, GW),
        (235., 52., 189.),
    ],
    //left face
    [
        (0., 0., 0.),
        (0., 0., GW),
        (0., GW, 0.),
        (235., 52., 189.),
        (0., GW, 0.),
        (0., GW, GW),
        (0., 0., GW),
        (235., 52., 189.),
    ],
    // right face
    [
        (GW, 0., 0.),
        (GW, 0., GW),
        (GW, GW, 0.),
        (235., 52., 189.),
        (GW, GW, 0.),
        (GW, GW, GW),
        (GW, 0., GW),
        (235., 52., 189.),
    ],
];

pub fn load(map: &str) -> Mesh {
    let rows: Vec<&str> = map.split("\n").collect();
    let mut mesh = Mesh::new([].into());

    for (z, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            match ch {
                'X' => {
                    let mut wall = Mesh::new(vec![]);

                    // Adding the visible face
                    if z != 0 && rows[z - 1].chars().nth(x).unwrap() != 'X' {
                        // add upper wall
                        wall = wall + Mesh::new(Vec::from(WALL[0]));
                    }
                    if z != rows.len() - 1 && rows[z + 1].chars().nth(x).unwrap() != 'X' {
                        // add bottom wall
                        wall = wall + Mesh::new(Vec::from(WALL[1]));
                    }
                    if x != 0 && rows[z].chars().nth(x - 1).unwrap() != 'X' {
                        // add left wall
                        wall = wall + Mesh::new(Vec::from(WALL[2]));
                    }
                    if x != row.len() - 1 && rows[z].chars().nth(x + 1).unwrap() != 'X' {
                        // add right wall
                        wall = wall + Mesh::new(Vec::from(WALL[3]));
                    }

                    // Translating wall to position
                    for tri in wall.mut_tris() {
                        tri.v0 = tri.v0
                            + Vec3 {
                                x: (x as f64) * GW,
                                z: (z as f64) * GW,
                                y: 0.,
                            };
                        tri.v1 = tri.v1
                            + Vec3 {
                                x: (x as f64) * GW,
                                z: (z as f64) * GW,
                                y: 0.,
                            };
                        tri.v2 = tri.v2
                            + Vec3 {
                                x: (x as f64) * GW,
                                z: (z as f64) * GW,
                                y: 0.,
                            };
                    }
                    mesh = mesh + wall;
                }
                '.' => (),
                ' ' => (),
                'S' => (),
                'E' => (),
                _ => panic!("bad map"),
            }
        }
    }
    mesh
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_load() {
        load(MAP);
    }
}
