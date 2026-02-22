#![windows_subsystem = "windows"]

mod app;
mod config;
mod library;
mod renderer;
mod sync;
mod ui;

use app::DocReaderApp;
use config::constants::*;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_INITIAL_WIDTH, WINDOW_INITIAL_HEIGHT])
            .with_min_inner_size([WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT])
            .with_title(WINDOW_TITLE),
        // Enable persistence for egui state (including ScrollArea positions)
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        WINDOW_TITLE,
        options,
        Box::new(|cc| Ok(Box::new(DocReaderApp::new(cc)))),
    )
}
