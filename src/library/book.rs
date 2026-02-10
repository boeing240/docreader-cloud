use std::path::PathBuf;

use crate::renderer::format::DocumentFormat;

#[derive(Debug, Clone)]
pub struct Book {
    pub file_path: PathBuf,
    pub file_name: String,
    pub file_hash: String,
    pub total_pages: u32,
    pub format: DocumentFormat,
}

impl Book {
    pub fn new(
        file_path: PathBuf,
        file_hash: String,
        total_pages: u32,
        format: DocumentFormat,
    ) -> Self {
        let file_name = file_path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        Self {
            file_path,
            file_name,
            file_hash,
            total_pages,
            format,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_extracts_filename() {
        let book = Book::new(
            PathBuf::from("/home/user/books/Novel.pdf"),
            "hash123".to_string(),
            100,
            DocumentFormat::Pdf,
        );
        assert_eq!(book.file_name, "Novel.pdf");
        assert_eq!(book.total_pages, 100);
        assert_eq!(book.format, DocumentFormat::Pdf);
    }

    #[test]
    fn test_new_no_filename() {
        let book = Book::new(
            PathBuf::from("/"),
            "hash".to_string(),
            0,
            DocumentFormat::Epub,
        );
        assert_eq!(book.file_name, "Unknown");
    }

    #[test]
    fn test_new_windows_path() {
        let book = Book::new(
            PathBuf::from(r"C:\Books\Test.fb2"),
            "hash456".to_string(),
            50,
            DocumentFormat::Fb2,
        );
        assert_eq!(book.file_name, "Test.fb2");
    }
}
