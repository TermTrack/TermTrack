use crate::mat::*;
use std::path::PathBuf;
use std::{fs, io};

pub const GW: f64 = 10.;
pub const GH: f64 = 15.;

#[derive(Clone)]
pub struct LevelMap {
    pub mesh: Mesh,
    pub colliders: Vec<BoxCollider>,
    pub start_pos: (f64, f64, f64),
    pub map_string: String,
    pub level_name: String,
}

const WALL: [[(f64, f64, f64); 8]; 6] = [
    // top face
    [
        (0., 0., 0.),
        (GW, 0., 0.),
        (GW, 0., GW),
        (207., 172., 85.),
        (0., 0., 0.),
        (0., 0., GW),
        (GW, 0., GW),
        (207., 172., 85.),
    ],
    // bottom face
    [
        (0., GH, 0.),
        (GW, GH, 0.),
        (GW, GH, GW),
        (207., 172., 85.),
        (0., GH, 0.),
        (0., GH, GW),
        (GW, GH, GW),
        (207., 172., 85.),
    ],
    // front face
    [
        (0., 0., 0.),
        (GW, 0., 0.),
        (GW, GH, 0.),
        (237., 233., 126.),
        (0., 0., 0.),
        (GW, GH, 0.),
        (0., GH, 0.),
        (237., 233., 126.),
    ],
    // back face
    [
        (0., 0., GW),
        (GW, 0., GW),
        (GW, GH, GW),
        (237., 233., 126.),
        (0., 0., GW),
        (GW, GH, GW),
        (0., GH, GW),
        (237., 233., 126.),
    ],
    //left face
    [
        (0., 0., 0.),
        (0., 0., GW),
        (0., GH, 0.),
        (237., 233., 126.),
        (0., GH, 0.),
        (0., GH, GW),
        (0., 0., GW),
        (237., 233., 126.),
    ],
    // right face
    [
        (GW, 0., 0.),
        (GW, 0., GW),
        (GW, GH, 0.),
        (237., 233., 126.),
        (GW, GH, 0.),
        (GW, GH, GW),
        (GW, 0., GW),
        (237., 233., 126.),
    ],
];

const WALL_COLLIDER: [(f64, f64, f64); 2] = [(0., GH, 0.), (GW, 0., GW)];

const HALF_WALL: [[(f64, f64, f64); 8]; 6] = [
    // top face
    [
        (0., 0.5 * GH, 0.),
        (GW, 0.5 * GH, 0.),
        (GW, 0.5 * GH, GW),
        (207., 172., 85.),
        (0., 0.5 * GH, 0.),
        (0., 0.5 * GH, GW),
        (GW, 0.5 * GH, GW),
        (207., 172., 85.),
    ],
    // bottom face
    [
        (0., GH, 0.),
        (GW, GH, 0.),
        (GW, GH, GW),
        (207., 172., 85.),
        (0., GH, 0.),
        (0., GH, GW),
        (GW, GH, GW),
        (207., 172., 85.),
    ],
    // front face
    [
        (0., 0.5 * GH, 0.),
        (GW, 0.5 * GH, 0.),
        (GW, GH, 0.),
        (237., 233., 126.),
        (0., 0.5 * GH, 0.),
        (GW, GH, 0.),
        (0., GH, 0.),
        (237., 233., 126.),
    ],
    // back face
    [
        (0., 0.5 * GH, GW),
        (GW, 0.5 * GH, GW),
        (GW, GH, GW),
        (237., 233., 126.),
        (0., 0.5 * GH, GW),
        (GW, GH, GW),
        (0., GH, GW),
        (237., 233., 126.),
    ],
    //left face
    [
        (0., 0.5 * GH, 0.),
        (0., 0.5 * GH, GW),
        (0., GH, 0.),
        (237., 233., 126.),
        (0., GH, 0.),
        (0., GH, GW),
        (0., 0.5 * GH, GW),
        (237., 233., 126.),
    ],
    // right face
    [
        (GW, 0.5 * GH, 0.),
        (GW, 0.5 * GH, GW),
        (GW, GH, 0.),
        (237., 233., 126.),
        (GW, GH, 0.),
        (GW, GH, GW),
        (GW, 0.5 * GH, GW),
        (237., 233., 126.),
    ],
];

const HALF_WALL_COLLIDER: [(f64, f64, f64); 2] = [(0., GH, 0.), (GW, 0.5 * GH, GW)];

