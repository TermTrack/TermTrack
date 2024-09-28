use rodio::OutputStreamHandle;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

pub fn play_audio(stream_handle: &OutputStreamHandle, path: &str) {
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());
}

pub fn audio_loop(stream_handle: &OutputStreamHandle, path: &str) {
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).unwrap());
    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();
    let source = source.repeat_infinite();
    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples());
}

// #[cfg(test)]

// mod tests {
//     use super::*;

//     #[test]
//     fn test() {
//         audiotest();
//     }
// }
