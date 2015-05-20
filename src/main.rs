//!
//! A demonstration of constructing and using a non-blocking stream.
//!
//! Audio from the default input device is passed directly to the default output device in a duplex
//! stream, so beware of feedback!
//!

extern crate portaudio;

mod oscillator;
use oscillator::Oscillator;

use portaudio::pa;
use std::error::Error;

const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES: u32 = 256;


fn main() {

    let mut oscillator = Oscillator::new(SAMPLE_RATE as f32);

    println!("PortAudio version : {}", pa::get_version());
    println!("PortAudio version text : {}", pa::get_version_text());

    match pa::initialize() {
        Ok(()) => println!("Successfully initialized PortAudio"),
        Err(err) => println!("An error occurred while initializing PortAudio: {}", err.description()),
    }

    println!("PortAudio host count : {}", pa::host::get_api_count() as isize);

    let default_host = pa::host::get_default_api();
    println!("PortAudio default host : {}", default_host as isize);

    match pa::host::get_api_info(default_host) {
        None => println!("Couldn't retrieve api info for the default host."),
        Some(info) => println!("PortAudio host name : {}", info.name),
    }

    let def_input = pa::device::get_default_input();
    let input_info = match pa::device::get_info(def_input) {
        Ok(info) => info,
        Err(err) => panic!("An error occurred while retrieving input info: {}", err.description()),
    };
    println!("Default input device info :");
    println!("\tversion : {}", input_info.struct_version);
    println!("\tname : {}", input_info.name);
    println!("\tmax input channels : {}", input_info.max_input_channels);
    println!("\tmax output channels : {}", input_info.max_output_channels);
    println!("\tdefault sample rate : {}", input_info.default_sample_rate);

    // Construct the input stream parameters.
    let input_stream_params = pa::StreamParameters {
        device : def_input,
        channel_count : 1,
        sample_format : pa::SampleFormat::Float32,
        suggested_latency : input_info.default_low_input_latency
    };

    let def_output = pa::device::get_default_output();
    let output_info = match pa::device::get_info(def_output) {
        Ok(info) => info,
        Err(err) => panic!("An error occurred while retrieving output info: {}", err.description()),
    };

    println!("Default output device name : {}", output_info.name);

    // Construct the output stream parameters.
    let output_stream_params = pa::StreamParameters {
        device : def_output,
        channel_count : 1,
        sample_format : pa::SampleFormat::Float32,
        suggested_latency : output_info.default_low_output_latency
    };

    // Check that the stream format is supported.
    if let Err(err) = pa::is_format_supported(&input_stream_params, &output_stream_params, SAMPLE_RATE) {
        panic!("The given stream format is unsupported: {:?}", err.description());
    }

    // Construct a stream with input and output sample types of f32.
    let mut stream : pa::Stream<f32, f32> = pa::Stream::new();

    // Construct a custom callback function - in this case we're using a FnMut closure.
    let callback = Box::new(move |
        input: &[f32],
        output: &mut[f32],
        frames: u32,
        _time_info: &pa::StreamCallbackTimeInfo,
        _flags: pa::StreamCallbackFlags,
    | -> pa::StreamCallbackResult {

        assert!(frames == FRAMES);

        // Pass the input straight to the output - BEWARE OF FEEDBACK!
        for (output_sample, _input_sample) in output.iter_mut().zip(input.iter()) {
            *output_sample = oscillator.sawtooth(440.0f32);
        }

        pa::StreamCallbackResult::Continue
    });


    // Open a non-blocking stream (indicated by giving Some(callback)).
    match stream.open(Some(&input_stream_params),
                      Some(&output_stream_params),
                      SAMPLE_RATE,
                      FRAMES,
                      pa::StreamFlags::empty(),
                      Some(callback)) {
        Ok(()) => println!("Successfully opened the stream."),
        Err(err) => println!("An error occurred while opening the stream: {}", err.description()),
    }

    match stream.start() {
        Ok(()) => println!("Successfully started the stream."),
        Err(err) => println!("An error occurred while starting the stream: {}", err.description()),
    }


    // Loop while the non-blocking stream is active.
    while let Ok(true) = stream.is_active() {}


    match stream.close() {
        Ok(()) => println!("Successfully closed the stream."),
        Err(err) => println!("An error occurred while closing the stream: {}", err.description()),
    }

    match pa::terminate() {
        Ok(()) => println!("Successfully terminated PortAudio."),
        Err(err) => println!("An error occurred while terminating PortAudio: {}", err.description()),
    }

}
