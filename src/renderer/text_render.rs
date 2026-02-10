use ab_glyph::{Font, FontRef, PxScale, PxScaleFont, ScaleFont};
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;

use crate::config::constants::*;

static EMBEDDED_FONT: &[u8] = include_bytes!("../../libs/fonts/NotoSans-Regular.ttf");

pub struct TextPageRenderer {
    font: FontRef<'static>,
}

impl TextPageRenderer {
    pub fn new() -> Self {
        let font = FontRef::try_from_slice(EMBEDDED_FONT).expect("Failed to load embedded font");
        Self { font }
    }

    /// Paginate text paragraphs into virtual pages.
    /// Returns a Vec of pages, where each page is a Vec of lines (strings).
    pub fn paginate(&self, paragraphs: &[String], scale: f32) -> Vec<Vec<String>> {
        let font_size = TEXT_FONT_SIZE * scale;
        let line_height = TEXT_LINE_HEIGHT * scale;
        let paragraph_spacing = TEXT_PARAGRAPH_SPACING * scale;
        let margin = TEXT_PAGE_MARGIN as f32 * scale;
        let page_height = TEXT_PAGE_HEIGHT as f32 * scale;
        let page_width = TEXT_PAGE_WIDTH as f32 * scale;
        let usable_width = page_width - 2.0 * margin;
        let usable_height = page_height - 2.0 * margin;

        let px_scale = PxScale::from(font_size);
        let scaled_font = self.font.as_scaled(px_scale);

        let mut pages: Vec<Vec<String>> = Vec::new();
        let mut current_page: Vec<String> = Vec::new();
        let mut y = 0.0f32;

        for paragraph in paragraphs {
            let lines = Self::wrap_text(paragraph, usable_width, &scaled_font);

            for (i, line) in lines.iter().enumerate() {
                let extra = if i == lines.len() - 1 {
                    paragraph_spacing
                } else {
                    0.0
                };

                if y + line_height > usable_height && !current_page.is_empty() {
                    pages.push(current_page);
                    current_page = Vec::new();
                    y = 0.0;
                }

                current_page.push(line.clone());
                y += line_height + extra;
            }
        }

        if !current_page.is_empty() {
            pages.push(current_page);
        }

        if pages.is_empty() {
            pages.push(vec!["(Пустой документ)".to_string()]);
        }

        pages
    }

    /// Render a single page (given its lines) to an RgbaImage.
    pub fn render_page(&self, lines: &[String], scale: f32) -> RgbaImage {
        let width = (TEXT_PAGE_WIDTH as f32 * scale) as u32;
        let height = (TEXT_PAGE_HEIGHT as f32 * scale) as u32;
        let margin = (TEXT_PAGE_MARGIN as f32 * scale) as i32;
        let font_size = TEXT_FONT_SIZE * scale;
        let line_height = TEXT_LINE_HEIGHT * scale;

        let mut image = RgbaImage::from_pixel(width, height, Rgba([255, 255, 255, 255]));
        let color = Rgba([0, 0, 0, 255]);
        let px_scale = PxScale::from(font_size);

        for (i, line) in lines.iter().enumerate() {
            let x = margin;
            let y = margin + (i as f32 * line_height) as i32;
            draw_text_mut(&mut image, color, x, y, px_scale, &self.font, line);
        }

        image
    }

    fn wrap_text(
        text: &str,
        max_width: f32,
        scaled_font: &PxScaleFont<&FontRef<'_>>,
    ) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0.0f32;

