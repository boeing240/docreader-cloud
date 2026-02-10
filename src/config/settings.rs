use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub library_path: PathBuf,
    pub progress_file_path: PathBuf,
    pub device_id: String,
    pub zoom_level: f32,
    pub auto_save_interval_secs: u64,
    #[serde(default)]
    pub last_opened_book: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        let device_id = Uuid::new_v4().to_string();

        // Default paths - user should configure these
        let home = directories::UserDirs::new()
            .map(|d| d.home_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));

        Self {
            library_path: home.join("YandexDisk").join("Books"),
            progress_file_path: home
                .join("YandexDisk")
                .join("Books")
                .join("reading_progress.json"),
            device_id,
            zoom_level: 1.0,
            auto_save_interval_secs: 5,
            last_opened_book: None,
        }
    }
}

impl AppSettings {
    pub fn config_path() -> Result<PathBuf> {
        let proj_dirs = directories::ProjectDirs::from("", "", "docreader-cloud")
            .context("Failed to get project directories")?;

        Ok(proj_dirs.config_dir().join("settings.json"))
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&config_path).context("Failed to read settings file")?;

        let settings: AppSettings =
            serde_json::from_str(&content).context("Failed to parse settings file")?;

        Ok(settings)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        let json = serde_json::to_string_pretty(self).context("Failed to serialize settings")?;

        fs::write(&config_path, json).context("Failed to write settings file")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = AppSettings::default();
        assert!(!settings.device_id.is_empty());
        assert_eq!(settings.zoom_level, 1.0);
        assert_eq!(settings.auto_save_interval_secs, 5);
        assert!(settings.last_opened_book.is_none());
    }

    #[test]
    fn test_default_unique_device_id() {
        let s1 = AppSettings::default();
        let s2 = AppSettings::default();
        assert_ne!(s1.device_id, s2.device_id);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut settings = AppSettings::default();
        settings.zoom_level = 1.5;
        settings.auto_save_interval_secs = 10;
        settings.last_opened_book = Some("abc".to_string());

        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: AppSettings = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.device_id, settings.device_id);
        assert_eq!(deserialized.zoom_level, 1.5);
        assert_eq!(deserialized.auto_save_interval_secs, 10);
        assert_eq!(deserialized.last_opened_book, Some("abc".to_string()));
    }

    #[test]
    fn test_config_path_not_empty() {
        let path = AppSettings::config_path();
        assert!(path.is_ok());
        let path = path.unwrap();
        assert!(path.to_string_lossy().contains("docreader-cloud"));
    }

    #[test]
    fn test_deserialization_missing_last_opened_book() {
        // last_opened_book has #[serde(default)], so it should handle missing field
        let json = r#"{
            "library_path": "C:\\Books",
            "progress_file_path": "C:\\Books\\progress.json",
            "device_id": "test-device",
            "zoom_level": 1.0,
            "auto_save_interval_secs": 5
        }"#;
        let settings: AppSettings = serde_json::from_str(json).unwrap();
        assert!(settings.last_opened_book.is_none());
        assert_eq!(settings.device_id, "test-device");
    }
}
