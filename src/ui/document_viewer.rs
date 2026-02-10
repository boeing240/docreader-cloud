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
                    let size = tex.size_vec2();
                    ui.image((tex.id(), size));
                });
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("Выберите книгу из библиотеки");
            });
        }
    }
}
