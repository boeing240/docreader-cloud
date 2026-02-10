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
