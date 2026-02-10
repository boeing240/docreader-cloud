use image::RgbaImage;
use std::sync::mpsc;
use std::sync::Arc;

use crate::config::constants::PDF_SCALE_MULTIPLIER;
use crate::renderer::format::DocumentFormat;
use crate::renderer::RendererRegistry;

pub(crate) struct RenderRequest {
    pub book_hash: String,
    pub page: u32,
    pub page_index: u32,
    pub zoom: f32,
    pub dpi: u32,
    pub bytes: Arc<Vec<u8>>,
    pub format: DocumentFormat,
}

pub(crate) struct RenderResult {
    pub book_hash: String,
    pub page: u32,
    pub dpi: u32,
    pub total_pages: u32,
    pub image: RgbaImage,
}

pub(crate) enum RenderResponse {
    Ok(RenderResult),
    Err(String),
}

pub(crate) fn spawn_render_thread() -> (mpsc::Sender<RenderRequest>, mpsc::Receiver<RenderResponse>)
{
    let (render_tx, render_rx) = mpsc::channel::<RenderRequest>();
    let (result_tx, result_rx) = mpsc::channel::<RenderResponse>();

    std::thread::spawn(move || {
        let registry = RendererRegistry::new();

        while let Ok(mut req) = render_rx.recv() {
            // Drain queue -- skip stale requests, keep only the latest
            while let Ok(newer) = render_rx.try_recv() {
                req = newer;
            }

            let Some(renderer) = registry.get(&req.format) else {
                let msg = format!("Формат {} не поддерживается", req.format.display_name());
                if result_tx.send(RenderResponse::Err(msg)).is_err() {
                    break;
                }
                continue;
            };

            let total_pages = renderer.get_page_count(&req.bytes).unwrap_or(0);

            let scale = if req.format == DocumentFormat::Pdf {
                req.zoom * PDF_SCALE_MULTIPLIER
            } else {
                req.zoom
            };

            let response = match renderer.render_page(&req.bytes, req.page_index, scale) {
                Ok(image) => RenderResponse::Ok(RenderResult {
                    book_hash: req.book_hash,
                    page: req.page,
                    dpi: req.dpi,
                    total_pages,
                    image,
                }),
                Err(e) => RenderResponse::Err(format!("Ошибка рендеринга: {}", e)),
            };
            if result_tx.send(response).is_err() {
                break;
            }
        }
    });

    (render_tx, result_rx)
}
