extern crate rand;

use util::{TAU, PI};

pub enum Waveform {
    Sine,
    Sawtooth,
    Square,
    Noise,
}

pub struct Oscillator {
    sample_rate: f32,
    phase: f32,
    waveform: Waveform,
    norm_freq: f32,
}

impl Oscillator {
    pub fn new(sample_rate: f32) -> Oscillator {
        Oscillator {
            phase: 0.0,
            sample_rate: sample_rate,
            waveform: Waveform::Sine,
            norm_freq: TAU * 440f32 / sample_rate,
        }
    }

    pub fn tick(&mut self) {
        self.phase = (self.phase + self.norm_freq) % TAU;
    }

    pub fn sample(&self) -> f32 {
        match self.waveform {
            Waveform::Sine => self.phase.sin(),
            Waveform::Sawtooth => self.phase / PI - 1.0f32,
            Waveform::Square => if self.phase < PI { -1f32 } else { 1f32 },
            Waveform::Noise => 2f32 * rand::random::<f32>() - 1f32,
        }
    }

    pub fn fill(&mut self, buffer: &mut[f32]) {
        for out_sample in buffer.iter_mut() {
            self.tick();
            *out_sample = self.sample();
        }
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.norm_freq = TAU * freq / self.sample_rate;
    }
}

impl Iterator for Oscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.tick();
        Some(self.sample())
    }
}
