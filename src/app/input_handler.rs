use egui::Context;

use super::book_manager;
use super::DocReaderApp;

pub(crate) fn handle_keyboard_input(app: &mut DocReaderApp, ctx: &Context) {
    ctx.input(|i| {
        if i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::PageUp) {
            book_manager::go_to_page(app, app.current_page.saturating_sub(1));
        }
        if i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::PageDown) {
            book_manager::go_to_page(app, app.current_page + 1);
        }
        if i.key_pressed(egui::Key::Home) {
            book_manager::go_to_page(app, 1);
        }
        if i.key_pressed(egui::Key::End) {
            if let Some(book_hash) = &app.selected_book_hash {
                if let Some(book) = app.books.iter().find(|b| &b.file_hash == book_hash) {
                    book_manager::go_to_page(app, book.total_pages);
                }
            }
        }
    });
}
