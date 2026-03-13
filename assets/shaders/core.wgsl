// Core raymarching shader for breakcore visualizer
// Outputs twisted, morphing 3D form reactive to audio

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

struct AudioData {
    total_energy: f32,
    kick_energy: f32,
    hat_energy: f32,
    chaos: f32,
    hue: f32,
    time: f32,
}

@group(0) @binding(0)
var<uniform> audio: AudioData;

@group(0) @binding(1)
var<uniform> feedback_tex: texture_2d<f32>;

const STEPS: i32 = 120;
const MAX_DIST: f32 = 70.0;
const SURF: f32 = 0.0055;

fn sdf(p: vec3<f32>) -> f32 {
    let scale = 1.8 + audio.kick_energy * 1.7;
    let twist = audio.time * 4.0 + audio.total_energy * 7.0 + audio.chaos * 6.0;
    let convulse = sin(twist * 14.0) * audio.chaos * 0.7;

    var pp = p;
    pp.x += convulse + sin(pp.y * 4.0 + twist) * audio.total_energy * 0.8;
    pp.y += sin(pp.z * 3.5 + twist * 1.0) * audio.total_energy * 0.7;
    pp.z += cos(pp.x * 3.0 + twist * 1.4) * audio.total_energy * 0.7;

    let r = length(pp.xy) - 1.5;
    let y = pp.z - 0.7 * sin(atan2(pp.y, pp.x) * 7.0 + twist);
    var dist = sqrt(r * r + y * y) - 0.18;

    dist += 0.25 * sin(pp.x * 16.0 + twist) * sin(pp.y * 16.0) * sin(pp.z * 16.0) * audio.total_energy;
    dist += 0.12 * sin(pp.x * 35.0) * sin(pp.y * 35.0) * sin(pp.z * 35.0) * audio.chaos;

    return dist * scale;
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> vec3<f32> {
    let h_norm = fract(h / 360.0);
    let s_norm = s / 100.0;
    let l_norm = l / 100.0;

    let a = s_norm * min(l_norm, 1.0 - l_norm);
    let f = h_norm * 6.0;
    let c = (1.0 - abs(fract(f / 2.0) * 2.0 - 1.0)) * a;
    let m = l_norm - a / 2.0;

    if h_norm < 1.0 / 6.0 {
        return vec3<f32>(a + m, c + m, m);
    } else if h_norm < 2.0 / 6.0 {
        return vec3<f32>(c + m, a + m, m);
    } else if h_norm < 3.0 / 6.0 {
        return vec3<f32>(m, a + m, c + m);
    } else if h_norm < 4.0 / 6.0 {
        return vec3<f32>(m, c + m, a + m);
    } else if h_norm < 5.0 / 6.0 {
        return vec3<f32>(c + m, m, a + m);
    } else {
        return vec3<f32>(a + m, m, c + m);
    }
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    var uv2 = input.uv * 2.0 - 1.0;
    uv2.x *= 1.777; // Aspect ratio

    let shake = vec3<f32>(
        sin(audio.time * 60.0) * audio.kick_energy * 1.2 + sin(audio.time * 20.0) * audio.chaos * 0.3,
        cos(audio.time * 50.0) * audio.kick_energy * 0.9 + cos(audio.time * 16.0) * audio.chaos * 0.25,
        sin(audio.time * 35.0) * audio.chaos * 0.25
    );

    let ro = vec3<f32>(0.0, 0.0, 5.0 + audio.total_energy * 2.8 + audio.chaos * 1.2) + shake;
    let rd = normalize(vec3<f32>(uv2, -1.0));

    var dist = 0.0;
    var hit = false;
    var p = ro;

    for (var i = 0; i < STEPS; i = i + 1) {
        p = ro + rd * dist;
        let d = sdf(p);
        if d < SURF {
            hit = true;
            break;
        }
        dist += d * 0.26;
        if dist > MAX_DIST {
            break;
        }
    }

    var col = vec3<f32>(0.0);

    if hit {
        let kick_col = hsl_to_rgb(310.0, 100.0, 50.0);
        let hue_col = hsl_to_rgb(audio.hue, 100.0, 58.0);
        let hat_col = hsl_to_rgb(fract(audio.hue / 360.0 + 0.5) * 360.0, 100.0, 60.0);

        let surface_col = mix(mix(kick_col, hue_col, audio.hat_energy * 0.7), hat_col, audio.chaos * 0.4);

        let glow = 2.0 / (1.0 + dist * dist * 0.07);
        let glow_mod = glow + audio.kick_energy * 0.8 + audio.hat_energy * 0.7;

        col = surface_col * glow_mod;
        col += kick_col * audio.kick_energy * 1.2;
        col += hue_col * audio.hat_energy * 0.8;
        col += (kick_col + hue_col + hat_col) * 0.4 * glow;
    }

    // Scanlines
    let scan = select(0.7 - audio.chaos * 0.2, 1.0, (u32(i32(input.position.y) % 2) == 0u));
    col *= scan;

    // Vignette
    let v = length(uv2) * (0.35 + audio.chaos * 0.3);
    col *= (1.0 - v * v * (0.45 + audio.total_energy * 0.5));

    return vec4<f32>(col, 1.0);
}
