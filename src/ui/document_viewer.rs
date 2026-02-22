use egui::{Ui, Vec2};

pub struct DocumentViewer;

impl DocumentViewer {
    pub fn show(
        ui: &mut Ui,
        texture: Option<&egui::TextureHandle>,
        _current_page: u32,
        _total_pages: u32,
        horizontal_offset: &mut f32,
    ) {
        if let Some(tex) = texture {
            let scroll_area = egui::ScrollArea::both()
                .auto_shrink([false, false])
                .scroll_offset(Vec2::new(*horizontal_offset, 0.0));

            let output = scroll_area.show(ui, |ui| {
                // Texture is rendered at native pixel density;
                // display in logical points so it appears at the correct size
                let ppp = ui.ctx().pixels_per_point();
                let size = tex.size_vec2() / ppp;
                ui.image((tex.id(), size));
            });

            // Save horizontal scroll offset
            *horizontal_offset = output.state.offset.x;
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("Выберите книгу из библиотеки");
            });
        }
    }
}
