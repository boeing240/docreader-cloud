use anyhow::Result;
use image::RgbaImage;

pub trait DocumentRenderer {
    fn get_page_count(&self, bytes: &[u8]) -> Result<u32>;
    fn render_page(&self, bytes: &[u8], page_index: u32, scale: f32) -> Result<RgbaImage>;
}
