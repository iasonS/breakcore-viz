use bevy::prelude::*;

#[derive(Clone)]
pub struct AudioData {
    pub total_energy: f32,
    pub kick_energy: f32,
    pub mid_energy: f32,
    pub hat_energy: f32,
    pub kick_onset: bool,
    pub hat_onset: bool,
    pub chaos: f32,
    pub hue: f32,
    pub centroid: f32,
}

impl Default for AudioData {
    fn default() -> Self {
        Self {
            total_energy: 0.0,
            kick_energy: 0.0,
            mid_energy: 0.0,
            hat_energy: 0.0,
            kick_onset: false,
            hat_onset: false,
            chaos: 0.0,
            hue: 200.0,
            centroid: 0.0,
        }
    }
}

pub struct AudioAnalyzer {
    freq_data: Vec<u8>,
    wave_data: Vec<u8>,
    energy_history: Vec<f32>,
    kick_energy: f32,
    mid_energy: f32,
    hat_energy: f32,
    last_kick_onset: i32,
    last_hat_onset: i32,
}

impl AudioAnalyzer {
    pub fn new() -> Self {
        Self {
            freq_data: vec![0; 1024],
            wave_data: vec![0; 2048],
            energy_history: vec![0.0; 120],
            kick_energy: 0.0,
            mid_energy: 0.0,
            hat_energy: 0.0,
            last_kick_onset: -100,
            last_hat_onset: -100,
        }
    }

    pub fn update(&mut self, frame_num: u32) -> AudioData {
        // Simulate audio analysis
        let total_energy = (frame_num as f32 * 0.001).sin() * 0.3 + 0.2;

        self.kick_energy += (total_energy * 0.5 - self.kick_energy) * 0.2;
        self.mid_energy += (total_energy * 0.4 - self.mid_energy) * 0.18;
        self.hat_energy += (total_energy * 0.3 - self.hat_energy) * 0.15;

        self.energy_history.remove(0);
        self.energy_history.push(total_energy);

        let prev = if self.energy_history.len() > 1 {
            self.energy_history[self.energy_history.len() - 2]
        } else {
            0.0
        };

        let kick_onset = total_energy - prev > 0.15 && self.kick_energy > 0.4;
        let hat_onset = total_energy - prev > 0.1 && self.hat_energy > 0.3;

        if kick_onset {
            self.last_kick_onset = frame_num as i32;
        }
        if hat_onset {
            self.last_hat_onset = frame_num as i32;
        }

        let mut onset_count = 0;
        for i in (self.energy_history.len().saturating_sub(60))..self.energy_history.len() - 1 {
            if self.energy_history[i + 1] - self.energy_history[i] > 0.08 {
                onset_count += 1;
            }
        }

        let chaos = onset_count as f32 / 1.0;
        let hue = (total_energy * 360.0) % 360.0;

        AudioData {
            total_energy,
            kick_energy: self.kick_energy,
            mid_energy: self.mid_energy,
            hat_energy: self.hat_energy,
            kick_onset,
            hat_onset,
            chaos,
            hue,
            centroid: total_energy * 5000.0,
        }
    }
}
