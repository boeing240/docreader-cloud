use anyhow::Result;
use image::RgbaImage;

use super::traits::DocumentRenderer;

pub struct DjvuRenderer;

impl DjvuRenderer {
    pub fn new() -> Result<Self> {
        anyhow::bail!("Формат DJVU пока не поддерживается")
    }
}

impl DocumentRenderer for DjvuRenderer {
    fn get_page_count(&self, _bytes: &[u8]) -> Result<u32> {
        anyhow::bail!("Формат DJVU пока не поддерживается")
    }

    fn render_page(&self, _bytes: &[u8], _page_index: u32, _scale: f32) -> Result<RgbaImage> {
        anyhow::bail!("Формат DJVU пока не поддерживается")
    }
}
