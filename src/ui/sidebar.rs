use egui::Ui;

use crate::config::constants::*;
use crate::library::book::Book;
use crate::library::progress::BookProgress;

pub struct Sidebar;

impl Sidebar {
    pub fn show(
        ui: &mut Ui,
        books: &[Book],
        progress: &std::collections::HashMap<String, BookProgress>,
        selected_book: Option<&str>,
        on_select: &mut dyn FnMut(&str),
    ) {
        ui.heading("Библиотека");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            for book in books {
                let is_selected = selected_book == Some(&book.file_hash);
                let book_progress = progress.get(&book.file_hash);

                let label = format!("[{}] {}", book.format.display_name(), book.file_name);
                let response = ui.selectable_label(is_selected, label);

                if let Some(bp) = book_progress {
                    // Use book.total_pages if available (from scanner), fallback to progress
                    let total = if book.total_pages > 0 {
                        book.total_pages
                    } else {
                        bp.total_pages
                    };
                    ui.horizontal(|ui| {
                        ui.label(format!("Стр. {}/{}", bp.current_page, total));
                        let progress_fraction = bp.current_page as f32 / total.max(1) as f32;
                        ui.add(
                            egui::ProgressBar::new(progress_fraction)
                                .desired_width(SIDEBAR_PROGRESS_BAR_WIDTH)
                                .show_percentage(),
                        );
                    });
                } else if book.total_pages > 0 {
                    ui.label(format!("{} стр.", book.total_pages));
                } else {
                    ui.label("Не начато");
                }

                ui.add_space(SIDEBAR_ITEM_SPACING);

                if response.clicked() {
                    on_select(&book.file_hash);
                }
            }
        });
    }
}
