use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::time::Duration;

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

fn get_audio_duration(_path: &Path) -> Option<Duration> {
    // Placeholder - returns None for now
    None
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