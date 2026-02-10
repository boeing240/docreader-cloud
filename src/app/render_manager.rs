use egui::Context;
use std::sync::Arc;

use super::render_thread::{RenderRequest, RenderResponse};
use super::DocReaderApp;

pub(crate) fn request_render(app: &mut DocReaderApp) {
    let Some(book_hash) = &app.selected_book_hash else {
        return;
    };

    let Some(book) = app.books.iter().find(|b| &b.file_hash == book_hash) else {
        return;
    };

    let Some(bytes) = &app.current_document_bytes else {
        return;
    };

    // Account for HiDPI: render at native pixel density
    let dpi = (96.0 * app.zoom * app.pixels_per_point) as u32;
    if let Some(texture) = app.page_cache.get(book_hash, app.current_page, dpi) {
        app.current_texture = Some(texture.clone());
        app.is_rendering = false;
        return;
    }

    // Send render request to background thread
    let page_index = app.current_page.saturating_sub(1);
    let request = RenderRequest {
        book_hash: book_hash.clone(),
        page: app.current_page,
        page_index,
        zoom: app.zoom * app.pixels_per_point,
        dpi,
        bytes: Arc::clone(bytes),
        format: book.format,
    };

    if app.render_tx.send(request).is_ok() {
        app.is_rendering = true;
    }
}

pub(crate) fn poll_render_results(app: &mut DocReaderApp, ctx: &Context) {
    while let Ok(response) = app.result_rx.try_recv() {
        app.is_rendering = false;
        match response {
            RenderResponse::Ok(result) => {
                let size = [
                    result.image.width() as usize,
                    result.image.height() as usize,
                ];
                let pixels = result.image.into_raw();
                let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

                let texture = ctx.load_texture(
                    format!("page_{}_{}", result.book_hash, result.page),
                    color_image,
                    egui::TextureOptions::LINEAR,
                );

                app.page_cache
                    .insert(&result.book_hash, result.page, result.dpi, texture.clone());

                // Update total_pages if we learned it from render thread
                if result.total_pages > 0 {
                    if let Some(book) = app
                        .books
                        .iter_mut()
                        .find(|b| b.file_hash == result.book_hash)
                    {
                        book.total_pages = result.total_pages;
                    }
                    if let Some(bp) = app.progress.books.get_mut(&result.book_hash) {
                        if bp.total_pages != result.total_pages {
                            bp.total_pages = result.total_pages;
                            app.needs_save = true;
                        }
                    }
                }

                // Only update if this is still the page we want
                if app.selected_book_hash.as_deref() == Some(&result.book_hash)
                    && app.current_page == result.page
                {
                    app.current_texture = Some(texture);
                    app.error_message = None;
                }
            }
            RenderResponse::Err(e) => {
                app.error_message = Some(e);
            }
        }
    }
}
