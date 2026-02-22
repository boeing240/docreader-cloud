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
                scroll_offset: (0.0, 0.0),
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
                scroll_offset: (0.0, 0.0),
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
                scroll_offset: (0.0, 0.0),
            },
        );

        let merged = ProgressMerger::merge(&local, &remote);
        assert!(merged.books.contains_key("book2"));
    }

    #[test]
    fn test_merge_keeps_local_when_newer() {
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
                current_page: 80,
                last_read: now,
                scroll_offset: (0.0, 0.0),
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
                current_page: 30,
                last_read: earlier,
                scroll_offset: (0.0, 0.0),
            },
        );

        let merged = ProgressMerger::merge(&local, &remote);
        assert_eq!(merged.books.get("book1").unwrap().current_page, 80);
    }

    #[test]
    fn test_merge_preserves_local_only_books() {
        let mut local = ReadingProgress::new("device1".to_string());
        local.books.insert(
            "local_book".to_string(),
            BookProgress {
                file_name: "Local.pdf".to_string(),
                file_path: "Local.pdf".to_string(),
                file_hash: "local_book".to_string(),
                total_pages: 50,
                current_page: 25,
                last_read: Utc::now(),
                scroll_offset: (0.0, 0.0),
            },
        );

        let remote = ReadingProgress::new("device2".to_string());

        let merged = ProgressMerger::merge(&local, &remote);
        assert!(merged.books.contains_key("local_book"));
        assert_eq!(merged.books.get("local_book").unwrap().current_page, 25);
    }

    #[test]
    fn test_merge_both_empty() {
        let local = ReadingProgress::new("device1".to_string());
        let remote = ReadingProgress::new("device2".to_string());

        let merged = ProgressMerger::merge(&local, &remote);
        assert!(merged.books.is_empty());
        assert_eq!(merged.device_id, "device1");
    }

    #[test]
    fn test_merge_multiple_books() {
        let now = Utc::now();
        let earlier = now - Duration::hours(1);

        let mut local = ReadingProgress::new("device1".to_string());
        local.books.insert(
            "book1".to_string(),
            BookProgress {
                file_name: "A.pdf".to_string(),
                file_path: "A.pdf".to_string(),
                file_hash: "book1".to_string(),
                total_pages: 100,
                current_page: 10,
                last_read: earlier,
                scroll_offset: (0.0, 0.0),
            },
        );
        local.books.insert(
            "book2".to_string(),
            BookProgress {
                file_name: "B.pdf".to_string(),
                file_path: "B.pdf".to_string(),
                file_hash: "book2".to_string(),
                total_pages: 200,
                current_page: 100,
                last_read: now,
                scroll_offset: (0.0, 0.0),
            },
        );

        let mut remote = ReadingProgress::new("device2".to_string());
        remote.books.insert(
            "book1".to_string(),
            BookProgress {
                file_name: "A.pdf".to_string(),
                file_path: "A.pdf".to_string(),
                file_hash: "book1".to_string(),
                total_pages: 100,
                current_page: 50,
                last_read: now,
                scroll_offset: (0.0, 0.0),
            },
        );
        remote.books.insert(
            "book3".to_string(),
            BookProgress {
                file_name: "C.pdf".to_string(),
                file_path: "C.pdf".to_string(),
                file_hash: "book3".to_string(),
                total_pages: 300,
                current_page: 5,
                last_read: now,
                scroll_offset: (0.0, 0.0),
            },
        );

        let merged = ProgressMerger::merge(&local, &remote);
        assert_eq!(merged.books.len(), 3);
        // book1: remote is newer
        assert_eq!(merged.books.get("book1").unwrap().current_page, 50);
        // book2: local only, preserved
        assert_eq!(merged.books.get("book2").unwrap().current_page, 100);
        // book3: remote only, added
        assert_eq!(merged.books.get("book3").unwrap().current_page, 5);
    }
}
