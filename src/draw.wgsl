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

const POINT_RADIUS = 1.;
const ASPECT_RATIO = 0.5625;
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    return VertexOutput(vec4<f32>(model.position, 0.));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0., 0., 0., 0.);
}
