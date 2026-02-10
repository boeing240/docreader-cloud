# DocReader Cloud

Десктопное приложение для чтения документов с облачной синхронизацией прогресса между устройствами.

## Сборка и запуск

```bash
cargo build           # debug сборка
cargo build --release # release сборка
cargo run             # запуск
cargo test            # тесты
cargo check           # быстрая проверка компиляции
```

## Поддерживаемые форматы

| Формат | Крейт | Статус |
|--------|-------|--------|
| PDF | `pdfium-render` + встроенный `pdfium.dll` | Полная поддержка |
| EPUB | `rbook` + `ab_glyph`/`imageproc` | Текстовый рендеринг |
| FB2 | `quick-xml` + `ab_glyph`/`imageproc` | Текстовый рендеринг |
| DJVU | — | Заглушка, не реализован |

## Архитектура

```
src/
  main.rs              — точка входа, настройка eframe
  app/
    mod.rs             — DocReaderApp struct, eframe::App impl, UI layout
    render_thread.rs   — типы RenderRequest/Result/Response, запуск render thread
    render_manager.rs  — request_render, poll_render_results (кэш, текстуры)
    book_manager.rs    — select_book, go_to_page, handle_toolbar_action, rescan_library
    progress_manager.rs — check_sync, maybe_save_progress
    input_handler.rs   — обработка клавиатуры (стрелки, PageUp/Down, Home/End)
    settings_dialog.rs — окно настроек
  config/
    settings.rs        — настройки приложения (пути, device_id, zoom)
  library/
    book.rs            — структура Book (path, hash, format, pages)
    scanner.rs         — сканирование директории, поиск файлов всех форматов
    progress.rs        — ReadingProgress, BookProgress (отслеживание прогресса)
  renderer/
    mod.rs             — RendererRegistry (диспетчер рендереров по формату)
    traits.rs          — трейт DocumentRenderer
    format.rs          — enum DocumentFormat (Pdf, Epub, Fb2, Djvu)
    pdf.rs             — PdfRenderer (pdfium-render)
    epub.rs            — EpubRenderer (rbook + текстовый рендеринг)
    fb2.rs             — Fb2Renderer (quick-xml + текстовый рендеринг)
    djvu.rs            — DjvuRenderer (заглушка)
    text_render.rs     — TextPageRenderer (пагинация текста, word-wrap, рендеринг в RgbaImage)
    cache.rs           — LRU кэш отрендеренных страниц
  sync/
    storage.rs         — сохранение/загрузка прогресса в JSON
    watcher.rs         — наблюдение за изменениями файла прогресса
    merge.rs           — слияние прогресса между устройствами (last-read-wins)
  ui/
    toolbar.rs         — панель навигации и зума
    sidebar.rs         — боковая панель библиотеки
    document_viewer.rs — отображение отрендеренной страницы
libs/
  pdfium.dll           — встроенная библиотека PDFium
  fonts/
    NotoSans-Regular.ttf — встроенный шрифт для текстовых форматов
```

## Ключевые паттерны

- **Render thread**: фоновый поток обрабатывает RenderRequest через mpsc канал, возвращает RgbaImage
- **Модули app/**: свободные функции `fn xxx(app: &mut DocReaderApp)` — разделение по ответственности без лишних абстракций
- **RendererRegistry**: создаётся внутри render thread, диспетчеризует по DocumentFormat
- **TextPageRenderer**: общий рендеринг текста для EPUB/FB2 — пагинация в виртуальные страницы 800x1100px
- **Atomic writes**: прогресс сохраняется через tmp файл + rename
- **LRU кэш**: 20 страниц, инвалидируется при смене зума

## Соглашения

- Язык интерфейса: русский
- Платформа: Windows (`#![windows_subsystem = "windows"]`)
- GUI: egui/eframe (immediate mode)
- Ошибки: `anyhow::Result` для пропагации, `thiserror` для кастомных типов
- Все пути: `std::path::PathBuf`
