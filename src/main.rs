mod player;
mod audio;
mod ui;
mod playlist;
mod playlist_manager;

use eframe::egui;
//use player::Player;
//use audio::find_audio_files;
//use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([300.0, 200.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Lil Glucose",
        options, 
        Box::new(|_cc| Ok(Box::new(ui::AudioPlayerApp::default()))),
    )
}