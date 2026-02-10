use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use super::book::Book;
use crate::pdf::renderer::PdfRenderer;

pub struct LibraryScanner;

impl LibraryScanner {
    pub fn scan_directory(path: &Path) -> Result<Vec<PathBuf>> {
        let mut pdf_files = Vec::new();

        if !path.exists() {
            return Ok(pdf_files);
        }

        Self::scan_recursive(path, &mut pdf_files)?;
        pdf_files.sort();

        Ok(pdf_files)
    }

    fn scan_recursive(path: &Path, pdf_files: &mut Vec<PathBuf>) -> Result<()> {
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.to_string_lossy().to_lowercase() == "pdf" {
                    pdf_files.push(path.to_path_buf());
                }
            }
            return Ok(());
        }

        if path.is_dir() {
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                Self::scan_recursive(&entry_path, pdf_files)?;
            }
        }

        Ok(())
    }

    pub fn compute_file_hash(path: &Path) -> Result<String> {
        let mut file = File::open(path).context("Failed to open file for hashing")?;

        // Read first 64KB + file size for quick hash
        let mut buffer = vec![0u8; 65536];
        let bytes_read = file.read(&mut buffer)?;
        buffer.truncate(bytes_read);

        let file_size = file.metadata()?.len();

        let mut hasher = Sha256::new();
        hasher.update(&buffer);
        hasher.update(file_size.to_le_bytes());

        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    pub fn scan_and_load_books(library_path: &Path, renderer: Option<&PdfRenderer>) -> Result<Vec<Book>> {
        let pdf_paths = Self::scan_directory(library_path)?;
        let mut books = Vec::new();

        for path in pdf_paths {
            match Self::load_book(&path, renderer) {
                Ok(book) => books.push(book),
                Err(e) => {
                    eprintln!("Failed to load book {:?}: {}", path, e);
                }
            }
        }

        Ok(books)
    }

    fn load_book(path: &Path, renderer: Option<&PdfRenderer>) -> Result<Book> {
        let file_hash = Self::compute_file_hash(path)?;

        let total_pages = if let Some(r) = renderer {
            let bytes = r.load_document(path)?;
            r.get_page_count(&bytes)?
        } else {
            0 // Will be loaded later when opening
        };

        Ok(Book::new(path.to_path_buf(), file_hash, total_pages))
    }
}
