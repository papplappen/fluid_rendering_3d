const MAX_DISTANCE = 1000.;
const MAX_STEPS: f32 = 100.;
const DENSITY_THRESHOLD: f32 = .1;
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
}

// * +X = Right; +Y = Up; +Z = Back
struct Camera {
    pos: vec3<f32>,
    screen_width: f32,
    dir: vec3<f32>,
    screen_height: f32,
    up: vec3<f32>,
    screen_dist: f32,
    view_matrix: mat4x4<f32>,
    inverse_view_matrix: mat4x4<f32>
}

struct Config {
    h: f32,
    alpha: f32,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(0)
var<uniform> config: Config;

@group(2) @binding(0)
var<storage> positions: array<vec3<f32>>;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    return VertexOutput(vec4<f32>(model.position, 1.));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let ray_dir = normalize(vec3<f32>(in.clip_position.x - camera.screen_width * 0.5, -in.clip_position.y + camera.screen_height * 0.5, -camera.screen_dist));

    var ray_pos = vec3<f32>(0., 0., 0.);
    for (var i = 0u; i < 200u; i++) {
        let min_dist = min_distance(ray_pos);
        if min_dist >= MAX_DISTANCE {
            return vec4<f32>(1., 0., 0., 1.);
        }
        let density = kernel_sum(ray_pos);
        if density > DENSITY_THRESHOLD {
            // return camera.inverse_view_matrix * (vec4<f32>(normalize(log_sum_exp_grad(ray_pos, camera.view_matrix, config.alpha)), 0.0));
            return vec4<f32>(0., density, 0., 1.);
        }
        // ray_pos += ray_dir * max(min_dist - 2. * config.h, config.h * 0.1);
        ray_pos += ray_dir * config.h;
    }
    return vec4<f32>(0., 1., 0., 1.);
}
fn min_distance(ray_pos: vec3<f32>) -> f32 {
    var minimum = 1. / 0.;
    for (var i = 0u; i < arrayLength(&positions); i++) {
        let pos = (camera.view_matrix * vec4<f32>(positions[i], 1.)).xyz;
        let dist = distance(pos, ray_pos) ;
        minimum = min(minimum, dist);
    }
    return minimum;
}


fn kernel_sum(ray_pos: vec3<f32>) -> f32 {
    var sum = 0.;
    for (var i = 0u; i < arrayLength(&positions); i++) {
        let pos = (camera.view_matrix * vec4<f32>(positions[i], 1.)).xyz;
        let dist = distance(pos, ray_pos) ;
        sum += kernel(dist);
    }
    return sum;
}

fn kernel(q: f32) -> f32 {
    let t1 = max(1. - q, 0.);
    let t2 = max(2. - q, 0.);
    return config.alpha * (t2 * t2 * t2 - 4. * t1 * t1 * t1);
}
fn kernel_grad(diff: vec3<f32>) -> vec3<f32> {
    let d = diff / config.h;
    let q = length(d);
    if q > 0. {
        let t1 = max(1. - q, 0.);
        let t2 = max(2. - q, 0.);
        return config.alpha * (d / q) * (-3. * t2 * t2 + 12. * t1 * t1);
    } else {
        return vec3<f32>(0., 0., 0.);
    }
}