use bevy::prelude::*;

mod audio;
mod particles;
mod renderer;
mod effects;

use audio::{AudioAnalyzer, AudioData};
use particles::ParticleSystem;
use renderer::BreakCoreRenderer;
use effects::{GlitchEngine, BloomPass, FeedbackTrail};

#[derive(Resource)]
struct VisualizerState {
    time: f32,
    frame_num: u32,
    particles: ParticleSystem,
    audio_analyzer: AudioAnalyzer,
    audio_data: AudioData,
    renderer: BreakCoreRenderer,
    glitch_engine: GlitchEngine,
    bloom_pass: BloomPass,
    feedback_trail: FeedbackTrail,
}

impl Default for VisualizerState {
    fn default() -> Self {
        Self {
            time: 0.0,
            frame_num: 0,
            particles: ParticleSystem::new(),
            audio_analyzer: AudioAnalyzer::new(),
            audio_data: AudioData::default(),
            renderer: BreakCoreRenderer::new(),
            glitch_engine: GlitchEngine::new(1280, 720),
            bloom_pass: BloomPass::new(1280, 720),
            feedback_trail: FeedbackTrail::new(1280, 720),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "BREAKCORE VISUALIZER".to_string(),
                resolution: bevy::window::WindowResolution::new(1280.0, 720.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(VisualizerState::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update_loop)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    });
}

fn update_loop(
    mut state: ResMut<VisualizerState>,
    mut gizmos: Gizmos,
) {
    const DELTA_TIME: f32 = 0.016;

    state.time += DELTA_TIME;
    state.frame_num += 1;

    // Update audio analysis
    state.audio_data = state.audio_analyzer.update(state.frame_num);

    // Spawn particles based on audio events
    state.particles.spawn(&state.audio_data, state.frame_num);

    // Update particle physics
    state.particles.update(DELTA_TIME);

    // Update renderer with audio data
    state.renderer.render(&state.audio_data);

    // Update effect engines
    if state.audio_data.kick_onset {
        state.glitch_engine.trigger(0.8);
    }
    if state.audio_data.chaos > 3.0 {
        state.glitch_engine.trigger(state.audio_data.chaos * 0.2);
    }

    state.glitch_engine.update();

    // Render particles as visual representation
    for particle in &state.particles.particles {
        let hsl_to_rgb = hsl_to_rgb(particle.h, particle.s, particle.l);
        let color = Color::srgb(hsl_to_rgb.0, hsl_to_rgb.1, hsl_to_rgb.2);

        // Draw particle as sphere gizmo
        gizmos.sphere(
            particle.pos,
            Quat::IDENTITY,
            particle.size * 0.01,
            color,
        );

        // Draw trail
        for i in 0..particle.trail.len().saturating_sub(1) {
            let trail_color = Color::srgba(
                hsl_to_rgb.0,
                hsl_to_rgb.1,
                hsl_to_rgb.2,
                (i as f32 / particle.trail.len() as f32) * particle.life,
            );
            gizmos.line(particle.trail[i], particle.trail[i + 1], trail_color);
        }
    }

    // Render core form visualization
    let form_scale = 1.8 + state.audio_data.kick_energy * 1.7;
    let form_color = if state.audio_data.kick_onset {
        Color::srgb(1.0, 0.0, 1.0)
    } else {
        let energy_fade = state.audio_data.total_energy.min(1.0);
        Color::srgb(energy_fade * 0.8, 0.0, energy_fade)
    };

    gizmos.sphere(Vec3::ZERO, Quat::IDENTITY, form_scale * 0.5, form_color);
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let h = h % 360.0;
    let s = (s / 100.0).clamp(0.0, 1.0);
    let l = (l / 100.0).clamp(0.0, 1.0);

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());

    let (r1, g1, b1) = match h_prime as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let m = l - c / 2.0;
    ((r1 + m).clamp(0.0, 1.0), (g1 + m).clamp(0.0, 1.0), (b1 + m).clamp(0.0, 1.0))
}
