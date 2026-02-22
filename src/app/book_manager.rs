use egui::Context;
use std::sync::Arc;

use crate::config::constants::*;
use crate::library::scanner::LibraryScanner;
use crate::ui::toolbar::ToolbarAction;

use super::render_manager;
use super::DocReaderApp;

pub(crate) fn select_book(app: &mut DocReaderApp, _ctx: &Context, book_hash: &str) {
    if app.selected_book_hash.as_deref() == Some(book_hash) {
        return;
    }

    app.selected_book_hash = Some(book_hash.to_string());
    app.settings.last_opened_book = Some(book_hash.to_string());
    app.current_document_bytes = None;

    // Find book and load saved page, scroll position, and zoom
    if let Some(bp) = app.progress.books.get(book_hash) {
        app.current_page = bp.current_page;
        app.page_input = bp.current_page.to_string();
        app.current_scroll_offset = bp.scroll_offset;

        // Restore zoom or use default
        let new_zoom = bp.zoom.unwrap_or(ZOOM_DEFAULT);
        if (new_zoom - app.zoom).abs() > 0.001 {
            app.zoom = new_zoom;
            app.page_cache.clear();
        }
    } else {
        app.current_page = 1;
        app.page_input = "1".to_string();
        app.current_scroll_offset = (0.0, 0.0);
        app.zoom = ZOOM_DEFAULT;
        app.page_cache.clear();
        // Add book to progress
        if let Some(book) = app.books.iter().find(|b| b.file_hash == book_hash) {
            app.progress.add_book(
                book_hash.to_string(),
                book.file_name.clone(),
                book.file_path.to_string_lossy().to_string(),
                book.total_pages,
            );
            app.needs_save = true;
        }
    }

    // Load document bytes
    if let Some(book) = app.books.iter().find(|b| b.file_hash == book_hash) {
        match std::fs::read(&book.file_path) {
            Ok(bytes) => {
                app.current_document_bytes = Some(Arc::new(bytes));
            }
            Err(e) => {
                app.error_message = Some(format!("Ошибка загрузки: {}", e));
            }
        }
    }

    render_manager::request_render(app);
}

pub(crate) fn go_to_page(app: &mut DocReaderApp, page: u32) {
    let Some(book_hash) = &app.selected_book_hash else {
        return;
    };

    let total_pages = app
        .books
        .iter()
        .find(|b| &b.file_hash == book_hash)
        .map(|b| b.total_pages)
        .unwrap_or(1);

    let new_page = page.clamp(1, total_pages.max(1));
    if new_page != app.current_page {
        app.current_page = new_page;
        app.page_input = new_page.to_string();
        app.current_scroll_offset = (0.0, 0.0); // Reset scroll on page change
        app.progress.update_book_progress(book_hash, new_page);
        app.needs_save = true;
        render_manager::request_render(app);
    }
}

pub(crate) fn handle_toolbar_action(app: &mut DocReaderApp, action: ToolbarAction) {
    match action {
        ToolbarAction::PrevPage => {
            go_to_page(app, app.current_page.saturating_sub(1));
        }
        ToolbarAction::NextPage => {
            go_to_page(app, app.current_page + 1);
        }
        ToolbarAction::GoToPage(page) => {
            go_to_page(app, page);
        }
        ToolbarAction::ZoomIn => {
            app.zoom = (app.zoom + ZOOM_STEP).min(ZOOM_MAX);
            app.page_cache.clear();
            if let Some(book_hash) = &app.selected_book_hash {
                app.progress.update_zoom(book_hash, app.zoom);
                app.needs_save = true;
            }
            render_manager::request_render(app);
        }
        ToolbarAction::ZoomOut => {
            app.zoom = (app.zoom - ZOOM_STEP).max(ZOOM_MIN);
            app.page_cache.clear();
            if let Some(book_hash) = &app.selected_book_hash {
                app.progress.update_zoom(book_hash, app.zoom);
                app.needs_save = true;
            }
            render_manager::request_render(app);
        }
        ToolbarAction::ZoomReset => {
            app.zoom = ZOOM_DEFAULT;
            app.page_cache.clear();
            if let Some(book_hash) = &app.selected_book_hash {
                app.progress.update_zoom(book_hash, app.zoom);
                app.needs_save = true;
            }
            render_manager::request_render(app);
        }
    }
}

pub(crate) fn rescan_library(app: &mut DocReaderApp) {
    let mut books =
        LibraryScanner::scan_and_load_books(&app.settings.library_path, None).unwrap_or_default();

    for book in &mut books {
        if let Some(bp) = app.progress.books.get(&book.file_hash) {
            if bp.total_pages > 0 {
                book.total_pages = bp.total_pages;
            }
        }
    }

    app.books = books;
}
