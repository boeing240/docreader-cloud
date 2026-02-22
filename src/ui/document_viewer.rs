use egui::{Ui, Vec2};

pub struct DocumentViewer;

impl DocumentViewer {
    pub fn show(
        ui: &mut Ui,
        texture: Option<&egui::TextureHandle>,
        _current_page: u32,
        _total_pages: u32,
        horizontal_offset: &mut f32,
        _is_first_frame: bool,
    ) {
        if let Some(tex) = texture {
            // Get current scroll state
            let scroll_id = ui.id().with("document_viewer_scroll");
            let saved_vertical = egui::scroll_area::State::load(ui.ctx(), scroll_id)
                .map(|s| s.offset.y)
                .unwrap_or(0.0);

            // Always apply saved horizontal offset, preserve vertical from egui state
            let scroll_area = egui::ScrollArea::both()
                .auto_shrink([false, false])
                .id_salt("document_viewer_scroll")
                .scroll_offset(Vec2::new(*horizontal_offset, saved_vertical));

            let output = scroll_area.show(ui, |ui| {
                // Texture is rendered at native pixel density;
                // display in logical points so it appears at the correct size
                let ppp = ui.ctx().pixels_per_point();
                let size = tex.size_vec2() / ppp;
                ui.image((tex.id(), size));
            });

            // Save current horizontal offset for next frame/session
            *horizontal_offset = output.state.offset.x;
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("Выберите книгу из библиотеки");
            });
        }
    }
}
