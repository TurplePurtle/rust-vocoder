
use util::{TAU, PI};

pub enum Waveform {
    Sine,
    Sawtooth,
}

pub struct Oscillator {
    sample_rate: f32,
    phase: f32,
    waveform: Waveform,
    frequency: f32,
}

impl Oscillator {
    pub fn new(sample_rate: f32) -> Oscillator {
        Oscillator {
            phase: 0.0,
            sample_rate: sample_rate,
            waveform: Waveform::Sine,
            frequency: 440f32,
        }
    }

    pub fn tick(&mut self) {
        self.phase = (self.phase + TAU * self.frequency / self.sample_rate) % TAU;
    }

    pub fn sample(&self) -> f32 {
        match self.waveform {
            Waveform::Sine => self.phase.sin(),
            Waveform::Sawtooth => self.phase / PI - 1.0f32,
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
        self.frequency = freq;
    }
}

impl Iterator for Oscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.tick();
        Some(self.sample())
    }
}
