use anyhow::{Context, Result};
use image::{DynamicImage, RgbaImage};
use once_cell::sync::Lazy;
use pdfium_render::prelude::*;
use std::path::{Path, PathBuf};

use super::traits::DocumentRenderer;

static EMBEDDED_PDFIUM: &[u8] = include_bytes!("../../libs/pdfium.dll");

static PDFIUM_DLL_PATH: Lazy<Result<PathBuf, String>> = Lazy::new(|| {
    let dir = std::env::temp_dir().join("docreader-cloud");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let dll_path = dir.join("pdfium.dll");

    // Write only if missing or size differs (new version)
    let needs_write = match std::fs::metadata(&dll_path) {
        Ok(meta) => meta.len() != EMBEDDED_PDFIUM.len() as u64,
        Err(_) => true,
    };

    if needs_write {
        std::fs::write(&dll_path, EMBEDDED_PDFIUM).map_err(|e| e.to_string())?;
    }

    Ok(dll_path)
});

pub struct PdfRenderer {
    pdfium: Pdfium,
}

impl PdfRenderer {
    pub fn new() -> Result<Self> {
        let dll_path = PDFIUM_DLL_PATH
            .as_ref()
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(
            dll_path.parent().unwrap_or(Path::new(".")),
        ))
        .or_else(|_| Pdfium::bind_to_system_library())
        .context("Failed to load pdfium library")?;

        Ok(Self {
            pdfium: Pdfium::new(bindings),
        })
    }
}

impl DocumentRenderer for PdfRenderer {
    fn get_page_count(&self, bytes: &[u8]) -> Result<u32> {
        let document = self
            .pdfium
            .load_pdf_from_byte_slice(bytes, None)
            .context("Failed to load PDF")?;

        Ok(document.pages().len() as u32)
    }

    fn render_page(&self, bytes: &[u8], page_index: u32, scale: f32) -> Result<RgbaImage> {
        let document = self
            .pdfium
            .load_pdf_from_byte_slice(bytes, None)
            .context("Failed to load PDF")?;

        let pages = document.pages();
        let page_index_u16: u16 = page_index.try_into().map_err(|_| {
            anyhow::anyhow!(
                "Page index {} exceeds maximum supported (65535)",
                page_index
            )
        })?;
        let page = pages
            .get(page_index_u16)
            .context("Page index out of bounds")?;

        let width = (page.width().value * scale) as i32;
        let height = (page.height().value * scale) as i32;

        let config = PdfRenderConfig::new()
            .set_target_width(width)
            .set_target_height(height);

        let bitmap = page
            .render_with_config(&config)
            .context("Failed to render page")?;

        let dynamic_image: DynamicImage = bitmap.as_image();
        Ok(dynamic_image.to_rgba8())
    }
}
