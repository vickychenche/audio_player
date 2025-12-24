use rodio::{OutputStream, Sink, Decoder};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::{Duration, Instant};

pub struct Player {
    _stream: OutputStream,
    sink: Sink,
    playback_start: Option<Instant>,
    pause_offset: Duration,
    paused_at: Option<Instant>,
}

impl Player {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let stream = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");
        let sink = Sink::connect_new(stream.mixer());
        Ok(Player {
            _stream: stream,
            sink,
            playback_start: None,        
            pause_offset: Duration::ZERO,  
            paused_at: None,   
        })
    }

    pub fn play(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        self.sink.stop();
        
        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;
        self.sink.append(source);
        self.playback_start = Some(Instant::now());
        self.pause_offset = Duration::ZERO;
        self.paused_at = None;

        Ok(())
    }

    pub fn pause(&mut self) {
        self.sink.pause();
        self.paused_at = Some(Instant::now())
    }

    pub fn resume(&mut self) {
        self.sink.play();
        if let Some(paused_at) = self.paused_at.take() {
            let pause_duration = paused_at.elapsed();
            self.pause_offset += pause_duration;
        }
    }

    pub fn stop(&mut self) {
        self.sink.stop();
        self.playback_start = None;
        self.pause_offset = Duration::ZERO;
        self.paused_at = None;
    }

    pub fn get_position(&self) -> Duration {
        match self.playback_start {
            Some(start) => {
                let elapsed = start.elapsed();
                
                // If currently paused, calculate position up to pause time
                if let Some(paused_at) = self.paused_at {
                    let position = paused_at.duration_since(start);
                    position.saturating_sub(self.pause_offset)
                } else {
                    // Currently playing
                    elapsed.saturating_sub(self.pause_offset)
                }
            }
            None => Duration::ZERO,
        }
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
