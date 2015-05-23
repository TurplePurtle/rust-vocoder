
use biquad_filter;

// number of bands for the vocoder
const BANDS: usize = 30;
const FRAMES: usize = 256;

pub struct Vocoder {
    filters_c: Vec<biquad_filter::BiquadFilter>,
    buffer: [[f32; FRAMES]; BANDS],
}

impl Vocoder {
    pub fn new(sample_rate: f32) -> Vocoder {
        // initialize the filter bank
        let mut filters_c = Vec::with_capacity(BANDS);
        for _i in 0..BANDS {
            let mut filter = biquad_filter::BiquadFilter::new(sample_rate);
            filter.set_bandpass(200f32, 1.5f32);
            filters_c.push(filter);
        }

        Vocoder {
            filters_c: filters_c,
            buffer: [[0f32; FRAMES]; BANDS],
        }
    }

    pub fn generate(&mut self, output: &mut[f32], modulator: &[f32], carrier: &mut[f32]) {
        // filter the carrier into bands
        for (band, filter) in self.buffer.iter_mut().zip(self.filters_c.iter_mut()) {
            filter.filter(carrier, band);
        }

        for (output_s, buffer_s) in output.iter_mut().zip(self.buffer[0].iter()) {
            *output_s = *buffer_s;
        }
    }
}
