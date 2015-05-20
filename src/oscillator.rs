
const TAU: f32 = 6.28318530717958647692528676655900576_f32;
const PI: f32 = TAU / 2.0f32;

pub struct Oscillator {
    phase: f32,
    sample_rate: f32,
}

impl Oscillator {
    pub fn new(sample_rate: f32) -> Oscillator {
        Oscillator {
            phase: 0.0,
            sample_rate: sample_rate,
        }
    }

    fn tick(&mut self, freq: f32) -> f32 {
        self.phase = (self.phase + TAU * freq / self.sample_rate) % TAU;
        self.phase
    }

    pub fn sine(&mut self, freq: f32) -> f32 {
        self.tick(freq).sin()
    }

    pub fn sawtooth(&mut self, freq: f32) -> f32 {
        self.tick(freq) / PI - 1.0f32
    }
}