const START: [(f64, f64, f64); 8 * 6] = [
    (0., GH * 0.9, 0.),
    (0., GH * 0.9, GW),
    (GW, GH * 0.9, 0.),
    (68., 218., 235.),
    (GW, GH * 0.9, 0.),
    (0., GH * 0.9, GW),
    (GW, GH * 0.9, GW),
    (68., 218., 235.),
    (0., GH, 0.),
    (0., GH, GW),
    (GW, GH, 0.),
    (68., 218., 235.),
    (GW, GH, 0.),
    (0., GH, GW),
    (GW, GH, GW),
    (68., 218., 235.),
    (0., GH, 0.),
    (0., GH, GW),
    (0., GH * 0.9, 0.),
    (68., 218., 235.),
    (0., GH * 0.9, 0.),
    (0., GH * 0.9, GW),
    (0., GH, GW),
    (68., 218., 235.),
    (GW, GH, 0.),
    (GW, GH, GW),
    (GW, GH * 0.9, 0.),
    (68., 218., 235.),
    (GW, GH * 0.9, 0.),
    (GW, GH * 0.9, GW),
    (GW, GH, GW),
    (68., 218., 235.),
    (0., GH, 0.),
    (GW, GH, 0.),
    (GW, GH * 0.9, 0.),
    (68., 218., 235.),
    (0., GH, 0.),
    (0., GH * 0.9, 0.),
    (GW, GH * 0.9, 0.),
    (68., 218., 235.),
    (0., GH, GW),
    (GW, GH, GW),
    (GW, GH * 0.9, GW),
    (68., 218., 235.),
    (0., GH, GW),
    (0., GH * 0.9, GW),
    (GW, GH * 0.9, GW),
    (68., 218., 235.),
];

const FLOOR_COLLIDER: [(f64, f64, f64); 2] = [(0., GH, 0.), (GW, GH * 0.9, GW)];

const FLOOR: [[(f64, f64, f64); 8]; 6] = [
    // top face
    [
        (0., GH * 0.9, 0.),
        (0., GH * 0.9, GW),
        (GW, GH * 0.9, 0.),
        (207., 172., 85.),
        (GW, GH * 0.9, 0.),
        (0., GH * 0.9, GW),
        (GW, GH * 0.9, GW),
        (207., 172., 85.),
    ],
    //bottom face
    [
        (0., GH, 0.),
        (0., GH, GW),
        (GW, GH, 0.),
        (207., 172., 85.),
        (GW, GH, 0.),
        (0., GH, GW),
        (GW, GH, GW),
        (207., 172., 85.),
    ],
    // side faces
    [
        (0., GH, 0.),
        (0., GH, GW),
        (0., GH * 0.9, 0.),
        (207., 172., 85.),
        (0., GH * 0.9, 0.),
        (0., GH * 0.9, GW),
        (0., GH, GW),
        (207., 172., 85.),
    ],
    [
        (GW, GH, 0.),
        (GW, GH, GW),
        (GW, GH * 0.9, 0.),
        (207., 172., 85.),
        (GW, GH * 0.9, 0.),
        (GW, GH * 0.9, GW),
        (GW, GH, GW),
        (207., 172., 85.),
    ],
    [
        (0., GH, 0.),
        (GW, GH, 0.),
        (GW, GH * 0.9, 0.),
        (207., 172., 85.),
        (0., GH, 0.),
        (0., GH * 0.9, 0.),
        (GW, GH * 0.9, 0.),
        (207., 172., 85.),
    ],
    [
        (0., GH, GW),
        (GW, GH, GW),
        (GW, GH * 0.9, GW),
        (207., 172., 85.),
        (0., GH, GW),
        (0., GH * 0.9, GW),
        (GW, GH * 0.9, GW),
        (207., 172., 85.),
    ],
];

const END: [(f64, f64, f64); 8 * 6] = [
    (0., GH * 0.9, 0.),
    (0., GH * 0.9, GW),
    (GW, GH * 0.9, 0.),
    (255., 0., 0.),
    (GW, GH * 0.9, 0.),
    (0., GH * 0.9, GW),
    (GW, GH * 0.9, GW),
    (255., 0., 0.),
    (0., GH, 0.),
    (0., GH, GW),
    (GW, GH, 0.),
    (102., 245., 66.),
    (GW, GH, 0.),
    (0., GH, GW),
    (GW, GH, GW),
    (102., 245., 66.),
    (0., GH, 0.),
    (0., GH, GW),
    (0., GH * 0.9, 0.),
    (235., 52., 189.),
    (0., GH * 0.9, 0.),
    (0., GH * 0.9, GW),
    (0., GH, GW),
    (235., 52., 189.),
    (GW, GH, 0.),
    (GW, GH, GW),
    (GW, GH * 0.9, 0.),
    (235., 52., 189.),
    (GW, GH * 0.9, 0.),
    (GW, GH * 0.9, GW),
    (GW, GH, GW),
    (235., 52., 189.),
    (0., GH, 0.),
    (GW, GH, 0.),
    (GW, GH * 0.9, 0.),
    (235., 52., 189.),
    (0., GH, 0.),
    (0., GH * 0.9, 0.),
    (GW, GH * 0.9, 0.),
    (235., 52., 189.),
    (0., GH, GW),
    (GW, GH, GW),
    (GW, GH * 0.9, GW),
    (235., 52., 189.),
    (0., GH, GW),
    (0., GH * 0.9, GW),
    (GW, GH * 0.9, GW),
    (235., 52., 189.),
];

