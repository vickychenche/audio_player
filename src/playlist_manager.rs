use std::path::PathBuf;
use std::fs;
use crate::audio::{AudioFile, find_audio_files};

pub struct PlaylistManager {
    playlist_dir: PathBuf,
}

impl PlaylistManager {
    pub fn new() -> Self {
        Self { playlist_dir: PathBuf::from("./playlist") }
    }

    pub fn scan_playlists(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut playlists = Vec::new();

        if !self.playlist_dir.exists() {
            fs::create_dir_all(&self.playlist_dir)?;
            return Ok(playlists);
        }

        let entries = fs::read_dir(&self.playlist_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    playlists.push(name.to_string());
                }
            }
        }
        playlists.sort();
        Ok(playlists)
    }

    pub fn create_playlist(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if name.is_empty() {
            return Err("Playlist name cannot be empty".into());
        }

        fs::create_dir_all(&self.playlist_dir)?;
        
        let playlist_path = self.playlist_dir.join(name);
        
        if playlist_path.exists() {
            return Err(format!("Playlist '{}' already exists", name).into());
        }
        
        // Step 6: Create the folder
        fs::create_dir(&playlist_path)?;
        
        Ok(())
    }

    pub fn get_playlist_songs(&self, playlist_name: &str) -> Result<Vec<AudioFile>, Box<dyn std::error::Error>> {
        // Step 1: Build path to playlist folder
        let playlist_path = self.playlist_dir.join(playlist_name);
        
        // Step 2: Check if playlist exists
        if !playlist_path.exists() {
            return Err(format!("Playlist '{}' does not exist", playlist_name).into());
        }
        
        // Step 3: Use existing find_audio_files function!
        find_audio_files(&playlist_path)
    }
}