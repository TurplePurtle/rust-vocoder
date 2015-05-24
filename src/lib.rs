
mod util;
pub mod oscillator;
mod biquad_filter;

use biquad_filter::BiquadFilter;

// number of bands for the vocoder
const BANDS: usize = 30;
const FRAMES: usize = 256;

pub struct Vocoder {
    carrier_filters: Vec<BiquadFilter>,
    modulator_filters: Vec<BiquadFilter>,
    carrier_filtered: [[f32; FRAMES]; BANDS],
    modulator_filtered: [[f32; FRAMES]; BANDS],
}

impl Vocoder {
    pub fn new(sample_rate: f32) -> Vocoder {
        Vocoder {
            carrier_filters: Vocoder::init_filterbank(sample_rate),
            modulator_filters: Vocoder::init_filterbank(sample_rate),
            carrier_filtered: [[0f32; FRAMES]; BANDS],
            modulator_filtered: [[0f32; FRAMES]; BANDS],
        }
    }

    // make a constant-Q filter bank of the given range
    // TODO don't hardcode the range
    fn init_filterbank(sample_rate: f32) -> Vec<BiquadFilter> {
        let mut carrier_filters = Vec::with_capacity(BANDS);
        let bands = BANDS;
        let range = 200f32..5000f32;
        let log_range = range.start.ln()..range.end.ln();
        let log_delta = (log_range.end - log_range.start) / bands as f32;
        for i in 0..bands {
            let from = (log_range.start + log_delta * i as f32).exp();
            let to = (log_range.start + log_delta * (i+1) as f32).exp();
            let fc = (to + from) / 2f32;
            let q = fc / (to - from);  // about the same for all bands

            let mut filter = BiquadFilter::new(sample_rate);
            filter.set_bandpass(fc, 2f32 * q);
            carrier_filters.push(filter);
        }
        carrier_filters
    }

    pub fn generate(&mut self, output: &mut[f32], modulator: &[f32], carrier: &mut[f32]) {
        // filter the modulator into bands
        for (modulator_band, filter) in self.modulator_filtered.iter_mut().zip(self.modulator_filters.iter_mut()) {
            filter.filter(modulator, modulator_band);
        }

        // measure modulator bands amplitude

        // filter the carrier into bands
        for (carrier_band, filter) in self.carrier_filtered.iter_mut().zip(self.carrier_filters.iter_mut()) {
            filter.filter(carrier, carrier_band);
        }

        // modulate carrier with modulator amplitudes
        for (carrier_band, modulator_band) in self.carrier_filtered.iter_mut().zip(self.modulator_filtered.iter()) {
            let mut max_s = 0f32;

            for modulator_s in modulator_band.iter() {
                max_s = if *modulator_s > max_s { *modulator_s } else { max_s };
            }

            for carrier_s in carrier_band.iter_mut() {
                *carrier_s = *carrier_s * max_s;
            }
        }

        // output mix
        for carrier_band in self.carrier_filtered.iter() {
            for (output_s, band_s) in output.iter_mut().zip(carrier_band.iter()) {
                *output_s += *band_s;
            }
        }
        let gain = 20f32;  // using insane gain because my filters suck
        for output_s in output.iter_mut() {
            *output_s *= gain / BANDS as f32;
        }
    }
}
