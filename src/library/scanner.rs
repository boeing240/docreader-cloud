use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use super::book::Book;
use crate::renderer::format::DocumentFormat;
use crate::renderer::RendererRegistry;

pub struct LibraryScanner;

impl LibraryScanner {
    pub fn scan_directory(path: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        if !path.exists() {
            return Ok(files);
        }

        Self::scan_recursive(path, &mut files)?;
        files.sort();

        Ok(files)
    }

    fn scan_recursive(path: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        if path.is_file() {
            if DocumentFormat::from_path(path).is_some() {
                files.push(path.to_path_buf());
            }
            return Ok(());
        }

        if path.is_dir() {
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                Self::scan_recursive(&entry_path, files)?;
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

    pub fn scan_and_load_books(
        library_path: &Path,
        registry: Option<&RendererRegistry>,
    ) -> Result<Vec<Book>> {
        let paths = Self::scan_directory(library_path)?;
        let mut books = Vec::new();

        for path in paths {
            match Self::load_book(&path, registry) {
                Ok(book) => books.push(book),
                Err(e) => {
                    eprintln!("Failed to load book {:?}: {}", path, e);
                }
            }
        }

        Ok(books)
    }

    fn load_book(path: &Path, registry: Option<&RendererRegistry>) -> Result<Book> {
        let format = DocumentFormat::from_path(path).context("Unsupported file format")?;
        let file_hash = Self::compute_file_hash(path)?;

        let total_pages = if let Some(reg) = registry {
            if let Some(renderer) = reg.get(&format) {
                let bytes = std::fs::read(path)?;
                renderer.get_page_count(&bytes).unwrap_or(0)
            } else {
                0
            }
        } else {
            0 // Will be loaded later when opening
        };

        Ok(Book::new(
            path.to_path_buf(),
            file_hash,
            total_pages,
            format,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_scan_nonexistent_directory() {
        let result = LibraryScanner::scan_directory(Path::new("/nonexistent_dir_12345"));
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_scan_directory_filters_by_format() {
        let dir = std::env::temp_dir().join("docreader_test_scan");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        // Create supported files
        fs::write(dir.join("book.pdf"), b"fake pdf").unwrap();
        fs::write(dir.join("book.epub"), b"fake epub").unwrap();
        // Create unsupported file
        fs::write(dir.join("notes.txt"), b"text").unwrap();

        let files = LibraryScanner::scan_directory(&dir).unwrap();
        assert_eq!(files.len(), 2);

        let names: Vec<String> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();
        assert!(names.contains(&"book.epub".to_string()));
        assert!(names.contains(&"book.pdf".to_string()));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_scan_directory_recursive() {
        let dir = std::env::temp_dir().join("docreader_test_recursive");
        let _ = fs::remove_dir_all(&dir);
        let subdir = dir.join("subdir");
        fs::create_dir_all(&subdir).unwrap();

        fs::write(dir.join("root.pdf"), b"pdf").unwrap();
        fs::write(subdir.join("nested.fb2"), b"fb2").unwrap();

        let files = LibraryScanner::scan_directory(&dir).unwrap();
        assert_eq!(files.len(), 2);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_compute_file_hash_deterministic() {
        let dir = std::env::temp_dir().join("docreader_test_hash");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        let file = dir.join("test.bin");
        fs::write(&file, b"hello world").unwrap();

        let hash1 = LibraryScanner::compute_file_hash(&file).unwrap();
        let hash2 = LibraryScanner::compute_file_hash(&file).unwrap();
        assert_eq!(hash1, hash2);
        assert!(!hash1.is_empty());

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_compute_file_hash_different_content() {
        let dir = std::env::temp_dir().join("docreader_test_hash_diff");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        let file1 = dir.join("a.bin");
        let file2 = dir.join("b.bin");
        fs::write(&file1, b"content A").unwrap();
        fs::write(&file2, b"content B").unwrap();

        let hash1 = LibraryScanner::compute_file_hash(&file1).unwrap();
        let hash2 = LibraryScanner::compute_file_hash(&file2).unwrap();
        assert_ne!(hash1, hash2);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_scan_and_load_books() {
        let dir = std::env::temp_dir().join("docreader_test_load");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        fs::write(dir.join("book.pdf"), b"fake pdf content").unwrap();
        fs::write(dir.join("book.epub"), b"fake epub content").unwrap();

        let books = LibraryScanner::scan_and_load_books(&dir, None).unwrap();
        assert_eq!(books.len(), 2);

        // Without renderer, total_pages should be 0
        for book in &books {
            assert_eq!(book.total_pages, 0);
            assert!(!book.file_hash.is_empty());
        }

        let _ = fs::remove_dir_all(&dir);
    }
}
