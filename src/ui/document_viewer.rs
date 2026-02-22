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

            // Center on first load only (when both offsets are exactly 0.0)
            // After user scrolls, save actual position even if it's near 0
            let should_center = scroll_offset.0 == 0.0 && scroll_offset.1 == 0.0;

            let offset = if should_center {
                // Calculate centered offset for first display
                let center_x = ((size.x - available.x) / 2.0).max(0.0);
                let center_y = ((size.y - available.y) / 2.0).max(0.0);
                Vec2::new(center_x, center_y)
            } else {
                Vec2::new(scroll_offset.0, scroll_offset.1)
            };

            let scroll_area = egui::ScrollArea::both()
                .auto_shrink([false, false])
                .scroll_offset(offset);

            let output = scroll_area.show(ui, |ui| {
                // Texture is rendered at native pixel density;
                // display in logical points so it appears at the correct size
                // Simply display the image - egui handles pixel alignment internally
                ui.image((tex.id(), size));
            });

            // Save actual scroll offset from ScrollArea
            // Only update if it actually changed to avoid feedback loops
            let actual_offset = output.state.offset;
            let new_offset = (actual_offset.x, actual_offset.y);

            // Update scroll_offset if it changed significantly (more than 0.1 pixel)
            if (new_offset.0 - scroll_offset.0).abs() > 0.1
                || (new_offset.1 - scroll_offset.1).abs() > 0.1
            {
                *scroll_offset = new_offset;
            }
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("Выберите книгу из библиотеки");
            });
        }
    }
}
