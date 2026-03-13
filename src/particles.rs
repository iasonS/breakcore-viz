use bevy::prelude::*;
use rand::Rng;

#[derive(Clone)]
pub struct Particle {
    pub pos: Vec3,
    pub vel: Vec3,
    pub life: f32,
    pub decay: f32,
    pub h: f32,
    pub s: f32,
    pub l: f32,
    pub size: f32,
    pub trail: Vec<Vec3>,
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    max_particles: usize,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            max_particles: 3000,
        }
    }

    pub fn spawn(&mut self, audio_data: &crate::audio::AudioData, _frame_num: u32) {
        let mut rng = rand::thread_rng();

        // Kick particles
        if audio_data.kick_onset {
            for _ in 0..80 {
                let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
                let elev = rng.gen_range(-std::f32::consts::PI / 2.0..std::f32::consts::PI / 2.0);
                let speed = 8.0 + rng.gen::<f32>() * 14.0 + audio_data.kick_energy * 6.0;

                if self.particles.len() < self.max_particles {
                    self.particles.push(Particle {
                        pos: Vec3::ZERO,
                        vel: Vec3::new(
                            angle.cos() * elev.cos() * speed,
                            elev.sin() * speed,
                            angle.sin() * elev.cos() * speed,
                        ),
                        life: 1.0,
                        decay: 0.014,
                        h: 310.0,
                        s: 100.0,
                        l: 50.0,
                        size: 4.5 + rng.gen::<f32>() * 3.5 + audio_data.kick_energy * 2.5,
                        trail: Vec::new(),
                    });
                }
            }
        }

        // Hat particles
        if audio_data.hat_onset {
            for _ in 0..60 {
                let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
                let elev = rng.gen_range(-std::f32::consts::PI / 2.0..std::f32::consts::PI / 2.0);
                let speed = 6.0 + rng.gen::<f32>() * 11.0 + audio_data.hat_energy * 4.0;

                if self.particles.len() < self.max_particles {
                    self.particles.push(Particle {
                        pos: Vec3::new(
                            (rng.gen::<f32>() - 0.5) * 0.5,
                            (rng.gen::<f32>() - 0.5) * 0.5,
                            (rng.gen::<f32>() - 0.5) * 0.5,
                        ),
                        vel: Vec3::new(
                            angle.cos() * elev.cos() * speed,
                            elev.sin() * speed,
                            angle.sin() * elev.cos() * speed,
                        ),
                        life: 1.0,
                        decay: 0.012,
                        h: audio_data.hue,
                        s: 100.0,
                        l: 60.0,
                        size: 3.5 + rng.gen::<f32>() * 2.5 + audio_data.hat_energy * 2.0,
                        trail: Vec::new(),
                    });
                }
            }
        }

        // Peak particles
        if audio_data.total_energy > 0.65 {
            for _ in 0..30 {
                let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
                let speed = 9.0 + rng.gen::<f32>() * 16.0;

                if self.particles.len() < self.max_particles {
                    self.particles.push(Particle {
                        pos: Vec3::new(
                            (rng.gen::<f32>() - 0.5) * 1.5,
                            (rng.gen::<f32>() - 0.5) * 1.5,
                            (rng.gen::<f32>() - 0.5) * 1.5,
                        ),
                        vel: Vec3::new(
                            angle.cos() * speed,
                            (rng.gen::<f32>() - 0.5) * speed * 2.8,
                            angle.sin() * speed,
                        ),
                        life: 0.9,
                        decay: 0.016,
                        h: (audio_data.hue + 180.0) % 360.0,
                        s: 100.0,
                        l: 65.0,
                        size: 3.0 + rng.gen::<f32>() * 2.5 + audio_data.total_energy * 2.5,
                        trail: Vec::new(),
                    });
                }
            }
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        for i in (0..self.particles.len()).rev() {
            let p = &mut self.particles[i];

            p.trail.push(p.pos);
            if p.trail.len() > 10 {
                p.trail.remove(0);
            }

            p.pos += p.vel * delta_time;
            p.vel *= 0.93;
            p.life -= p.decay;

            if p.life <= 0.0 {
                self.particles.swap_remove(i);
            }
        }
    }
}
