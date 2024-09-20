use crate::mat::*;

pub const GW: f64 = 20.;

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

// pub const MAP: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXX
// X                        X
// X S . . ..  . ..         X
// X        .     .         X
// X                        X
// X        . . .. ..       X
// X                . E     X
// X                        X
// XXXXXXXXXXXXXXXXXXXXXXXXXX";

pub const MAP: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXX
XS.......X...............X
X........................X
XXXXXX.........X   X.....X
X....XXXXX...............X
X....X...X...............X
X....X...X....XXXXXXX....X
X.....................XXXX
X.......................UX
XXXXXXXXXXXXXXXXXXXXXXXXXX
sep
XXXXXXXXXXXXXXXXXXXXXXXXXX
X          .            EX
X                        X
X.......XX     .   .     X
X    ....X    XXXXXXX    X
X    .   X               X
X    .   .    .......    X
X                     ...X
X                       DX
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

// const HOLE: [[(f64, f64, f64); 8]; 4] = [
//     // front face
//     [
//         (0., GW * 0.9, 0.),
//         (GW, GW * 0.9, 0.),
//         (GW, RENDER_DIST * 3., 0.),
//         (235., 52., 189.),
//         (0., GW * 0.9, 0.),
//         (GW, RENDER_DIST * 3., 0.),
//         (0., RENDER_DIST * 3., 0.),
//         (235., 52., 189.),
//     ],
//     // back face
//     [
//         (0., GW * 0.9, GW),
//         (GW, GW * 0.9, GW),
//         (GW, RENDER_DIST * 3., GW),
//         (235., 52., 189.),
//         (0., GW * 0.9, GW),
//         (GW, RENDER_DIST * 3., GW),
//         (0., RENDER_DIST * 3., GW),
//         (235., 52., 189.),
//     ],
//     //left face
//     [
//         (0., GW * 0.9, 0.),
//         (0., GW * 0.9, GW),
//         (0., RENDER_DIST * 3., 0.),
//         (235., 52., 189.),
//         (0., RENDER_DIST * 3., 0.),
//         (0., RENDER_DIST * 3., GW),
//         (0., GW * 0.9, GW),
//         (235., 52., 189.),
//     ],
//     // right face
//     [
//         (GW, GW * 0.9, 0.),
//         (GW, GW * 0.9, GW),
//         (GW, RENDER_DIST * 3., 0.),
//         (235., 52., 189.),
//         (GW, RENDER_DIST * 3., 0.),
//         (GW, RENDER_DIST * 3., GW),
//         (GW, GW * 0.9, GW),
//         (235., 52., 189.),
//     ],
// ];

const START: [(f64, f64, f64); 8 * 6] = [
    (0., GW * 0.9, 0.),
    (0., GW * 0.9, GW),
    (GW, GW * 0.9, 0.),
    (122., 173., 255.),
    (GW, GW * 0.9, 0.),
    (0., GW * 0.9, GW),
    (GW, GW * 0.9, GW),
    (122., 173., 255.),
    (0., GW, 0.),
    (0., GW, GW),
    (GW, GW, 0.),
    (102., 245., 66.),
    (GW, GW, 0.),
    (0., GW, GW),
    (GW, GW, GW),
    (102., 245., 66.),
    (0., GW, 0.),
    (0., GW, GW),
    (0., GW * 0.9, 0.),
    (235., 52., 189.),
    (0., GW * 0.9, 0.),
    (0., GW * 0.9, GW),
    (0., GW, GW),
    (235., 52., 189.),
    (GW, GW, 0.),
    (GW, GW, GW),
    (GW, GW * 0.9, 0.),
    (235., 52., 189.),
    (GW, GW * 0.9, 0.),
    (GW, GW * 0.9, GW),
    (GW, GW, GW),
    (235., 52., 189.),
    (0., GW, 0.),
    (GW, GW, 0.),
    (GW, GW * 0.9, 0.),
    (235., 52., 189.),
    (0., GW, 0.),
    (0., GW * 0.9, 0.),
    (GW, GW * 0.9, 0.),
    (235., 52., 189.),
    (0., GW, GW),
    (GW, GW, GW),
    (GW, GW * 0.9, GW),
    (235., 52., 189.),
    (0., GW, GW),
    (0., GW * 0.9, GW),
    (GW, GW * 0.9, GW),
    (235., 52., 189.),
];

