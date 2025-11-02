use std::path::PathBuf;
use eframe::egui;
use catppuccin_egui::{set_theme, MOCHA, LATTE};

pub struct AudioPlayerApp {
    playlist: Option<crate::playlist::Playlist>,  // Instead of audio_files + current_index + durations
    is_playing: bool,
    player: Option<crate::player::Player>,
    is_dark_theme: bool,
    folder_path: String,
    volume: f32,
}

impl Default for AudioPlayerApp {
    fn default() -> Self {
        let mut app = Self {
            folder_path: "./tmp/audio".to_string(),
            playlist: None,  // Changed
            volume: 1.0, 
            is_playing: false,
            player: None,
            is_dark_theme: true,
        };
        app.load_files();
        app
    }
    
}

impl eframe::App for AudioPlayerApp {
    fn update(&mut self, ctx: &egui::Context , _frame: &mut eframe::Frame) {

        if self.is_dark_theme {
            set_theme(ctx, MOCHA);
        } else {
            set_theme(ctx, LATTE);
        }

        if self.is_playing {
            if let Some(player) = &self.player {
                if player.is_empty() {
                    self.play_next();
                }
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lil Glucose Player");
            ui.separator();
            // Folder selection
            ui.horizontal(|ui| {
                let theme_text = if self.is_dark_theme { "🌙" } else { "☀️" };
                if ui.button(theme_text).clicked() {
                    self.is_dark_theme = !self.is_dark_theme;
                }
                ui.label("Folder:");
                ui.text_edit_singleline(&mut self.folder_path);
                if ui.button("Load Files").clicked() {
                    // We'll implement this method next
                    self.load_files();
                }
            });

            ui.separator();

            let file_count = self.playlist.as_ref().map(|p| p.len()).unwrap_or(0);
            ui.label(format!("Found {} files", file_count));

            ui.separator(); 

            // Play/Pause controls
            ui.horizontal(|ui| {
                if ui.button(if self.is_playing { "⏸" } else { "▶" }).clicked() {
                    self.toggle_play_pause();
                }
            });

            if let Some(playlist) = &self.playlist {
                if let Some(audio_file) = playlist.current() {
                    ui.label(format!("Now: {}", audio_file.title));
                }

                
            }
            
        });
        ctx.request_repaint();
    }
}

impl AudioPlayerApp {
    fn load_files(&mut self) {
        let path = PathBuf::from("./tmp/audio");
        match crate::audio::find_audio_files(&path) {
            Ok(files) => {
                self.playlist = Some(crate::playlist::Playlist::new(files));
            }
            Err(e) => {
                eprintln!("error loading files {}", e);
            }
        }
    }

    fn toggle_play_pause(&mut self) {
        if self.player.is_none() {
            if let Ok(mut player) = crate::player::Player::new(){
                if let Some(playlist) = &self.playlist {
                    if let Some(audio_file) = playlist.current() {
                        let _ = player.play(&audio_file.path);
                        self.is_playing = true;
                        self.player = Some(player);
                    }
                }
            }
        }else if let Some(player) = &self.player {
            if self.is_playing{
                player.pause();
                self.is_playing = false;
            } else {
                player.resume();
                self.is_playing = true;
            }
        }
    }

    pub fn play_next(&mut self) {
        if let Some(playlist) = &mut self.playlist {
            if let Some(next_file) = playlist.next() {
                if let Some(player) = &mut self.player {
                    let _ = player.play(&next_file.path);
                }
            } else {
                // Reached end
                self.is_playing = false;
            }
        }
    }
}