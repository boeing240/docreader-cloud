#![windows_subsystem = "windows"]

mod app;
mod config;
mod library;
mod pdf;
mod sync;
mod ui;

use app::PdfReaderApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("PDF Reader"),
        ..Default::default()
    };

    eframe::run_native(
        "PDF Reader",
        options,
        Box::new(|cc| Ok(Box::new(PdfReaderApp::new(cc)))),
    )
}
