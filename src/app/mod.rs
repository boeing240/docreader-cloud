mod book_manager;
mod input_handler;
mod progress_manager;
mod render_manager;
mod render_thread;
mod settings_dialog;

use egui::{Context, TextureHandle};
use std::sync::mpsc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::config::constants::*;
use crate::config::settings::AppSettings;
use crate::library::book::Book;
use crate::library::progress::ReadingProgress;
use crate::library::scanner::LibraryScanner;
use crate::renderer::cache::PageCache;
use crate::sync::storage::ProgressStorage;
use crate::sync::watcher::SyncWatcher;
use crate::ui::document_viewer::DocumentViewer;
use crate::ui::sidebar::Sidebar;
use crate::ui::toolbar::Toolbar;

use render_thread::{RenderRequest, RenderResponse};

pub struct DocReaderApp {
    // Settings
    pub(crate) settings: AppSettings,

    // Library
    pub(crate) books: Vec<Book>,
    pub(crate) progress: ReadingProgress,

    // Current state
    pub(crate) selected_book_hash: Option<String>,
    pub(crate) current_page: u32,
    pub(crate) current_texture: Option<TextureHandle>,
    pub(crate) current_document_bytes: Option<Arc<Vec<u8>>>,

    // Services
    pub(crate) storage: ProgressStorage,
    pub(crate) watcher: Option<SyncWatcher>,
    pub(crate) page_cache: PageCache,

    // Async rendering
    pub(crate) render_tx: mpsc::Sender<RenderRequest>,
    pub(crate) result_rx: mpsc::Receiver<RenderResponse>,
    pub(crate) is_rendering: bool,
    first_frame: bool,

    // UI state
    pub(crate) zoom: f32,
    pub(crate) pixels_per_point: f32,
    pub(crate) last_save: Instant,
    pub(crate) needs_save: bool,
    pub(crate) horizontal_scroll_offset: f32,

    // Settings dialog
    pub(crate) show_settings: bool,
    pub(crate) settings_library_path: String,
    pub(crate) settings_progress_path: String,

    // Page navigation input
    pub(crate) page_input: String,

    // Error display
    pub(crate) error_message: Option<String>,
}

impl DocReaderApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let settings = AppSettings::load().unwrap_or_default();

        let storage = ProgressStorage::new(settings.progress_file_path.clone());
        let progress = storage
            .load_or_create(&settings.device_id)
            .unwrap_or_else(|_| ReadingProgress::new(settings.device_id.clone()));

        let mut books =
            LibraryScanner::scan_and_load_books(&settings.library_path, None).unwrap_or_default();

        // Restore total_pages from saved progress
        for book in &mut books {
            if let Some(bp) = progress.books.get(&book.file_hash) {
                if bp.total_pages > 0 {
                    book.total_pages = bp.total_pages;
                }
            }
        }

        let watcher = SyncWatcher::new(&settings.progress_file_path).ok();

        let (render_tx, result_rx) = render_thread::spawn_render_thread();

        // Save horizontal scroll offset before moving settings
        let horizontal_scroll_offset = settings.horizontal_scroll_offset;

        Self {
            settings_library_path: settings.library_path.to_string_lossy().to_string(),
            settings_progress_path: settings.progress_file_path.to_string_lossy().to_string(),
            settings,
            books,
            progress,
            selected_book_hash: None,
            current_page: 1,
            current_texture: None,
            current_document_bytes: None,
            storage,
            watcher,
            page_cache: PageCache::new(PAGE_CACHE_CAPACITY),
            render_tx,
            result_rx,
            is_rendering: false,
            first_frame: true,
            zoom: ZOOM_DEFAULT,
            pixels_per_point: 1.0,
            last_save: Instant::now(),
            needs_save: false,
            horizontal_scroll_offset,
            show_settings: false,
            page_input: "1".to_string(),
            error_message: None,
        }
    }

    fn selected_book(&self) -> Option<&Book> {
        self.selected_book_hash
            .as_ref()
            .and_then(|h| self.books.iter().find(|b| &b.file_hash == h))
    }
}