const FLOOR_COLLIDER: [(f64, f64, f64); 2] = [(0., GW * 0.9, 0.), (GW, GW, GW)];

const FLOOR: [[(f64, f64, f64); 8]; 6] = [
    // top face
    [
        (0., GW * 0.9, 0.),
        (0., GW * 0.9, GW),
        (GW, GW * 0.9, 0.),
        (102., 245., 66.),
        (GW, GW * 0.9, 0.),
        (0., GW * 0.9, GW),
        (GW, GW * 0.9, GW),
        (102., 245., 66.),
    ],
    //bottom face
    [
        (0., GW, 0.),
        (0., GW, GW),
        (GW, GW, 0.),
        (102., 245., 66.),
        (GW, GW, 0.),
        (0., GW, GW),
        (GW, GW, GW),
        (102., 245., 66.),
    ],
    // side faces
    [
        (0., GW, 0.),
        (0., GW, GW),
        (0., GW * 0.9, 0.),
        (235., 52., 189.),
        (0., GW * 0.9, 0.),
        (0., GW * 0.9, GW),
        (0., GW, GW),
        (235., 52., 189.),
    ],
    [
        (GW, GW, 0.),
        (GW, GW, GW),
        (GW, GW * 0.9, 0.),
        (235., 52., 189.),
        (GW, GW * 0.9, 0.),
        (GW, GW * 0.9, GW),
        (GW, GW, GW),
        (235., 52., 189.),
    ],
    [
        (0., GW, 0.),
        (GW, GW, 0.),
        (GW, GW * 0.9, 0.),
        (235., 52., 189.),
        (0., GW, 0.),
        (0., GW * 0.9, 0.),
        (GW, GW * 0.9, 0.),
        (235., 52., 189.),
    ],
    [
        (0., GW, GW),
        (GW, GW, GW),
        (GW, GW * 0.9, GW),
        (235., 52., 189.),
        (0., GW, GW),
        (0., GW * 0.9, GW),
        (GW, GW * 0.9, GW),
        (235., 52., 189.),
    ],
];

const END: [(f64, f64, f64); 8 * 6] = [
    (0., GW * 0.9, 0.),
    (0., GW * 0.9, GW),
    (GW, GW * 0.9, 0.),
    (255., 0., 0.),
    (GW, GW * 0.9, 0.),
    (0., GW * 0.9, GW),
    (GW, GW * 0.9, GW),
    (255., 0., 0.),
    (0., GW, 0.),
    (0., GW, GW),
    (GW, GW, 0.),
    (102., 245., 66.),
    (GW, GW, 0.),
    (0., GW, GW),
    (GW, GW, GW),
    (102., 245., 66.),
    (0., GW, 0.),
    (0., GW, GW),
    (0., GW * 0.9, 0.),
    (235., 52., 189.),
    (0., GW * 0.9, 0.),
    (0., GW * 0.9, GW),
    (0., GW, GW),
    (235., 52., 189.),
    (GW, GW, 0.),
    (GW, GW, GW),
    (GW, GW * 0.9, 0.),
    (235., 52., 189.),
    (GW, GW * 0.9, 0.),
    (GW, GW * 0.9, GW),
    (GW, GW, GW),
    (235., 52., 189.),
    (0., GW, 0.),
    (GW, GW, 0.),
    (GW, GW * 0.9, 0.),
    (235., 52., 189.),
    (0., GW, 0.),
    (0., GW * 0.9, 0.),
    (GW, GW * 0.9, 0.),
    (235., 52., 189.),
    (0., GW, GW),
    (GW, GW, GW),
    (GW, GW * 0.9, GW),
    (235., 52., 189.),
    (0., GW, GW),
    (0., GW * 0.9, GW),
    (GW, GW * 0.9, GW),
    (235., 52., 189.),
];

fn separate_map(map: &str) -> Vec<&str> {
    map.split("sep\n").collect()
}

