use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::time::Duration;
use std::fs::File;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;

const AUDIO_EXTENSIONS: &[&str] = &["mp3", "wav"];

#[derive(Clone)]
pub struct AudioFile {
    pub path: PathBuf,
    pub duration: Option<Duration>,
    pub title: String,
}

impl AudioFile {
    pub fn new(path: PathBuf) -> Self {
        let duration = get_audio_duration(&path);
        let title = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();
        
        Self { path, duration, title }
    }
}

fn get_audio_duration(path: &Path) -> Option<Duration> {
    let file = File::open(path).ok()?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }
    
    // Probe the media source for metadata
    let format_opts = Default::default();
    let metadata_opts = Default::default();
    
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)
        .ok()?;
    
    // Get the default track
    let track = probed.format.default_track()?;
    
    // Calculate duration from time base and number of frames
    let time_base = track.codec_params.time_base?;
    let n_frames = track.codec_params.n_frames?;
    
    let duration_secs = (n_frames as f64) * time_base.numer as f64 / time_base.denom as f64;
    
    Some(Duration::from_secs_f64(duration_secs))
}

fn is_audio_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| AUDIO_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

pub fn find_audio_files(path: &Path) -> Result<Vec<AudioFile>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();

    if path.is_dir(){
        for entry in WalkDir::new(path) {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_file() && is_audio_file(entry_path) {
                files.push(AudioFile::new(entry_path.to_path_buf()))
            }
        }
    } else if path.is_file() {
        if is_audio_file(path) {
            files.push(AudioFile::new(path.to_path_buf()));
        }
    }
    Ok(files)
} 