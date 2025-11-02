use crate::audio::AudioFile;

pub struct Playlist {
    files: Vec<AudioFile>,
    current_index: Option<usize>,
}

impl Playlist {
    pub fn new(files: Vec<AudioFile>) -> Self {
        let current_index = if files.is_empty() { None } else { Some(0) };
        Self { files, current_index }
    }

    pub fn current(&self) -> Option<&AudioFile> {
        self.current_index.and_then(|i| self.files.get(i))
    }

    pub fn next(&mut self) -> Option<&AudioFile> {
        if let Some(idx) = self.current_index {
            let next_idx = idx + 1;
            if next_idx < self.files.len() {
                self.current_index = Some(next_idx);
                return self.files.get(next_idx);
            }
        }
        None
    }

    pub fn all_files(&self) -> &[AudioFile] {
        &self.files
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }
}