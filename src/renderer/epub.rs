use anyhow::{Context, Result};
use image::RgbaImage;
use rbook::Ebook;
use std::collections::HashMap;
use std::io::Write;
use std::sync::Mutex;

use super::text_render::TextPageRenderer;
use super::traits::DocumentRenderer;

struct CachedDocument {
    pages: Vec<Vec<String>>,
    scale: f32,
}

pub struct EpubRenderer {
    text_renderer: TextPageRenderer,
    cache: Mutex<HashMap<u64, CachedDocument>>,
}

impl Default for EpubRenderer {
    fn default() -> Self {
        Self {
            text_renderer: TextPageRenderer::new(),
            cache: Mutex::new(HashMap::new()),
        }
    }
}

impl EpubRenderer {
    fn extract_paragraphs(bytes: &[u8]) -> Result<Vec<String>> {
        // Write bytes to a temp file since rbook requires a file path
        let temp_dir = std::env::temp_dir().join("docreader-cloud");
        std::fs::create_dir_all(&temp_dir)?;
        let temp_path = temp_dir.join("_temp_epub.epub");
        {
            let mut f = std::fs::File::create(&temp_path)?;
            f.write_all(bytes)?;
        }

        let epub = rbook::Epub::new(&temp_path).context("Не удалось открыть EPUB")?;

        let mut paragraphs = Vec::new();
        let mut reader = epub.reader();

        // Read first page
        if let Ok(content) = reader.current_page() {
            let text_paragraphs = Self::strip_html_to_paragraphs(&content.as_lossy_str());
            paragraphs.extend(text_paragraphs);
        }

        // Read remaining pages
        while let Some(content) = reader.next_page() {
            let text_paragraphs = Self::strip_html_to_paragraphs(&content.as_lossy_str());
            paragraphs.extend(text_paragraphs);
        }

        // Clean up temp file (best effort)
        let _ = std::fs::remove_file(&temp_path);

        Ok(paragraphs)
    }

    /// Simple HTML tag stripper that extracts text content, splitting by block elements.
    fn strip_html_to_paragraphs(html: &str) -> Vec<String> {
        let mut paragraphs = Vec::new();
        let mut current = String::new();
        let mut in_tag = false;
        let mut tag_name = String::new();

        let block_tags = [
            "p",
            "div",
            "h1",
            "h2",
            "h3",
            "h4",
            "h5",
            "h6",
            "br",
            "li",
            "tr",
            "blockquote",
        ];

        for ch in html.chars() {
            match ch {
                '<' => {
                    in_tag = true;
                    tag_name.clear();
                }
                '>' if in_tag => {
                    in_tag = false;
                    let tag_lower = tag_name.to_lowercase();
                    // Strip leading '/' for closing tags
                    let tag_base = tag_lower
                        .trim_start_matches('/')
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .to_string();

                    if block_tags.contains(&tag_base.as_str()) {
                        let trimmed = current.trim().to_string();
                        if !trimmed.is_empty() {
                            paragraphs.push(trimmed);
                        }
                        current.clear();
                    }
                }
                _ if in_tag => {
                    tag_name.push(ch);
                }
                _ => {
                    current.push(ch);
                }
            }
        }

        let trimmed = current.trim().to_string();
        if !trimmed.is_empty() {
            paragraphs.push(trimmed);
        }

        // Decode basic HTML entities
        paragraphs
            .into_iter()
            .map(|p| {
                p.replace("&amp;", "&")
                    .replace("&lt;", "<")
                    .replace("&gt;", ">")
                    .replace("&quot;", "\"")
                    .replace("&apos;", "'")
                    .replace("&nbsp;", " ")
            })
            .filter(|p| !p.is_empty())
            .collect()
    }

    fn bytes_hash(bytes: &[u8]) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        bytes.len().hash(&mut hasher);
        if bytes.len() >= 256 {
            bytes[..256].hash(&mut hasher);
        } else {
            bytes.hash(&mut hasher);
        }
        hasher.finish()
    }

    fn get_pages(&self, bytes: &[u8], scale: f32) -> Result<Vec<Vec<String>>> {
        let hash = Self::bytes_hash(bytes);

        {
            let cache = self.cache.lock().unwrap();
            if let Some(cached) = cache.get(&hash) {
                if (cached.scale - scale).abs() < 0.01 {
                    return Ok(cached.pages.clone());
                }
            }
        }

        let paragraphs = Self::extract_paragraphs(bytes)?;
        let pages = self.text_renderer.paginate(&paragraphs, scale);

        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(
                hash,
                CachedDocument {
                    pages: pages.clone(),
                    scale,
                },
            );
        }

        Ok(pages)
    }
}

impl DocumentRenderer for EpubRenderer {
    fn get_page_count(&self, bytes: &[u8]) -> Result<u32> {
        let pages = self.get_pages(bytes, 1.0)?;
        Ok(pages.len() as u32)
    }

    fn render_page(&self, bytes: &[u8], page_index: u32, scale: f32) -> Result<RgbaImage> {
        let pages = self.get_pages(bytes, scale)?;
        let idx = page_index as usize;
        if idx >= pages.len() {
            anyhow::bail!(
                "Страница {} за пределами документа ({} стр.)",
                page_index + 1,
                pages.len()
            );
        }
        Ok(self.text_renderer.render_page(&pages[idx], scale))
    }
}
