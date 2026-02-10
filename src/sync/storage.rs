use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::library::progress::ReadingProgress;

pub struct ProgressStorage {
    file_path: PathBuf,
}

impl ProgressStorage {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    pub fn load(&self) -> Result<ReadingProgress> {
        if !self.file_path.exists() {
            return Err(anyhow::anyhow!("Progress file does not exist"));
        }

        let content =
            fs::read_to_string(&self.file_path).context("Failed to read progress file")?;

        let progress: ReadingProgress =
            serde_json::from_str(&content).context("Failed to parse progress file")?;

        Ok(progress)
    }

    pub fn load_or_create(&self, device_id: &str) -> Result<ReadingProgress> {
        match self.load() {
            Ok(progress) => Ok(progress),
            Err(_) => {
                let progress = ReadingProgress::new(device_id.to_string());
                self.save(&progress)?;
                Ok(progress)
            }
        }
    }

    pub fn save(&self, progress: &ReadingProgress) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).context("Failed to create progress directory")?;
        }

        // Write to temporary file first (atomic write)
        let temp_path = self.file_path.with_extension("json.tmp");

        let json =
            serde_json::to_string_pretty(progress).context("Failed to serialize progress")?;

        fs::write(&temp_path, &json).context("Failed to write temporary file")?;

        // Atomic rename
        fs::rename(&temp_path, &self.file_path).context("Failed to rename temporary file")?;

        Ok(())
    }

    #[allow(dead_code)]
    pub fn file_path(&self) -> &Path {
        &self.file_path
    }

    #[allow(dead_code)]
    pub fn exists(&self) -> bool {
        self.file_path.exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_storage(name: &str) -> (ProgressStorage, PathBuf) {
        let dir = std::env::temp_dir().join(format!("docreader_test_storage_{}", name));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("progress.json");
        (ProgressStorage::new(path.clone()), dir)
    }

    #[test]
    fn test_save_and_load() {
        let (storage, dir) = temp_storage("save_load");

        let mut progress = ReadingProgress::new("device1".to_string());
        progress.add_book(
            "h1".to_string(),
            "A.pdf".to_string(),
            "/a.pdf".to_string(),
            100,
        );

        storage.save(&progress).unwrap();
        assert!(storage.exists());

        let loaded = storage.load().unwrap();
        assert_eq!(loaded.device_id, "device1");
        assert_eq!(loaded.books.len(), 1);
        assert_eq!(loaded.books.get("h1").unwrap().file_name, "A.pdf");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_nonexistent() {
        let dir = std::env::temp_dir().join("docreader_test_storage_nofile");
        let _ = fs::remove_dir_all(&dir);
        let storage = ProgressStorage::new(dir.join("nope.json"));

        assert!(storage.load().is_err());
        assert!(!storage.exists());
    }

    #[test]
    fn test_load_or_create() {
        let (storage, dir) = temp_storage("load_or_create");

        let progress = storage.load_or_create("device2").unwrap();
        assert_eq!(progress.device_id, "device2");
        assert!(progress.books.is_empty());
        // File should now exist
        assert!(storage.exists());

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_save_creates_parent_dirs() {
        let dir = std::env::temp_dir().join("docreader_test_storage_nested");
        let _ = fs::remove_dir_all(&dir);
        let path = dir.join("a").join("b").join("progress.json");
        let storage = ProgressStorage::new(path);

        let progress = ReadingProgress::new("d1".to_string());
        storage.save(&progress).unwrap();
        assert!(storage.exists());

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_save_overwrites() {
        let (storage, dir) = temp_storage("overwrite");

        let mut p1 = ReadingProgress::new("d1".to_string());
        p1.add_book("h1".to_string(), "A.pdf".to_string(), "/a".to_string(), 10);
        storage.save(&p1).unwrap();

        let mut p2 = ReadingProgress::new("d1".to_string());
        p2.add_book("h2".to_string(), "B.pdf".to_string(), "/b".to_string(), 20);
        storage.save(&p2).unwrap();

        let loaded = storage.load().unwrap();
        assert_eq!(loaded.books.len(), 1);
        assert!(loaded.books.contains_key("h2"));

        let _ = fs::remove_dir_all(&dir);
    }
}
