use crate::library::progress::ReadingProgress;

#[cfg(test)]
use crate::library::progress::BookProgress;

pub struct ProgressMerger;

impl ProgressMerger {
    /// Merge remote progress into local, using "last read wins" strategy
    pub fn merge(local: &ReadingProgress, remote: &ReadingProgress) -> ReadingProgress {
        let mut merged = local.clone();

        for (book_hash, remote_book) in &remote.books {
            match merged.books.get(book_hash) {
                Some(local_book) => {
                    // Book exists in both - take the one with most recent last_read
                    if remote_book.last_read > local_book.last_read {
                        merged.books.insert(book_hash.clone(), remote_book.clone());
                    }
                }
                None => {
                    // Book only exists in remote - add it
                    merged.books.insert(book_hash.clone(), remote_book.clone());
                }
            }
        }

        // Update last_modified to now
        merged.last_modified = chrono::Utc::now();

        merged
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_merge_takes_newer() {
        let now = Utc::now();
        let earlier = now - Duration::hours(1);

        let mut local = ReadingProgress::new("device1".to_string());
        local.books.insert(
            "book1".to_string(),
            BookProgress {
                file_name: "Book.pdf".to_string(),
                file_path: "Book.pdf".to_string(),
                file_hash: "book1".to_string(),
                total_pages: 100,
                current_page: 50,
                last_read: earlier,
            },
        );

        let mut remote = ReadingProgress::new("device2".to_string());
        remote.books.insert(
            "book1".to_string(),
            BookProgress {
                file_name: "Book.pdf".to_string(),
                file_path: "Book.pdf".to_string(),
                file_hash: "book1".to_string(),
                total_pages: 100,
                current_page: 75,
                last_read: now,
            },
        );

        let merged = ProgressMerger::merge(&local, &remote);
        assert_eq!(merged.books.get("book1").unwrap().current_page, 75);
    }

    #[test]
    fn test_merge_adds_new_books() {
        let local = ReadingProgress::new("device1".to_string());
        let mut remote = ReadingProgress::new("device2".to_string());

        remote.books.insert(
            "book2".to_string(),
            BookProgress {
                file_name: "NewBook.pdf".to_string(),
                file_path: "NewBook.pdf".to_string(),
                file_hash: "book2".to_string(),
                total_pages: 200,
                current_page: 10,
                last_read: Utc::now(),
            },
        );

        let merged = ProgressMerger::merge(&local, &remote);
        assert!(merged.books.contains_key("book2"));
    }
}
