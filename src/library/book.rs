use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Book {
    pub file_path: PathBuf,
    pub file_name: String,
    pub file_hash: String,
    pub total_pages: u32,
}

impl Book {
    pub fn new(file_path: PathBuf, file_hash: String, total_pages: u32) -> Self {
        let file_name = file_path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        Self {
            file_path,
            file_name,
            file_hash,
            total_pages,
        }
    }
}
