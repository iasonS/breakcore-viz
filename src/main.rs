use bevy::prelude::*;

mod audio;
mod particles;
mod renderer;
mod effects;

use audio::{AudioAnalyzer, AudioData};
use particles::ParticleSystem;
use renderer::BreakCoreRenderer;
use effects::{GlitchEngine, BloomPass, FeedbackTrail};

#[derive(Component)]
struct ParticleVisual;

#[derive(Component)]
struct FormCore;

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
        .add_systems(Update, (
            update_audio_and_particles,
            update_effects,
            render_visualization,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    });

    // Directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(1.0, 1.0, 1.0),
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Core form (will be updated each frame)
    // Create a simple box mesh - can be replaced with raymarched form later
    let core_mesh = meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0)));
    let core_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 0.2, 1.0, 0.8),
        emissive: Color::srgb(0.0, 0.1, 0.5).into(),
        ..default()
    });

    commands.spawn((
        FormCore,
        PbrBundle {
            mesh: core_mesh,
            material: core_material,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
}

fn update_audio_and_particles(mut state: ResMut<VisualizerState>) {
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
}

fn update_effects(mut state: ResMut<VisualizerState>) {
    // Update effect engines
    if state.audio_data.kick_onset {
        state.glitch_engine.trigger(0.8);
    }
    if state.audio_data.chaos > 3.0 {
        state.glitch_engine.trigger(state.audio_data.chaos * 0.2);
    }

    state.glitch_engine.update();
}

fn render_visualization(
    state: Res<VisualizerState>,
    mut query: Query<&mut Transform, With<FormCore>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut form_query: Query<&mut Handle<StandardMaterial>, With<FormCore>>,
) {
    // Update core form scale and color based on audio
    if let Ok(mut transform) = query.get_single_mut() {
        let scale = 0.5 * (1.8 + state.audio_data.kick_energy * 1.7);
        transform.scale = Vec3::splat(scale);
    }

    // Update core form color
    if let Ok(material_handle) = form_query.get_single_mut() {
        if let Some(material) = materials.get_mut(material_handle.as_ref()) {
            let (r, g, b) = if state.audio_data.kick_onset {
                (1.0, 0.0, 1.0)
            } else {
                let energy = state.audio_data.total_energy.min(1.0);
                (energy * 0.8, 0.0, energy)
            };

            material.base_color = Color::srgba(r, g, b, 0.8);
            material.emissive = Color::srgb(r * 0.5, g * 0.5, b * 0.5).into();
        }
    }
}
