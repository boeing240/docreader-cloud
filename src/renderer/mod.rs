pub mod cache;
pub mod djvu;
pub mod epub;
pub mod fb2;
pub mod format;
pub mod pdf;
pub mod text_render;
pub mod traits;

use std::collections::HashMap;

use format::DocumentFormat;
use traits::DocumentRenderer;

pub struct RendererRegistry {
    renderers: HashMap<DocumentFormat, Box<dyn DocumentRenderer>>,
}

impl RendererRegistry {
    pub fn new() -> Self {
        let mut renderers: HashMap<DocumentFormat, Box<dyn DocumentRenderer>> = HashMap::new();

        if let Ok(r) = pdf::PdfRenderer::new() {
            renderers.insert(DocumentFormat::Pdf, Box::new(r));
        }

        renderers.insert(
            DocumentFormat::Epub,
            Box::new(epub::EpubRenderer::default()),
        );
        renderers.insert(DocumentFormat::Fb2, Box::new(fb2::Fb2Renderer::default()));

        // DJVU not yet supported — intentionally omitted

        Self { renderers }
    }

    pub fn get(&self, format: &DocumentFormat) -> Option<&dyn DocumentRenderer> {
        self.renderers.get(format).map(|r| r.as_ref())
    }

    pub fn supports(&self, format: &DocumentFormat) -> bool {
        self.renderers.contains_key(format)
    }

    pub fn has_pdf(&self) -> bool {
        self.renderers.contains_key(&DocumentFormat::Pdf)
    }
}
