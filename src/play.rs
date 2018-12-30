extern crate portaudio;

use portaudio as pa;
use std::f32::consts::PI;

const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const TABLE_SIZE: usize = 200;
const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 2;

pub fn run() -> Result<(), pa::Error> {
    println!(
        "PortAudio Test: output sine wave. SR = {}, BufSize = {}",
        SAMPLE_RATE, FRAMES_PER_BUFFER
    );

    let mut sine = [0.0; TABLE_SIZE];
    for i in 0..TABLE_SIZE {
        sine[i] = (i as f32 / TABLE_SIZE as f32 * PI * 2.0).sin() as f32;
    }

    let mut left_phase = 0;
    let mut right_phase = 0;

    let pa = pa::PortAudio::new()?;

    let settings =
        pa.default_output_stream_settings::<f32>(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)?;

    let callback = move |pa::stream::OutputCallbackArgs { buffer, frames, .. }| {
        let mut idx = 0;
        for _ in 0..frames {
            buffer[idx] = sine[left_phase];
            buffer[idx + 1] = sine[right_phase];
            left_phase += 1;
            if left_phase >= TABLE_SIZE {
                left_phase -= TABLE_SIZE;
            };
            right_phase += 3;
            if right_phase >= TABLE_SIZE {
                right_phase -= TABLE_SIZE;
            };
            idx += 2;
        }

        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;

    println!("Playing for {} seconds...", NUM_SECONDS);
    pa.sleep(NUM_SECONDS * 1_000);

    stream.stop()?;
    stream.close()?;

    println!("Test finished.");

    Ok(())
}
