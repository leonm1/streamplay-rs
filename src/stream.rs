extern crate opus;
extern crate portaudio;

use portaudio as pa;
use std::f32::consts::PI;

const SAMPLE_RATE: u32 = 44_100;
const FRAMES_PER_BUFFER: u32 = 64;
const TABLE_SIZE: usize = 200;
const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 2;

pub fn run() -> Result<(), pa::Error> {
    println!(
        "PortAudio Test: input. SR = {}, BufSize = {}",
        SAMPLE_RATE, FRAMES_PER_BUFFER
    );
    let pa = pa::PortAudio::new()?;
    let enc = opus::Encoder::new(SAMPLE_RATE, opus::Channels::Stereo, opus::Application::Voip);

    let settings =
        pa.default_input_stream_settings::<f32>(CHANNELS, SAMPLE_RATE as f64, FRAMES_PER_BUFFER)?;

    fn wait_for_stream<F>(f: F, name: &str) -> u32 
    where F: Fn() -> Result<pa::StreamAvailable, pa::Error> {
        'waiting_for_stream: loop {
            match f() {
                Ok(available) => match available {
                    pa::StreamAvailable::Frames(frames) => return frames as u32,
                    pa::StreamAvailable::InputOverflowed => println!("An input overflow occurred"),
                    pa::StreamAvailable::OutputUnderflowed => println!("An input underflow occurred"),
                },
                Err(e) => panic!("An error occurred while waiting for the {} stream: {}", name, e),
            }
        }
    }

    'stream: loop {
        let in_frames = wait_for_stream(|| stream.read_available(), "Read");
    }

    let callback = move |pa::stream::InputCallbackArgs { buffer, frames, .. }| {
        for _ in 0..frames {
            
        }

        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    Ok(())
}
