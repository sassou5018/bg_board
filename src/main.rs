use inputbot::KeybdKey;
use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("audiofile.mp3").unwrap());
    let source = Decoder::new(file).unwrap();
    let source_buffered = source.buffered();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let sink = Arc::new(Mutex::new(sink));
    KeybdKey::bind_all(move |e| {
        match e {
            KeybdKey::QKey => std::process::exit(0),
            _ => {
                println!("character pressed: {:?}", e);
                sink.lock().unwrap().append(source_buffered.clone());
            }
        };
    });
    inputbot::handle_input_events();
}
