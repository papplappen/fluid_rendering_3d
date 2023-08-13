const MAX_DISTANCE = 1000.;
const MAX_STEPS: f32 = 100.;
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
    for (var i = 0u; i < 100u; i++) {
        let min_dist = log_sum_exp(ray_pos, camera.view_matrix, config.alpha);
        if min_dist >= MAX_DISTANCE {
            return vec4<f32>(1., 0., 0., 1.);
        }
        if min_dist < config.radius * 1.01 {
            return camera.inverse_view_matrix * (vec4<f32>(normalize(log_sum_exp_grad(ray_pos, camera.view_matrix, config.alpha)), 0.0));
            // return vec4<f32>(0., 1., 0., 1.);
        }
        ray_pos += ray_dir * (min_dist - config.radius);
    }
    return vec4<f32>(0., 0., 1., 1.);
}
