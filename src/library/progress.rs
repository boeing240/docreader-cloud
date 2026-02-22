use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingProgress {
    pub version: u32,
    pub last_modified: DateTime<Utc>,
    pub device_id: String,
    pub books: HashMap<String, BookProgress>,
}

impl ReadingProgress {
    pub fn new(device_id: String) -> Self {
        Self {
            version: 1,
            last_modified: Utc::now(),
            device_id,
            books: HashMap::new(),
        }
    }

    pub fn update_book_progress(&mut self, book_hash: &str, current_page: u32) {
        if let Some(bp) = self.books.get_mut(book_hash) {
            bp.current_page = current_page;
            bp.last_read = Utc::now();
        }
        self.last_modified = Utc::now();
    }

    pub fn add_book(
        &mut self,
        book_hash: String,
        file_name: String,
        file_path: String,
        total_pages: u32,
    ) {
        let book_progress = BookProgress {
            file_name,
            file_path,
            file_hash: book_hash.clone(),
            total_pages,
            current_page: 1,
            last_read: Utc::now(),
            scroll_offset: (0.0, 0.0),
            zoom: None,
        };
        self.books.insert(book_hash, book_progress);
        self.last_modified = Utc::now();
    }

    pub fn update_scroll_offset(&mut self, book_hash: &str, offset: (f32, f32)) {
        if let Some(bp) = self.books.get_mut(book_hash) {
            bp.scroll_offset = offset;
        }
        self.last_modified = Utc::now();
    }

    pub fn update_zoom(&mut self, book_hash: &str, zoom: f32) {
        if let Some(bp) = self.books.get_mut(book_hash) {
            bp.zoom = Some(zoom);
        }
        self.last_modified = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookProgress {
    pub file_name: String,
    pub file_path: String,
    pub file_hash: String,
    pub total_pages: u32,
    pub current_page: u32,
    pub last_read: DateTime<Utc>,
    #[serde(default)]
    pub scroll_offset: (f32, f32),
    #[serde(default)]
    pub zoom: Option<f32>,
}

#[allow(dead_code)]
impl BookProgress {
    pub fn progress_percent(&self) -> f32 {
        if self.total_pages == 0 {
            0.0
        } else {
            (self.current_page as f32 / self.total_pages as f32) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_book_progress(current_page: u32, total_pages: u32) -> BookProgress {
        BookProgress {
            file_name: "test.pdf".to_string(),
            file_path: "/books/test.pdf".to_string(),
            file_hash: "abc123".to_string(),
            total_pages,
            current_page,
            last_read: Utc::now(),
            scroll_offset: (0.0, 0.0),
            zoom: None,
        }
    }

    #[test]
    fn test_progress_percent_normal() {
        let bp = make_book_progress(50, 100);
        assert!((bp.progress_percent() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_progress_percent_zero_pages() {
        let bp = make_book_progress(0, 0);
        assert!((bp.progress_percent() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_progress_percent_first_page() {
        let bp = make_book_progress(1, 200);
        assert!((bp.progress_percent() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_progress_percent_last_page() {
        let bp = make_book_progress(100, 100);
        assert!((bp.progress_percent() - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_new_reading_progress() {
        let rp = ReadingProgress::new("device1".to_string());
        assert_eq!(rp.version, 1);
        assert_eq!(rp.device_id, "device1");
        assert!(rp.books.is_empty());
    }

    #[test]
    fn test_add_book() {
        let mut rp = ReadingProgress::new("device1".to_string());
        rp.add_book(
            "hash1".to_string(),
            "Book.pdf".to_string(),
            "/books/Book.pdf".to_string(),
            100,
        );

        assert_eq!(rp.books.len(), 1);
        let bp = rp.books.get("hash1").unwrap();
        assert_eq!(bp.file_name, "Book.pdf");
        assert_eq!(bp.current_page, 1);
        assert_eq!(bp.total_pages, 100);
    }

    #[test]
    fn test_update_book_progress() {
        let mut rp = ReadingProgress::new("device1".to_string());
        rp.add_book(
            "hash1".to_string(),
            "Book.pdf".to_string(),
            "/books/Book.pdf".to_string(),
            100,
        );

        let before = rp.books.get("hash1").unwrap().last_read;
        std::thread::sleep(std::time::Duration::from_millis(10));

        rp.update_book_progress("hash1", 42);

        let bp = rp.books.get("hash1").unwrap();
        assert_eq!(bp.current_page, 42);
        assert!(bp.last_read >= before);
    }

    #[test]
    fn test_update_nonexistent_book() {
        let mut rp = ReadingProgress::new("device1".to_string());
        // Should not panic
        rp.update_book_progress("nonexistent", 10);
        assert!(rp.books.is_empty());
    }

    #[test]
    fn test_add_multiple_books() {
        let mut rp = ReadingProgress::new("device1".to_string());
        rp.add_book(
            "h1".to_string(),
            "A.pdf".to_string(),
            "/a.pdf".to_string(),
            50,
        );
        rp.add_book(
            "h2".to_string(),
            "B.epub".to_string(),
            "/b.epub".to_string(),
            200,
        );

        assert_eq!(rp.books.len(), 2);
        assert!(rp.books.contains_key("h1"));
        assert!(rp.books.contains_key("h2"));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut rp = ReadingProgress::new("device1".to_string());
        rp.add_book(
            "h1".to_string(),
            "A.pdf".to_string(),
            "/a.pdf".to_string(),
            50,
        );
        rp.update_book_progress("h1", 25);

        let json = serde_json::to_string(&rp).unwrap();
        let deserialized: ReadingProgress = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.device_id, "device1");
        assert_eq!(deserialized.books.len(), 1);
        assert_eq!(deserialized.books.get("h1").unwrap().current_page, 25);
    }
}
