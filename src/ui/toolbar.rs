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
        _current_page: u32,
        total_pages: u32,
        zoom: f32,
        page_input: &mut String,
    ) -> Option<ToolbarAction> {
        let mut action = None;

        ui.horizontal(|ui| {
            if ui.button("◀ Назад").clicked() {
                action = Some(ToolbarAction::PrevPage);
            }

            ui.label("Стр.");
            let response = ui.add(
                egui::TextEdit::singleline(page_input)
                    .desired_width(40.0)
                    .horizontal_align(egui::Align::Center),
            );
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if let Ok(page) = page_input.trim().parse::<u32>() {
                    action = Some(ToolbarAction::GoToPage(page));
                }
            }
            ui.label(format!("/ {}", total_pages));

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
