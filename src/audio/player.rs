use rodio::{OutputStream, Sink, Source};
use std::time::Duration;

pub fn play_fractal_sound(zoom_level: f64) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    let freq = 440.0 + (zoom_level * 10.0);
    let duration = Duration::from_secs_f32(0.5);

    let source = rodio::source::SineWave::new(freq as f32)
        .take_duration(duration)
        .amplify(0.25);

    sink.append(source);
    sink.sleep_until_end();
}