const GOAL_COLLIDER: [(f64, f64, f64); 2] = [
    (GW * 0.1, GH * 0.9, GW * 0.1),
    (GW * 0.9, GH * 0.1, GW * 0.9),
];

const SPIKE_COLLIDER: [(f64, f64, f64); 2] = [
    (GW * 0.15, GH * 0.9, GW * 0.15),
    (GW * 0.85, GH * 0.7, GW * 0.85),
];
const SPIKE: [(f64, f64, f64); 4 * 4] = [
    (GW, GH * 0.9, 0.),
    (GW / 2., GH * 0.6, GW / 2.),
    (GW, GH * 0.9, GW),
    (100., 100., 100.),
    (GW, GH * 0.9, 0.),
    (GW / 2., GH * 0.6, GW / 2.),
    (0., GH * 0.9, 0.),
    (100., 100., 100.),
    (0., GH * 0.9, 0.),
    (GW / 2., GH * 0.6, GW / 2.),
    (0., GH * 0.9, GW),
    (100., 100., 100.),
    (0., GH * 0.9, GW),
    (GW / 2., GH * 0.6, GW / 2.),
    (GW, GH * 0.9, GW),
    (100., 100., 100.),
];

fn separate_map(map: &str) -> Vec<&str> {
    map.split("sep\n").collect()
}

