#![windows_subsystem = "windows"]

mod app;
mod config;
mod library;
mod renderer;
mod sync;
mod ui;

use app::DocReaderApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("DocReader"),
        ..Default::default()
    };

    eframe::run_native(
        "DocReader",
        options,
        Box::new(|cc| Ok(Box::new(DocReaderApp::new(cc)))),
    )
}
