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
            // Get the scroll area state ID
            let scroll_id = ui.id().with("document_viewer_scroll");

            // Check if we have saved horizontal offset to restore
            let mut initial_offset = None;
            if let Some(state) = egui::scroll_area::State::load(ui.ctx(), scroll_id) {
                // If saved offset is different from what we want, prepare to override
                if (*horizontal_offset - state.offset.x).abs() > 1.0 {
                    initial_offset = Some(Vec2::new(*horizontal_offset, state.offset.y));
                }
            } else if *horizontal_offset > 0.1 {
                // First time, apply saved offset
                initial_offset = Some(Vec2::new(*horizontal_offset, 0.0));
            }

            let mut scroll_area = egui::ScrollArea::both()
                .auto_shrink([false, false])
                .id_salt("document_viewer_scroll");

            // Apply initial offset if needed
            if let Some(offset) = initial_offset {
                scroll_area = scroll_area.scroll_offset(offset);
            }

            let output = scroll_area.show(ui, |ui| {
                // Texture is rendered at native pixel density;
                // display in logical points so it appears at the correct size
                let ppp = ui.ctx().pixels_per_point();
                let size = tex.size_vec2() / ppp;
                ui.image((tex.id(), size));
            });

            // Always save current horizontal offset
            *horizontal_offset = output.state.offset.x;
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("Выберите книгу из библиотеки");
            });
        }
    }
}
