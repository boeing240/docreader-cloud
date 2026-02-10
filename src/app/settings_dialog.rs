use egui::Context;
use std::path::PathBuf;

use crate::library::progress::ReadingProgress;
use crate::sync::storage::ProgressStorage;
use crate::sync::watcher::SyncWatcher;

use super::book_manager;
use super::DocReaderApp;

pub(crate) fn show_settings_window(app: &mut DocReaderApp, ctx: &Context) {
    let mut show = app.show_settings;
    egui::Window::new("Настройки")
        .open(&mut show)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Папка библиотеки:");
                ui.text_edit_singleline(&mut app.settings_library_path);
            });

            ui.horizontal(|ui| {
                ui.label("Файл прогресса:");
                ui.text_edit_singleline(&mut app.settings_progress_path);
            });

            ui.horizontal(|ui| {
                ui.label("ID устройства:");
                ui.label(&app.settings.device_id);
            });

            ui.separator();

            if ui.button("Сохранить настройки").clicked() {
                app.settings.library_path = PathBuf::from(&app.settings_library_path);
                app.settings.progress_file_path = PathBuf::from(&app.settings_progress_path);

                if let Err(e) = app.settings.save() {
                    app.error_message = Some(format!("Ошибка сохранения настроек: {}", e));
                } else {
                    app.storage = ProgressStorage::new(app.settings.progress_file_path.clone());
                    app.progress = app
                        .storage
                        .load_or_create(&app.settings.device_id)
                        .unwrap_or_else(|_| ReadingProgress::new(app.settings.device_id.clone()));
                    app.watcher = SyncWatcher::new(&app.settings.progress_file_path).ok();
                    book_manager::rescan_library(app);
                    app.show_settings = false;
                }
            }

            if ui.button("Пересканировать библиотеку").clicked() {
                book_manager::rescan_library(app);
            }
        });
    app.show_settings = show;
}
