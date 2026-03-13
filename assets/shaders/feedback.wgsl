// Feedback trail shader - recursive frame composition

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var output: VertexOutput;
    let uv = vec2<f32>(f32(vertex_index & 1u), f32((vertex_index >> 1u) & 1u));
    output.position = vec4<f32>(uv * 2.0 - 1.0, 0.0, 1.0);
    output.uv = uv;
    return output;
}

@group(0) @binding(0)
var current_tex: texture_2d<f32>;
@group(0) @binding(1)
var prev_tex: texture_2d<f32>;
@group(0) @binding(2)
var sample_samp: sampler;

@group(0) @binding(3)
var<uniform> energy: f32;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let current = textureSample(current_tex, sample_samp, input.uv).rgb;
    let prev = textureSample(prev_tex, sample_samp, input.uv).rgb;

    // Recursive zoom effect
    let centered = input.uv * 2.0 - 1.0;
    let scale = 0.965 - energy * 0.06;
    let zoomed = centered / scale * 0.5 + 0.5;

    let feedback = textureSample(prev_tex, sample_samp, zoomed).rgb * (0.52 - energy * 0.15);

    let col = current + feedback * 0.6;

    return vec4<f32>(col, 1.0);
}
