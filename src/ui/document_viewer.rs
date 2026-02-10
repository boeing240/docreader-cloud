use egui::Ui;

pub struct DocumentViewer;

impl DocumentViewer {
    pub fn show(
        ui: &mut Ui,
        texture: Option<&egui::TextureHandle>,
        _current_page: u32,
        _total_pages: u32,
    ) {
        if let Some(tex) = texture {
            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    // Texture is rendered at native pixel density;
                    // display in logical points so it appears at the correct size
                    let ppp = ui.ctx().pixels_per_point();
                    let size = tex.size_vec2() / ppp;
                    ui.image((tex.id(), size));
                });
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("Выберите книгу из библиотеки");
            });
        }
    }
}
