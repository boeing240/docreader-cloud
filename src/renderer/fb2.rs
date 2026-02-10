use anyhow::{Context, Result};
use image::RgbaImage;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::sync::Mutex;

use super::text_render::TextPageRenderer;
use super::traits::DocumentRenderer;

/// Cached pagination result for a document.
struct CachedDocument {
    pages: Vec<Vec<String>>,
    scale: f32,
}

pub struct Fb2Renderer {
    text_renderer: TextPageRenderer,
    cache: Mutex<HashMap<u64, CachedDocument>>,
}

impl Fb2Renderer {
    fn parse_paragraphs(bytes: &[u8]) -> Result<Vec<String>> {
        let text = std::str::from_utf8(bytes).context("FB2 файл не является валидным UTF-8")?;
        let mut reader = Reader::from_str(text);

        let mut paragraphs = Vec::new();
        let mut current_text = String::new();
        let mut in_body = false;
        let mut in_p = false;
        let mut depth = 0u32;

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    match name.as_str() {
                        "body" => {
                            in_body = true;
                        }
                        "p" if in_body => {
                            in_p = true;
                            current_text.clear();
                        }
                        "section" if in_body => {
                            depth += 1;
                        }
                        "title" if in_body => {
                            in_p = true;
                            current_text.clear();
                        }
                        "empty-line" if in_body => {
                            paragraphs.push(String::new());
                        }
                        _ => {}
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    match name.as_str() {
                        "body" => {
                            in_body = false;
                        }
                        "p" | "title" if in_p => {
                            in_p = false;
                            let trimmed = current_text.trim().to_string();
                            if !trimmed.is_empty() {
                                paragraphs.push(trimmed);
                            }
                            current_text.clear();
                        }
                        "section" if in_body && depth > 0 => {
                            depth -= 1;
                        }
                        _ => {}
                    }
                }
                Ok(Event::Text(ref e)) if in_p => {
                    if let Ok(t) = e.unescape() {
                        if !current_text.is_empty() {
                            current_text.push(' ');
                        }
                        current_text.push_str(t.trim());
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    anyhow::bail!("Ошибка парсинга FB2: {}", e);
                }
                _ => {}
            }
        }

        Ok(paragraphs)
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

        let paragraphs = Self::parse_paragraphs(bytes)?;
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

impl Default for Fb2Renderer {
    fn default() -> Self {
        Self {
            text_renderer: TextPageRenderer::new(),
            cache: Mutex::new(HashMap::new()),
        }
    }
}

impl DocumentRenderer for Fb2Renderer {
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
