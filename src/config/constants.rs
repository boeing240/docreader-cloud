// ── Окно приложения ──────────────────────────────────────────────
pub const WINDOW_TITLE: &str = "DocReader";
pub const WINDOW_INITIAL_WIDTH: f32 = 1200.0;
pub const WINDOW_INITIAL_HEIGHT: f32 = 800.0;
pub const WINDOW_MIN_WIDTH: f32 = 800.0;
pub const WINDOW_MIN_HEIGHT: f32 = 600.0;

// ── Боковая панель ──────────────────────────────────────────────
pub const SIDEBAR_DEFAULT_WIDTH: f32 = 250.0;
pub const SIDEBAR_MIN_WIDTH: f32 = 150.0;
pub const SIDEBAR_ITEM_SPACING: f32 = 8.0;
pub const SIDEBAR_PROGRESS_BAR_WIDTH: f32 = 80.0;

// ── Зум ─────────────────────────────────────────────────────────
pub const ZOOM_DEFAULT: f32 = 1.0;
pub const ZOOM_MIN: f32 = 0.3;
pub const ZOOM_MAX: f32 = 3.0;
pub const ZOOM_STEP: f32 = 0.1;

// ── PDF рендеринг ───────────────────────────────────────────────
pub const PDF_SCALE_MULTIPLIER: f32 = 1.5;

// ── Текстовый рендеринг (EPUB / FB2) ───────────────────────────
pub const TEXT_PAGE_WIDTH: u32 = 800;
pub const TEXT_PAGE_HEIGHT: u32 = 1100;
pub const TEXT_PAGE_MARGIN: u32 = 40;
pub const TEXT_FONT_SIZE: f32 = 20.0;
pub const TEXT_LINE_HEIGHT: f32 = 28.0;
pub const TEXT_PARAGRAPH_SPACING: f32 = 14.0;

// ── Кэш ─────────────────────────────────────────────────────────
pub const PAGE_CACHE_CAPACITY: usize = 20;
pub const SCALE_COMPARE_EPSILON: f32 = 0.01;
pub const HASH_BUFFER_SIZE: usize = 256;

// ── Тайминги ────────────────────────────────────────────────────
pub const REPAINT_INTERVAL_MS: u64 = 100;
pub const FILE_WATCHER_POLL_SECS: u64 = 2;
pub const AUTO_SAVE_INTERVAL_SECS_DEFAULT: u64 = 5;

// ── HiDPI ───────────────────────────────────────────────────────
pub const HIDPI_CHANGE_THRESHOLD: f32 = 0.01;

// ── Статусбар ───────────────────────────────────────────────────
pub const DEVICE_ID_DISPLAY_LEN: usize = 8;

// ── Временные файлы ─────────────────────────────────────────────
pub const TEMP_DIR_NAME: &str = "docreader-cloud";
pub const TEMP_EPUB_FILENAME: &str = "_temp_epub.epub";
pub const PDFIUM_DLL_FILENAME: &str = "pdfium.dll";

// ── Файлы конфигурации и прогресса ──────────────────────────────
pub const PROJECT_NAME: &str = "docreader-cloud";
pub const SETTINGS_FILENAME: &str = "settings.json";
pub const PROGRESS_FILENAME: &str = "reading_progress.json";
pub const DEFAULT_CLOUD_DIR: &str = "YandexDisk";
pub const DEFAULT_BOOKS_DIR: &str = "Books";
