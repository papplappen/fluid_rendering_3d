struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct CameraUniform {
    view_proj: mat4x4<f32>,
}

struct Config {
    alpha: f32,
    min_dist: f32,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> config: Config;

@group(2) @binding(0)
var<storage> positions: array<vec3<f32>>;

const POINT_RADIUS = 1.;
const ASPECT_RATIO = 0.5625;
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    return VertexOutput(vec4<f32>(model.position, 1.));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    for (var i = 0u; i < arrayLength(&positions); i++) {
        if any(in.clip_position.xy <= positions[i].xy) {
            return vec4<f32>(in.clip_position.x / 1600., in.clip_position.y / 900., 1., 1.);
        }
    }
    return vec4<f32>(in.clip_position.x / 1600., in.clip_position.y / 900., 0., 1.);
}
