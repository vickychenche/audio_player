use std::path::PathBuf;
use eframe::egui;
use catppuccin_egui::{set_theme, MOCHA, LATTE};
use std::time::Duration;

pub struct AudioPlayerApp {
    playlist: Option<crate::playlist::Playlist>,  // Instead of audio_files + current_index + durations
    is_playing: bool,
    player: Option<crate::player::Player>,
    is_dark_theme: bool,
    folder_path: String,
    volume: f32,

    playlist_manager: crate::playlist_manager::PlaylistManager,
    playlist_names: Vec<String>,
    selected_playlist: Option<String>,
    show_create_dialog: bool,
    new_playlist_name: String,
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
            playlist_manager: crate::playlist_manager::PlaylistManager::new(),
            playlist_names: Vec::new(),
            selected_playlist: None,
            show_create_dialog: false,
            new_playlist_name: String::new(),
        };

        if let Ok(names) = app.playlist_manager.scan_playlists() {
            app.playlist_names = names;
        }
        
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
        
        // NEW: Add sidebar BEFORE CentralPanel
        egui::SidePanel::left("playlist_sidebar")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Playlists");
                ui.separator();
                
                // "Create Playlist" button
                if ui.button("➕ Create Playlist").clicked() {
                    self.show_create_dialog = true;
                }
                
                ui.separator();
                
                // List of playlists
                let mut clicked_playlist = None;
                for playlist_name in &self.playlist_names {
                    // Highlight if selected
                    let is_selected = self.selected_playlist.as_ref() == Some(playlist_name);
                    
                    // Create button style
                    let response = if is_selected {
                        ui.selectable_label(true, playlist_name)
                    } else {
                        ui.selectable_label(false, playlist_name)
                    };
                    
                    // Handle click - store name to process after loop
                    if response.clicked() {
                        clicked_playlist = Some(playlist_name.clone());
                    }
                }
                
                // Process click outside the loop to avoid borrowing issues
                if let Some(name) = clicked_playlist {
                    self.selected_playlist = Some(name.clone());
                    self.load_playlist_songs(&name);
                }
            });
        
        // Create Playlist Dialog
        if self.show_create_dialog {
            egui::Window::new("Create Playlist")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Playlist Name:");
                    ui.text_edit_singleline(&mut self.new_playlist_name);
                    
                    ui.add_space(10.0);
                    
                    // Show error if name is invalid
                    let name_trimmed = self.new_playlist_name.trim();
                    let is_valid = !name_trimmed.is_empty() 
                        && !name_trimmed.chars().any(|c| matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|'));
                    
                    if !is_valid && !name_trimmed.is_empty() {
                        ui.label(egui::RichText::new("Invalid characters: / \\ : * ? \" < > |").color(egui::Color32::RED));
                    }
                    
                    // Convert to owned String to avoid borrowing issues
                    let playlist_name = name_trimmed.to_string();
                    
                    ui.horizontal(|ui| {
                        // Create button
                        let create_enabled = is_valid;
                        if ui.add_enabled(create_enabled, egui::Button::new("Create")).clicked() {
                            if let Err(e) = self.playlist_manager.create_playlist(&playlist_name) {
                                eprintln!("Error creating playlist: {}", e);
                            } else {
                                // Success - refresh playlist list and close dialog
                                self.refresh_playlists();
                                self.new_playlist_name.clear();
                                self.show_create_dialog = false;
                            }
                        }
                        
                        // Cancel button
                        if ui.button("Cancel").clicked() {
                            self.new_playlist_name.clear();
                            self.show_create_dialog = false;
                        }
                    });
                });
        }
        
        // Existing CentralPanel stays the same
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Lil Glucose Player");
            ui.separator();
            // Folder selection
            ui.horizontal(|ui| {
                let theme_text = if self.is_dark_theme { "🌙" } else { "light" };
                if ui.button(theme_text).clicked() {
                    self.is_dark_theme = !self.is_dark_theme;
                }
                // ui.label("Folder:");
                // ui.text_edit_singleline(&mut self.folder_path);
                // if ui.button("Load Files").clicked() {
                //     self.load_files();
                // }
            });

            ui.separator();

            let file_count = self.playlist.as_ref().map(|p| p.len()).unwrap_or(0);
            ui.label(format!("Found {} files", file_count));

            ui.separator(); 
            
            // Play/Pause controls
            ui.horizontal(|ui| {
                // Previous Button
                if ui.button("⏮").clicked() {
                    self.play_previous();
                }
                // Play/Pause button
                if ui.button(if self.is_playing { "⏸" } else { "▶" }).clicked() {
                    self.toggle_play_pause();
                }
                // Next Button
                if ui.button("⏭").clicked() {
                    self.play_next();
                }
            });
            

            if let Some(playlist) = &self.playlist {
                if let Some(audio_file) = playlist.current() {
                    ui.label(format!("Now: {}", audio_file.title));
                    if let Some(player) = &self.player {
                        let current_pos = player.get_position();
                        let total_duration = audio_file.duration.unwrap_or(Duration::ZERO);
    
                        let mut progress = if total_duration.as_secs() > 0 {
                            current_pos.as_secs_f64() / total_duration.as_secs_f64()
                        } else {
                            0.0
                        };
                        progress = progress.min(1.0);
                        // Display slider (read-only for now)
                        ui.add(
                            egui::Slider::new(&mut progress, 0.0..=1.0)
                                .show_value(false)  // Don't show the 0.0-1.0 number
                        );
                        
                        // Show time below slider: "1:23 / 3:45"
                        ui.label(format!(
                            "{} / {}", 
                            format_duration(current_pos),
                            format_duration(total_duration)
                        ));
                    }
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
        } else if let Some(player) = &mut self.player {
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

    pub fn play_previous(&mut self) {
        if let Some(playlist) = &mut self.playlist {
            if let Some(previous_file) = playlist.previous() {
                if let Some(player) = &mut self.player {
                    let _ = player.play(&previous_file.path);
                }
            } 
        }
    }

    fn refresh_playlists(&mut self) {
        if let Ok(names) = self.playlist_manager.scan_playlists() {
            self.playlist_names = names;
        }
    }

    // NEW: Load songs from selected playlist
    fn load_playlist_songs(&mut self, playlist_name: &str) {
        if let Ok(songs) = self.playlist_manager.get_playlist_songs(playlist_name) {
            self.playlist = Some(crate::playlist::Playlist::new(songs));
        } else {
            eprintln!("Failed to load songs from playlist: {}", playlist_name);
        }
    }
}

fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let minutes = total_secs / 60;
    let seconds = total_secs % 60;
    format!("{}:{:02}", minutes, seconds)
}