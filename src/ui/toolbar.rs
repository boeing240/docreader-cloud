use egui::Ui;

pub struct Toolbar;

#[allow(dead_code)]
pub enum ToolbarAction {
    PrevPage,
    NextPage,
    GoToPage(u32),
    ZoomIn,
    ZoomOut,
    ZoomReset,
}

impl Toolbar {
    pub fn show(
        ui: &mut Ui,
        current_page: u32,
        total_pages: u32,
        zoom: f32,
    ) -> Option<ToolbarAction> {
        let mut action = None;

        ui.horizontal(|ui| {
            if ui.button("◀ Назад").clicked() {
                action = Some(ToolbarAction::PrevPage);
            }

            ui.label(format!("Страница {} / {}", current_page, total_pages));

            if ui.button("Вперёд ▶").clicked() {
                action = Some(ToolbarAction::NextPage);
            }

            ui.separator();

            if ui.button("-").clicked() {
                action = Some(ToolbarAction::ZoomOut);
            }
            ui.label(format!("{:.0}%", zoom * 100.0));
            if ui.button("+").clicked() {
                action = Some(ToolbarAction::ZoomIn);
            }
            if ui.button("100%").clicked() {
                action = Some(ToolbarAction::ZoomReset);
            }
        });

        action
    }
}
