use rodio::{OutputStream, Sink, Decoder};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct Player {
    _stream: OutputStream,
    sink: Sink,
}

impl Player {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let stream = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");
        let sink = Sink::connect_new(stream.mixer());
        Ok(Player {
            _stream: stream,
            sink,
        })
    }

    pub fn play(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;
        self.sink.append(source);
        Ok(())
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn resume(&self) {
        self.sink.play();
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }

    pub fn wait_until_end(&self) {
        self.sink.sleep_until_end();
    }
}
