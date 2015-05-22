
const BANDS: usize = 30;
const FRAMES: usize = 256;

pub struct Vocoder {
    buffer: [[f32; FRAMES]; BANDS],
}

impl Vocoder {
    pub fn new() -> Vocoder {
        Vocoder {
            buffer: [[0f32; FRAMES]; BANDS],
        }
    }

    pub fn generate(&mut self, output: &mut[f32], modulator: &[f32], carrier: &mut[f32]) {
        for band in self.buffer.iter_mut() {
            for (buffer_s, carrier_s) in band.iter_mut().zip(carrier.iter()) {
                *buffer_s = *carrier_s;
            }
        }

        for band in self.buffer.iter_mut() {
            for (buffer_s, modulator_s) in band.iter_mut().zip(modulator.iter()) {
                *buffer_s = *modulator_s;
            }
        }

        for (output_s, buffer_s) in output.iter_mut().zip(self.buffer[0].iter()) {
            *output_s = *buffer_s;
        }
    }
}
