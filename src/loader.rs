use crate::mat::*;
use crate::renderer::RENDER_DIST;

const GW: f64 = 15.;

// pub const MAP: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXX
// XS.......X...............X
// X........................X
// XXXXXX.........X...X.....X
// X....XXXXX...............X
// X....X...X...............X
// X....X...X....XXXXXXX....X
// X........................X
// X.......................EX
// XXXXXXXXXXXXXXXXXXXXXXXXXX";

pub const MAP: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXX
X                        X
X S . . ..  . ..         X
X        .     .         X
X                        X
X        . . .. ..       X
X                . E     X
X                        X
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

const FLOOR: [(f64, f64, f64); 8] = [
    (0., GW, 0.),
    (0., GW, GW),
    (GW, GW, 0.),
    (102., 245., 66.),
    (GW, GW, 0.),
    (0., GW, GW),
    (GW, GW, GW),
    (102., 245., 66.),
];

const HOLE: [[(f64, f64, f64); 8]; 4] = [
    // front face
    [
        (0., GW, 0.),
        (GW, GW, 0.),
        (GW, RENDER_DIST * 3., 0.),
        (235., 52., 189.),
        (0., GW, 0.),
        (GW, RENDER_DIST * 3., 0.),
        (0., RENDER_DIST * 3., 0.),
        (235., 52., 189.),
    ],
    // back face
    [
        (0., GW, GW),
        (GW, GW, GW),
        (GW, RENDER_DIST * 3., GW),
        (235., 52., 189.),
        (0., GW, GW),
        (GW, RENDER_DIST * 3., GW),
        (0., RENDER_DIST * 3., GW),
        (235., 52., 189.),
    ],
    //left face
    [
        (0., GW, 0.),
        (0., GW, GW),
        (0., RENDER_DIST * 3., 0.),
        (235., 52., 189.),
        (0., RENDER_DIST * 3., 0.),
        (0., RENDER_DIST * 3., GW),
        (0., GW, GW),
        (235., 52., 189.),
    ],
    // right face
    [
        (GW, GW, 0.),
        (GW, GW, GW),
        (GW, RENDER_DIST * 3., 0.),
        (235., 52., 189.),
        (GW, RENDER_DIST * 3., 0.),
        (GW, RENDER_DIST * 3., GW),
        (GW, GW, GW),
        (235., 52., 189.),
    ],
];

const START: [(f64, f64, f64); 8] = [
    (0., GW, 0.),
    (0., GW, GW),
    (GW, GW, 0.),
    (122., 173., 255.),
    (GW, GW, 0.),
    (0., GW, GW),
    (GW, GW, GW),
    (122., 173., 255.),
];

const END: [(f64, f64, f64); 8] = [
    (0., GW, 0.),
    (0., GW, GW),
    (GW, GW, 0.),
    (255., 0., 0.),
    (GW, GW, 0.),
    (0., GW, GW),
    (GW, GW, GW),
    (255., 0., 0.),
];

pub fn load(map: &str) -> Mesh {
    let rows: Vec<&str> = map.split("\n").collect();
    let mut mesh = Mesh::new([].into());

    for (z, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let mut grid = Mesh::new(vec![]);
            match ch {
                'X' => {
                    // Adding the visible face
                    if z != 0 && rows[z - 1].chars().nth(x).unwrap() != 'X' {
                        // add upper wall
                        grid = grid + Mesh::new(Vec::from(WALL[0]));
                    }
                    if z != rows.len() - 1 && rows[z + 1].chars().nth(x).unwrap() != 'X' {
                        // add bottom wall
                        grid = grid + Mesh::new(Vec::from(WALL[1]));
                    }
                    if x != 0 && rows[z].chars().nth(x - 1).unwrap() != 'X' {
                        // add left wall
                        grid = grid + Mesh::new(Vec::from(WALL[2]));
                    }
                    if x != row.len() - 1 && rows[z].chars().nth(x + 1).unwrap() != 'X' {
                        // add right wall
                        grid = grid + Mesh::new(Vec::from(WALL[3]));
                    }
                }

                '.' => grid = Mesh::new(Vec::from(FLOOR)),

                ' ' => {
                    // Adding the visible face
                    if z != 0 && rows[z - 1].chars().nth(x).unwrap() != ' ' {
                        // add upper wall
                        grid = grid + Mesh::new(Vec::from(HOLE[0]));
                    }
                    if z != rows.len() - 1 && rows[z + 1].chars().nth(x).unwrap() != ' ' {
                        // add bottom wall
                        grid = grid + Mesh::new(Vec::from(HOLE[1]));
                    }
                    if x != 0 && rows[z].chars().nth(x - 1).unwrap() != ' ' {
                        // add left wall
                        grid = grid + Mesh::new(Vec::from(HOLE[2]));
                    }
                    if x != row.len() - 1 && rows[z].chars().nth(x + 1).unwrap() != ' ' {
                        // add right wall
                        grid = grid + Mesh::new(Vec::from(HOLE[3]));
                    }
                }

                'S' => grid = Mesh::new(Vec::from(START)),

                'E' => grid = Mesh::new(Vec::from(END)),

                _ => panic!("bad map"),
            }

            // Translating grid to position
            for tri in grid.mut_tris() {
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

            mesh = mesh + grid;
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
