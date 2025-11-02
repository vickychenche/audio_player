mod player;
mod audio;
mod ui;
mod playlist;

use eframe::egui;
//use player::Player;
//use audio::find_audio_files;
//use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Lil Glucose",
        options, 
        Box::new(|_cc| Ok(Box::new(ui::AudioPlayerApp::default()))),
    )
}