impl eframe::App for DocReaderApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Save first_frame status before clearing it
        let is_first_frame = self.first_frame;

        // Restore last opened book on first frame
        if self.first_frame {
            self.first_frame = false;
            self.pixels_per_point = ctx.pixels_per_point();
            if let Some(book_hash) = self.settings.last_opened_book.clone() {
                if self.books.iter().any(|b| b.file_hash == book_hash) {
                    book_manager::select_book(self, ctx, &book_hash);
                }
            }
        }

        // Track HiDPI scale factor; re-render if it changed
        let ppp = ctx.pixels_per_point();
        if (ppp - self.pixels_per_point).abs() > HIDPI_CHANGE_THRESHOLD {
            self.pixels_per_point = ppp;
            self.page_cache.clear();
            render_manager::request_render(self);
        }

        // Background tasks
        progress_manager::check_sync(self);
        render_manager::poll_render_results(self, ctx);

        // Keyboard input
        input_handler::handle_keyboard_input(self, ctx);

        // Top panel (toolbar)
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Настройки").clicked() {
                    self.show_settings = true;
                }
                ui.separator();

                let total_pages = self.selected_book().map(|b| b.total_pages).unwrap_or(0);

                if let Some(action) = Toolbar::show(
                    ui,
                    self.current_page,
                    total_pages,
                    self.zoom,
                    &mut self.page_input,
                ) {
                    book_manager::handle_toolbar_action(self, action);
                }
            });
        });

        // Bottom panel (status bar)
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(err) = &self.error_message {
                    ui.colored_label(egui::Color32::RED, err);
                } else if self.is_rendering {
                    ui.label("Загрузка страницы...");
                } else if self.needs_save {
                    ui.label("Сохранение...");
                } else {
                    ui.label("Готово");
                }

                ui.separator();

                if let Some(book) = self.selected_book() {
                    ui.label(book.format.display_name());
                    ui.separator();
                }

                ui.label(format!(
                    "Устройство: {}",
                    &self.settings.device_id
                        [..DEVICE_ID_DISPLAY_LEN.min(self.settings.device_id.len())]
                ));
                ui.separator();
                ui.label(format!("Книг: {}", self.books.len()));
            });
        });

        // Left panel (library sidebar)
        egui::SidePanel::left("library")
            .resizable(true)
            .default_width(SIDEBAR_DEFAULT_WIDTH)
            .min_width(SIDEBAR_MIN_WIDTH)
            .show(ctx, |ui| {
                let selected = self.selected_book_hash.clone();
                let mut new_selection = None;

                Sidebar::show(
                    ui,
                    &self.books,
                    &self.progress.books,
                    selected.as_deref(),
                    &mut |hash| {
                        new_selection = Some(hash.to_string());
                    },
                );

                if let Some(hash) = new_selection {
                    book_manager::select_book(self, ctx, &hash);
                }
            });

        // Central panel (viewer)
        egui::CentralPanel::default().show(ctx, |ui| {
            let total_pages = self.selected_book().map(|b| b.total_pages).unwrap_or(0);

            DocumentViewer::show(
                ui,
                self.current_texture.as_ref(),
                self.current_page,
                total_pages,
                &mut self.horizontal_scroll_offset,
                is_first_frame,
            );
        });

        // Save horizontal scroll offset to settings
        self.settings.horizontal_scroll_offset = self.horizontal_scroll_offset;

        // Settings window
        if self.show_settings {
            settings_dialog::show_settings_window(self, ctx);
        }

        // Auto-save progress
        progress_manager::maybe_save_progress(self);

        // Request repaint for smooth updates
        ctx.request_repaint_after(Duration::from_millis(REPAINT_INTERVAL_MS));
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let _ = self.storage.save(&self.progress);
        let _ = self.settings.save();
    }
}
