# DocReader Cloud

[Русская версия](README.ru.md)

A desktop application for reading documents with cloud synchronization of reading progress across devices.

## Features

- **Multiple Format Support**: PDF, EPUB, FB2, DJVU
- **Cloud Sync**: Synchronize reading progress across multiple devices
- **User-Friendly Interface**: Clean, intuitive document viewer
- **Cross-Device Reading**: Continue reading on any device from where you left off
- **Zoom Control**: Adjust document scale for comfortable reading
- **Library Management**: Browse and organize your document collection

## Supported Formats

| Format | Status | Rendering Engine |
|--------|--------|------------------|
| PDF | Full support | `pdfium-render` |
| EPUB | Text rendering | `rbook` + custom text renderer |
| FB2 | Text rendering | `quick-xml` + custom text renderer |
| DJVU | Stub (not implemented) | — |

## For Users

### System Requirements

- **Operating System**: Windows 10/11 (64-bit)
- **RAM**: 2 GB minimum
- **Disk Space**: 50 MB for application + space for your documents

### Installation

1. Download the latest release from the [Releases](https://github.com/yourusername/docreader-cloud/releases) page
2. Extract the archive to a folder of your choice
3. Run `docreader-cloud.exe`

### First Launch

On the first launch, the application will:
1. Create a configuration folder at `%APPDATA%\docreader-cloud\`
2. Set default paths:
   - Library: `%USERPROFILE%\Documents\Books\`
   - Progress file: `%USERPROFILE%\Documents\Books\reading_progress.json`

### Usage

#### Setting Up Your Library

1. Click the **Settings** button (gear icon) in the toolbar
2. Set your **Library Path** to the folder containing your documents
3. Set your **Progress File Path** to a shared folder (e.g., Dropbox, Google Drive, OneDrive, Yandex Disk) for cross-device sync
4. Click **Save** and then **Rescan Library**

#### Reading Documents

- Click on any book in the left sidebar to open it
- Use **arrow keys**, **Page Up/Down**, or **Home/End** to navigate
- Use the **zoom buttons** or toolbar controls to adjust scale
- Your progress is automatically saved every 5 seconds (configurable in settings)

#### Synchronizing Across Devices

1. Place your `reading_progress.json` file in a cloud-synchronized folder (Dropbox, Google Drive, etc.)
2. On each device, configure the **Progress File Path** in settings to point to this shared file
3. The application will automatically sync your reading position across devices

### Keyboard Shortcuts

- **Arrow Up/Down**: Navigate pages
- **Page Up/Down**: Navigate pages
- **Home**: Go to first page
- **End**: Go to last page
- **+/-**: Zoom in/out

## For Developers

### Building from Source

#### Prerequisites

- **Rust**: 1.70 or later ([Install Rust](https://rustup.rs/))
- **Windows**: MSVC toolchain (install via Visual Studio Build Tools)

#### Build Steps

1. Clone the repository:
```bash
git clone https://github.com/yourusername/docreader-cloud.git
cd docreader-cloud
```

2. Build the project:
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

3. Run the application:
```bash
# Run debug version
cargo run

# Run release version
cargo run --release
```

4. Run tests:
```bash
cargo test
```

The compiled executable will be located at:
- Debug: `target\debug\docreader-cloud.exe`
- Release: `target\release\docreader-cloud.exe`

### Project Structure

```
src/
  main.rs              — Entry point, eframe setup
  app/                 — Application core, UI layout, event handling
  config/              — Settings, constants
  library/             — Book management, scanning, progress tracking
  renderer/            — Document rendering (PDF, EPUB, FB2, DJVU)
  sync/                — Progress synchronization, file watching
  ui/                  — UI components (toolbar, sidebar, viewer)
libs/
  pdfium.dll           — Embedded PDFium library for PDF rendering
  fonts/               — Embedded fonts for text rendering
```

### Architecture

- **Render Thread**: Background thread processes render requests via `mpsc` channel
- **Document Renderers**: Format-specific renderers implement `DocumentRenderer` trait
- **LRU Cache**: 20-page cache for rendered images
- **Progress Sync**: Atomic file writes + file watcher for cross-device sync
- **Text Rendering**: Common text renderer for EPUB/FB2 with pagination (800x1100px virtual pages)

### Git Workflow

This project uses **GitHub Flow**:

1. Create a **feature branch** from `main`
2. Make your changes and commit
3. Open a Pull Request to `main`
4. After review, merge and delete the feature branch
5. Releases are created when ready and tagged as `vX.Y.Z` on `main`

**Note**: Releases are created periodically when significant features are ready or bug fixes accumulate, not necessarily for every merge to `main`.

Commit messages should be in Russian or English.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Acknowledgments

- [pdfium-render](https://crates.io/crates/pdfium-render) - PDF rendering
- [rbook](https://crates.io/crates/rbook) - EPUB parsing
- [egui](https://crates.io/crates/egui) - Immediate mode GUI framework
- [quick-xml](https://crates.io/crates/quick-xml) - XML parsing for FB2