        for word in text.split_whitespace() {
            let word_width = Self::measure_text(word, scaled_font);

            if current_line.is_empty() {
                current_line = word.to_string();
                current_width = word_width;
            } else {
                let space_width = Self::measure_text(" ", scaled_font);
                if current_width + space_width + word_width <= max_width {
                    current_line.push(' ');
                    current_line.push_str(word);
                    current_width += space_width + word_width;
                } else {
                    lines.push(current_line);
                    current_line = word.to_string();
                    current_width = word_width;
                }
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }

    fn measure_text(text: &str, scaled_font: &PxScaleFont<&FontRef<'_>>) -> f32 {
        text.chars()
            .map(|c| {
                let glyph_id = scaled_font.glyph_id(c);
                scaled_font.h_advance(glyph_id)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn renderer() -> TextPageRenderer {
        TextPageRenderer::new()
    }

    #[test]
    fn test_paginate_empty() {
        let r = renderer();
        let pages = r.paginate(&[], 1.0);
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0][0], "(Пустой документ)");
    }

    #[test]
    fn test_paginate_single_paragraph() {
        let r = renderer();
        let pages = r.paginate(&["Hello world".to_string()], 1.0);
        assert!(!pages.is_empty());
        assert!(pages[0].iter().any(|l| l.contains("Hello")));
    }

    #[test]
    fn test_paginate_creates_multiple_pages() {
        let r = renderer();
        // Create enough text to overflow one page
        let long_paragraphs: Vec<String> = (0..200)
            .map(|i| {
                format!(
                    "Paragraph number {} with some extra text to fill the page.",
                    i
                )
            })
            .collect();
        let pages = r.paginate(&long_paragraphs, 1.0);
        assert!(
            pages.len() > 1,
            "Expected multiple pages, got {}",
            pages.len()
        );
    }

    #[test]
    fn test_paginate_scale_affects_pages() {
        let r = renderer();
        let paragraphs: Vec<String> = (0..100)
            .map(|i| format!("Line {} with content.", i))
            .collect();
        let pages_small = r.paginate(&paragraphs, 0.5);
        let pages_large = r.paginate(&paragraphs, 2.0);
        // Larger scale = larger text = more pages
        assert!(
            pages_large.len() >= pages_small.len(),
            "Large scale {} should produce >= pages than small scale {}",
            pages_large.len(),
            pages_small.len()
        );
    }

    #[test]
    fn test_render_page_dimensions() {
        let r = renderer();
        let lines = vec!["Test line".to_string()];
        let image = r.render_page(&lines, 1.0);
        assert_eq!(image.width(), TEXT_PAGE_WIDTH);
        assert_eq!(image.height(), TEXT_PAGE_HEIGHT);
    }

    #[test]
    fn test_render_page_scaled_dimensions() {
        let r = renderer();
        let lines = vec!["Test".to_string()];
        let image = r.render_page(&lines, 2.0);
        assert_eq!(image.width(), TEXT_PAGE_WIDTH * 2);
        assert_eq!(image.height(), TEXT_PAGE_HEIGHT * 2);
    }

    #[test]
    fn test_render_page_white_background() {
        let r = renderer();
        let lines = vec![];
        let image = r.render_page(&lines, 1.0);
        // Corner pixel should be white
        let pixel = image.get_pixel(0, 0);
        assert_eq!(pixel, &Rgba([255, 255, 255, 255]));
    }

    #[test]
    fn test_wrap_text_short() {
        let r = renderer();
        let px_scale = PxScale::from(TEXT_FONT_SIZE);
        let scaled_font = r.font.as_scaled(px_scale);
        let lines = TextPageRenderer::wrap_text("Hello", 1000.0, &scaled_font);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "Hello");
    }

    #[test]
    fn test_wrap_text_empty() {
        let r = renderer();
        let px_scale = PxScale::from(TEXT_FONT_SIZE);
        let scaled_font = r.font.as_scaled(px_scale);
        let lines = TextPageRenderer::wrap_text("", 1000.0, &scaled_font);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "");
    }

    #[test]
    fn test_wrap_text_wraps_long_line() {
        let r = renderer();
        let px_scale = PxScale::from(TEXT_FONT_SIZE);
        let scaled_font = r.font.as_scaled(px_scale);
        let long_text = "word ".repeat(100);
        let lines = TextPageRenderer::wrap_text(&long_text.trim(), 200.0, &scaled_font);
        assert!(
            lines.len() > 1,
            "Expected wrapping, got {} lines",
            lines.len()
        );
    }

    #[test]
    fn test_measure_text_positive() {
        let r = renderer();
        let px_scale = PxScale::from(TEXT_FONT_SIZE);
        let scaled_font = r.font.as_scaled(px_scale);
        let width = TextPageRenderer::measure_text("Hello", &scaled_font);
        assert!(width > 0.0);
    }

    #[test]
    fn test_measure_text_empty() {
        let r = renderer();
        let px_scale = PxScale::from(TEXT_FONT_SIZE);
        let scaled_font = r.font.as_scaled(px_scale);
        let width = TextPageRenderer::measure_text("", &scaled_font);
        assert!((width - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_measure_text_longer_is_wider() {
        let r = renderer();
        let px_scale = PxScale::from(TEXT_FONT_SIZE);
        let scaled_font = r.font.as_scaled(px_scale);
        let w1 = TextPageRenderer::measure_text("Hi", &scaled_font);
        let w2 = TextPageRenderer::measure_text("Hello World", &scaled_font);
        assert!(w2 > w1);
    }
}
