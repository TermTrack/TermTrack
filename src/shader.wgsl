@group(0)
@binding(0)
var<storage, read_write> pixel_coords: array<vec3<f32>>;
@group(0)
@binding(1)
var<storage, read_write> colors: array<vec3<f32>>;
@group(0)
@binding(2)
var<storage, read_write> vertices: array<vec3<f32>>;
@group(0)
@binding(3)
var<storage, read_write> camera: array<f32>;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
            var pixel: vec3<f32> = pixel_coords[id.x];
            var camera_pos: vec3<f32> = vec3<f32>(camera[0],camera[1],camera[2]);
            var result_color: vec3<f32> = get_color(pixel, camera_pos);
            pixel_coords[id.x] = result_color;
}

fn get_color(pixel: vec3<f32>, camera_pos: vec3<f32>) -> vec3<f32> {
            var closest_tri_idx: u32 = 0;
            var min_dist: f32 = 0.0;
            var hit_tri: bool = false;
            var ray_dir: vec3<f32> = pixel;
            var ray_o = camera_pos;
            for (var i: u32 = 0; i < arrayLength(&vertices); i+=3u) {
                        var v0 = vertices[i];
                        var v1 = vertices[i+1];
                        var v2 = vertices[i+2];

                        var result = hit(ray_o, ray_dir, v0,v1,v2);
                        if (result.x == 1.0) {
                                    if (result.y < min_dist || !hit_tri) {
                                                min_dist = result.y;
                                                closest_tri_idx = i;
                                                hit_tri = true;
                                    }
                        }
            }
            if (hit_tri) {
                        var v0 = vertices[closest_tri_idx];
                        var v1 = vertices[closest_tri_idx + 1 ];
                        var v2 = vertices[closest_tri_idx + 2 ];
                        var color = colors[closest_tri_idx / 3];
                        var n = cross(v2-v0, v1-v0);
                        // color = color * clamp(0.5,1.0, abs(dot(n, ray_dir*(-1.0))/(length(ray_dir)* length(n))));
                        return color * (1.0 - min_dist / 45.0);
            } else {
                        return vec3<f32>(0.0,0.0,0.0);
            }
}

fn hit(ray_o: vec3<f32>, ray_dir: vec3<f32>, v0: vec3<f32>, v1: vec3<f32>, v2: vec3<f32>) -> vec2<f32> {
            var e1 = v1-v0;
            var e2 = v2-v0;
            var p = cross(ray_dir, e2);
            var det = dot(p, e1);
            if (abs(det) < 0.001) {
                        return vec2<f32>(0.0, 0.0);
            }
            var t = ray_o - v0;
            var inv_det = 1.0/det;
            var u = dot(t, p)*inv_det;
            if (u < 0.0 || u > 1.0) {
                        return vec2<f32>(0.0,0.0);
            }
            var q = cross(t, e1);
            var v = dot(ray_dir, q)*inv_det;
            if (v < 0.0 || v+u > 1.0) {
                        return vec2<f32>(0.0,0.0);
            } 
            var T = dot(e2, q) * inv_det;
            if (T < 0.0) {
                        return vec2<f32>(0.0,0.0);
            }
            return vec2<f32>(1.0, T);
}

// @compute
// @workgroup_size(1)
// fn main(@builtin(global_invocation_id) id: vec3<u32>) {
//     var pixel: vec3<f32> = pixel_coords[id.x];
//     var camera_pos: vec3<f32> = vec3<f32>(camera[0], camera[1], camera[2]);
//     var result_color: vec3<f32> = get_color(pixel, camera_pos);
//     pixel_coords[id.x] = result_color;
// }

// fn get_color(pixel: vec3<f32>, camera_pos: vec3<f32>) -> vec3<f32> {
//     var closest_tri_idx: u32 = 0;
//     var min_dist: f32 = 0.0; // Large initial value
//     var hit_tri: bool = false;
//     var ray_dir: vec3<f32> = normalize(pixel - camera_pos);  // Normalize ray direction
//     var ray_o = camera_pos;

//     for (var i: u32 = 0; i < arrayLength(&vertices); i += 3u) {  // Loop through triangles
//         var v0 = vertices[i];
//         var v1 = vertices[i + 1];
//         var v2 = vertices[i + 2];

//         var result = hit(ray_o, ray_dir, v0, v1, v2);
//         if (result.x == 1.0) {
//             if (result.y < min_dist || !hit_tri) {
//                 min_dist = result.y;
//                 closest_tri_idx = i;
//                 hit_tri = true;  // Mark hit
//             }
//         }
//     }

//     if (hit_tri) {
//         var v0 = vertices[closest_tri_idx];
//         var v1 = vertices[closest_tri_idx + 1];
//         var v2 = vertices[closest_tri_idx + 2];
//         var color = colors[closest_tri_idx / 3];
//         return color;
//     } else {
//         return vec3<f32>(0.0, 0.0, 0.0);  // Return black if no hit
//     }
// }