pub fn load(map: &str) -> (Mesh, Vec<BoxCollider>, (f64, f64)) {
    let mut mesh = Mesh::new([].into());
    let mut start = (0., 0.);
    let mut colliders: Vec<BoxCollider> = vec![];

    for (level, map) in separate_map(map).iter().enumerate() {
        let rows: Vec<&str> = map.split("\n").collect();
        for (z, row) in rows.iter().enumerate() {
            if row.is_empty() {
                continue;
            }
            for (x, ch) in row.chars().enumerate() {
                let mut grid = Mesh::new(vec![]);
                let mut collider = None;
                match ch {
                    'X' => {
                        // Adding the visible face
                        if z != 0 && rows[z - 1].chars().nth(x) != Some('X') {
                            // add upper wall
                            grid = grid + Mesh::new(Vec::from(WALL[0]));
                        }
                        if z != rows.len() - 1 && rows[z + 1].chars().nth(x) != Some('X') {
                            // add bottom wall
                            grid = grid + Mesh::new(Vec::from(WALL[1]));
                        }
                        if x != 0 && rows[z].chars().nth(x - 1) != Some('X') {
                            // add left wall
                            grid = grid + Mesh::new(Vec::from(WALL[2]));
                        }
                        if x != row.len() - 1 && rows[z].chars().nth(x + 1) != Some('X') {
                            // add right wall
                            grid = grid + Mesh::new(Vec::from(WALL[3]));
                        }
                    }

                    '.' => {
                        // add floor
                        // add lower section (roof)
                        grid = grid + Mesh::new(Vec::from(FLOOR[1]));
                        if x != 0 && row.chars().nth(x - 1) != Some('.') {
                            // add left floor wall
                            grid = grid + Mesh::new(Vec::from(FLOOR[2]));
                        }
                        if x != row.len() - 1 && row.chars().nth(x + 1) != Some('.') {
                            // add right floor wall
                            grid = grid + Mesh::new(Vec::from(FLOOR[3]));
                        }
                        if z != 0 && rows[z - 1].chars().nth(x) != Some('.') {
                            // add front floor wall
                            grid = grid + Mesh::new(Vec::from(FLOOR[4]));
                        }
                        if z != rows.len() - 1 && rows[z + 1].chars().nth(x) != Some('.') {
                            // add back floor wall
                            grid = grid + Mesh::new(Vec::from(FLOOR[5]));
                        }

                        // add collider to colliders

                        collider = Some(BoxCollider::new(FLOOR_COLLIDER[0], FLOOR_COLLIDER[1]));
                    }

                    ' ' => {
                        continue;
                    }

                    'S' => {
                        start = (z as f64 * GW + GW / 2., x as f64 * GW + GW / 2.);
                        grid = Mesh::new(Vec::from(START))
                    }

                    'E' => grid = Mesh::new(Vec::from(END)),

                    'U' => {
                        // add floor
                        grid = grid + Mesh::new(Vec::from(FLOOR[0]));
                        if level != 0 {
                            // add lower section (roof)
                            grid = grid + Mesh::new(Vec::from(FLOOR[1]));
                            if x != 0 && row.chars().nth(x - 1) != Some('.') {
                                // add left floor wall
                                grid = grid + Mesh::new(Vec::from(FLOOR[2]));
                            }
                            if x != row.len() - 1 && row.chars().nth(x + 1) != Some('.') {
                                // add right floor wall
                                grid = grid + Mesh::new(Vec::from(FLOOR[3]));
                            }
                            if z != 0 && rows[z - 1].chars().nth(x) != Some('.') {
                                // add front floor wall
                                grid = grid + Mesh::new(Vec::from(FLOOR[4]));
                            }
                            if z != rows.len() - 1 && rows[z + 1].chars().nth(x) != Some('.') {
                                // add back floor wall
                                grid = grid + Mesh::new(Vec::from(FLOOR[5]));
                            }
                        }
                    }

                    'D' => {
                        continue;
                    }

                    _ => panic!("bad map"),
                }

                // Translating grid to position
                for tri in grid.mut_tris() {
                    tri.v0 = tri.v0
                        + Vec3 {
                            x: (x as f64) * GW,
                            z: (z as f64) * GW,
                            y: -(level as f64) * GW,
                        };
                    tri.v1 = tri.v1
                        + Vec3 {
                            x: (x as f64) * GW,
                            z: (z as f64) * GW,
                            y: -(level as f64) * GW,
                        };
                    tri.v2 = tri.v2
                        + Vec3 {
                            x: (x as f64) * GW,
                            z: (z as f64) * GW,
                            y: -(level as f64) * GW,
                        };
                }

                mesh = mesh + grid;

                // Translating collider to position
                if let Some(mut t) = collider {
                    t.translate(Vec3 {
                        x: (x as f64) * GW,
                        z: (z as f64) * GW,
                        y: -(level as f64) * GW,
                    });
                    colliders.push(t)
                }
            }
        }
    }
    (mesh, colliders, start)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_load() {
        load(MAP);
    }
}
