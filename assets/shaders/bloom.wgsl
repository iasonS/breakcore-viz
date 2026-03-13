// Bloom/glow post-processing shader

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
var scene_tex: texture_2d<f32>;
@group(0) @binding(1)
var sample_samp: sampler;

@group(0) @binding(2)
var<uniform> intensity: f32;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let texel_size = 1.0 / vec2<f32>(textureDimensions(scene_tex));
    var col = textureSample(scene_tex, sample_samp, input.uv).rgb;

    // Gaussian blur sampling
    let offsets = array<vec2<f32>, 9>(
        vec2<f32>(-1.0, -1.0), vec2<f32>(0.0, -1.0), vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0,  0.0), vec2<f32>(0.0,  0.0), vec2<f32>(1.0,  0.0),
        vec2<f32>(-1.0,  1.0), vec2<f32>(0.0,  1.0), vec2<f32>(1.0,  1.0)
    );

    let weights = array<f32, 9>(
        1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0,
        2.0 / 16.0, 4.0 / 16.0, 2.0 / 16.0,
        1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0
    );

    var bloom = vec3<f32>(0.0);
    for (var i = 0u; i < 9u; i = i + 1u) {
        let sample_uv = input.uv + offsets[i] * texel_size * 2.0;
        let sample_col = textureSample(scene_tex, sample_samp, sample_uv).rgb;
        let brightness = max(max(sample_col.r, sample_col.g), sample_col.b);

        if brightness > 0.5 {
            bloom += sample_col * weights[i] * intensity;
        }
    }

    col = col + bloom * 0.3;

    return vec4<f32>(col, 1.0);
}
