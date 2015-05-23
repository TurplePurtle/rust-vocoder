
use util::TAU;

// BiquadFilter
// so far, only bandpass is implemented
pub struct BiquadFilter {
    sample_rate: f32,
    a0: f32,  // for x[n]
    a1: f32,  // for x[n-1]
    a2: f32,  // for x[n-2]
    b1: f32,  // for y[n-1]
    b2: f32,  // for y[n-2]
    x2: f32,  // last x[N-2]
    x1: f32,  // last x[N-1]
    y2: f32,  // last y[N-2]
    y1: f32,  // last y[N-1]
}

impl BiquadFilter {
    // create a new biquad filter
    // sample_rate is required to convert Hz to discrete frequency
    pub fn new(sample_rate: f32) -> BiquadFilter {
        BiquadFilter {
            sample_rate: sample_rate,
            a0: 1f32, a1: 0f32, a2: 0f32, b1: 0f32, b2: 0f32,
            x2: 0f32, x1: 0f32, y2: 0f32, y1: 0f32,
        }
    }

    // set the bandpass filter coefficients
    pub fn set_bandpass(&mut self, fc: f32, q: f32) {
        let w0: f32 = (TAU * fc / (2f32 * self.sample_rate)).tan();
        let norm: f32 = 1f32 / (1f32 + w0/q + w0*w0);
        self.a0 = w0 / q * norm;
        self.a1 = 0f32;
        self.a2 = -self.a0;
        self.b1 = 2f32 * (w0*w0 - 1f32) * norm;
        self.b2 = (1f32 - w0/q + w0*w0) * norm;
    }

    // filter input to output buffer
    // this function should be called sequentially only once
    // for each input buffer since it keeps track of the last
    // two samples from the previous input and output buffers
    pub fn filter(&mut self, input: &[f32], output: &mut[f32]) {
        output[0] = self.a0*input[0]
                  + self.a1*self.x1 + self.a2*self.x2
                  - self.b1*self.y1 - self.b2*self.y2;
        output[1] = self.a0*input[1]
                  + self.a1*input[0] + self.a2*self.x1
                  - self.b1*output[0] - self.b2*self.y1;

        let n = output.len();
        for i in 2..n {
            output[i] = self.a0 * input[i  ]
                      + self.a1 * input[i-1]
                      + self.a2 * input[i-2]
                      - self.b1 * output[i-1]
                      - self.b2 * output[i-2];
        }
        self.x2 = input[n-2];
        self.x1 = input[n-1];
        self.y2 = output[n-2];
        self.y1 = output[n-1];
    }
}
