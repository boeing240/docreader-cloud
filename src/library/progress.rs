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
        };
        self.books.insert(book_hash, book_progress);
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
