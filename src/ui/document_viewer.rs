use egui::Ui;

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
            // Use persistent ID so ScrollArea remembers its state
            let scroll_area = egui::ScrollArea::both()
                .auto_shrink([false, false])
                .id_salt("document_viewer_scroll");

            let output = scroll_area.show(ui, |ui| {
                // Texture is rendered at native pixel density;
                // display in logical points so it appears at the correct size
                let ppp = ui.ctx().pixels_per_point();
                let size = tex.size_vec2() / ppp;
                ui.image((tex.id(), size));
            });

            // Just save current offset for external persistence
            // Don't try to force it back - let egui handle it
            *horizontal_offset = output.state.offset.x;
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("Выберите книгу из библиотеки");
            });
        }
    }
}
