use egui::{Ui, Vec2};

pub struct DocumentViewer;

impl DocumentViewer {
    pub fn show(
        ui: &mut Ui,
        texture: Option<&egui::TextureHandle>,
        _current_page: u32,
        _total_pages: u32,
        scroll_offset: &mut (f32, f32),
    ) {
        if let Some(tex) = texture {
            let ppp = ui.ctx().pixels_per_point();
            let size = tex.size_vec2() / ppp;
            let available = ui.available_size();

            // Center by default if scroll offset is (0, 0)
            let mut offset = Vec2::new(scroll_offset.0, scroll_offset.1);
            if scroll_offset.0 == 0.0 && scroll_offset.1 == 0.0 {
                // Calculate centered offset
                let center_x = ((size.x - available.x) / 2.0).max(0.0);
                let center_y = ((size.y - available.y) / 2.0).max(0.0);
                offset = Vec2::new(center_x, center_y);
            }

            let scroll_area = egui::ScrollArea::both()
                .auto_shrink([false, false])
                .scroll_offset(offset);

            let output = scroll_area.show(ui, |ui| {
                // Texture is rendered at native pixel density;
                // display in logical points so it appears at the correct size
                // Simply display the image - egui handles pixel alignment internally
                ui.image((tex.id(), size));
            });

            // Save scroll offset
            *scroll_offset = (output.state.offset.x, output.state.offset.y);
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("Выберите книгу из библиотеки");
            });
        }
    }
}
