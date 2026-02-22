use egui::{Ui, Vec2};

pub struct DocumentViewer;

impl DocumentViewer {
    pub fn show(
        ui: &mut Ui,
        texture: Option<&egui::TextureHandle>,
        _current_page: u32,
        _total_pages: u32,
        horizontal_offset: &mut f32,
        is_first_frame: bool,
    ) {
        if let Some(tex) = texture {
            let mut scroll_area = egui::ScrollArea::both()
                .auto_shrink([false, false])
                .id_salt("document_viewer_scroll");

            // Only set initial horizontal offset on first frame to avoid breaking vertical scroll
            if is_first_frame && *horizontal_offset > 0.1 {
                scroll_area = scroll_area.scroll_offset(Vec2::new(*horizontal_offset, 0.0));
            }

            let output = scroll_area.show(ui, |ui| {
                // Texture is rendered at native pixel density;
                // display in logical points so it appears at the correct size
                let ppp = ui.ctx().pixels_per_point();
                let size = tex.size_vec2() / ppp;
                ui.image((tex.id(), size));
            });

            // Save current horizontal offset for persistence
            *horizontal_offset = output.state.offset.x;
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("Выберите книгу из библиотеки");
            });
        }
    }
}