pub fn load(path: &PathBuf) -> LevelMap {
    let mut mesh = Mesh::new([].into());
    let mut start = (0., 0., 0.);
    let mut colliders: Vec<BoxCollider> = vec![];
    let map_string = fs::read_to_string(path).expect("couldn't read level");
    let floors = separate_map(&map_string).len();
    let level_name = path.file_stem().unwrap().to_str().unwrap().to_owned();
    let maps = separate_map(&map_string)
        .iter()
        .map(|x| x.split("\n").map(|y| y.trim()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for (level, map) in maps.iter().enumerate() {
        let rows = map;
        for (z, row) in rows.iter().enumerate() {
            if row.is_empty() {
                continue;
            }
            for (x, ch) in row.chars().enumerate() {
                let mut grid = Mesh::new(vec![]);
                let mut colliders_grid: Vec<BoxCollider> = vec![];
                match ch {
                    'v' => {
                        grid = add_spike(grid, &mut colliders_grid);
                        grid = add_floor(grid, level, x, row, z, rows, &mut colliders_grid);
                    }
                    'X' => {
                        grid = add_wall(level, &maps, z, x, grid, rows, row, &mut colliders_grid);
                    }
                    'x' => {
                        grid =
                            add_half_wall(grid, level, &maps, z, x, rows, row, &mut colliders_grid);
                    }
                    '.' => {
                        grid = add_floor(grid, level, x, row, z, rows, &mut colliders_grid);
                    }

                    ' ' => {
                        continue;
                    }

                    'S' => {
                        start = (
                            z as f64 * GW + GW / 2.,
                            level as f64 * GH + GH * 0.5,
                            x as f64 * GW + GW / 2.,
                        );
                        grid = Mesh::new(Vec::from(START));
                        colliders_grid.push(BoxCollider::new(
                            FLOOR_COLLIDER[0],
                            FLOOR_COLLIDER[1],
                            None,
                        ));
                    }

                    'E' => {
                        grid = Mesh::new(Vec::from(END));
                        colliders_grid.push(BoxCollider::new(
                            FLOOR_COLLIDER[0],
                            FLOOR_COLLIDER[1],
                            None,
                        ));
                        colliders_grid.push(BoxCollider::new(
                            GOAL_COLLIDER[0],
                            GOAL_COLLIDER[1],
                            Some("goal"),
                        ))
                    }

                    c => panic!("bad map, {c}",),
                }

                // Translating grid to position
                for tri in grid.mut_tris() {
                    tri.v0 = tri.v0
                        + Vec3 {
                            x: (x as f64) * GW,
                            z: (z as f64) * GW,
                            y: -(level as f64) * GH,
                        };
                    tri.v1 = tri.v1
                        + Vec3 {
                            x: (x as f64) * GW,
                            z: (z as f64) * GW,
                            y: -(level as f64) * GH,
                        };
                    tri.v2 = tri.v2
                        + Vec3 {
                            x: (x as f64) * GW,
                            z: (z as f64) * GW,
                            y: -(level as f64) * GH,
                        };
                }

                mesh = mesh + grid;

                // Translating collider to position
                for collider in colliders_grid.iter_mut() {
                    collider.translate(Vec3 {
                        x: (x as f64) * GW,
                        z: (z as f64) * GW,
                        y: -(level as f64) * GH,
                    });
                    colliders.push(collider.clone())
                }
            }
        }
    }
    LevelMap {
        mesh,
        colliders,
        start_pos: start,
        map_string: map_string,
        level_name,
    }
}

fn add_spike(mut grid: Mesh, colliders: &mut Vec<BoxCollider>) -> Mesh {
    grid = grid + Mesh::new(Vec::from(SPIKE));
    colliders.push(BoxCollider::new(
        SPIKE_COLLIDER[0],
        SPIKE_COLLIDER[1],
        Some("death"),
    ));
    return grid;
}

fn add_wall(
    level: usize,
    maps: &Vec<Vec<&str>>,
    z: usize,
    x: usize,
    mut grid: Mesh,
    rows: &Vec<&str>,
    row: &&str,
    colliders_grid: &mut Vec<BoxCollider>,
) -> Mesh {
    // Adding the visible face
    if level == maps.len() - 1
        || ![Some('X'), Some('.'), Some('v'), Some('S'), Some('E')]
            .contains(&maps[level + 1][z].chars().nth(x))
    {
        // add top wall
        grid = grid + Mesh::new(Vec::from(WALL[0]));
    }
    if level != 0 && maps[level - 1][z].chars().nth(x) != Some('X') {
        //add under-wall
        grid = grid + Mesh::new(Vec::from(WALL[1]));
    }
    if z != 0 && rows[z - 1].chars().nth(x) != Some('X') {
        // add upper wall
        grid = grid + Mesh::new(Vec::from(WALL[2]));
    }
    if z != rows.len() - 1 && rows[z + 1].chars().nth(x) != Some('X') {
        // add bottom wall
        grid = grid + Mesh::new(Vec::from(WALL[3]));
    }
    if x != 0 && rows[z].chars().nth(x - 1) != Some('X') {
        // add left wall
        grid = grid + Mesh::new(Vec::from(WALL[4]));
    }
    if x != row.len() - 1 && rows[z].chars().nth(x + 1) != Some('X') {
        // add right wall
        grid = grid + Mesh::new(Vec::from(WALL[5]));
    }

    colliders_grid.push(BoxCollider::new(WALL_COLLIDER[0], WALL_COLLIDER[1], None));
    return grid;
}

fn add_half_wall(
    mut grid: Mesh,
    level: usize,
    maps: &Vec<Vec<&str>>,
    z: usize,
    x: usize,
    rows: &Vec<&str>,
    row: &&str,
    colliders_grid: &mut Vec<BoxCollider>,
) -> Mesh {
    // Adding the visible face
    // add top wall
    grid = grid + Mesh::new(Vec::from(HALF_WALL[0]));
    if level != 0 && maps[level - 1][z].chars().nth(x) != Some('X') {
        //add under-wall
        grid = grid + Mesh::new(Vec::from(HALF_WALL[1]));
    }

    if z != 0 && rows[z - 1].chars().nth(x) != Some('X') {
        // add upper wall
        grid = grid + Mesh::new(Vec::from(HALF_WALL[2]));
    }
    if z != rows.len() - 1 && rows[z + 1].chars().nth(x) != Some('X') {
        // add bottom wall
        grid = grid + Mesh::new(Vec::from(HALF_WALL[3]));
    }
    if x != 0 && rows[z].chars().nth(x - 1) != Some('X') {
        // add left wall
        grid = grid + Mesh::new(Vec::from(HALF_WALL[4]));
    }
    if x != row.len() - 1 && rows[z].chars().nth(x + 1) != Some('X') {
        // add right wall
        grid = grid + Mesh::new(Vec::from(HALF_WALL[5]));
    }

    colliders_grid.push(BoxCollider::new(
        HALF_WALL_COLLIDER[0],
        HALF_WALL_COLLIDER[1],
        None,
    ));
    return grid;
}

fn add_floor(
    mut grid: Mesh,
    level: usize,
    x: usize,
    row: &&str,
    z: usize,
    rows: &Vec<&str>,
    colliders_grid: &mut Vec<BoxCollider>,
) -> Mesh {
    // add floor
    // add lower section (roof)
    grid = grid + Mesh::new(Vec::from(FLOOR[0]));
    if level != 0 {
        grid = grid + Mesh::new(Vec::from(FLOOR[1]));
    }
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

    colliders_grid.push(BoxCollider::new(FLOOR_COLLIDER[0], FLOOR_COLLIDER[1], None));
    return grid;
}

// #[cfg(test)]

// mod tests {
//     use super::*;

//     #[test]

// }
