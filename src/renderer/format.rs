use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DocumentFormat {
    Pdf,
    Epub,
    Fb2,
    Djvu,
}

impl DocumentFormat {
    pub fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?.to_lowercase();
        match ext.as_str() {
            "pdf" => Some(Self::Pdf),
            "epub" => Some(Self::Epub),
            "fb2" => Some(Self::Fb2),
            "djvu" | "djv" => Some(Self::Djvu),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Pdf => "PDF",
            Self::Epub => "EPUB",
            Self::Fb2 => "FB2",
            Self::Djvu => "DJVU",
        }
    }

    pub fn supported_extensions() -> &'static [&'static str] {
        &["pdf", "epub", "fb2", "djvu", "djv"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_from_path_pdf() {
        assert_eq!(
            DocumentFormat::from_path(Path::new("book.pdf")),
            Some(DocumentFormat::Pdf)
        );
    }

    #[test]
    fn test_from_path_epub() {
        assert_eq!(
            DocumentFormat::from_path(Path::new("book.epub")),
            Some(DocumentFormat::Epub)
        );
    }

    #[test]
    fn test_from_path_fb2() {
        assert_eq!(
            DocumentFormat::from_path(Path::new("book.fb2")),
            Some(DocumentFormat::Fb2)
        );
    }

    #[test]
    fn test_from_path_djvu() {
        assert_eq!(
            DocumentFormat::from_path(Path::new("book.djvu")),
            Some(DocumentFormat::Djvu)
        );
        assert_eq!(
            DocumentFormat::from_path(Path::new("book.djv")),
            Some(DocumentFormat::Djvu)
        );
    }

    #[test]
    fn test_from_path_case_insensitive() {
        assert_eq!(
            DocumentFormat::from_path(Path::new("book.PDF")),
            Some(DocumentFormat::Pdf)
        );
        assert_eq!(
            DocumentFormat::from_path(Path::new("book.Epub")),
            Some(DocumentFormat::Epub)
        );
    }

    #[test]
    fn test_from_path_unknown() {
        assert_eq!(DocumentFormat::from_path(Path::new("book.txt")), None);
        assert_eq!(DocumentFormat::from_path(Path::new("book.doc")), None);
        assert_eq!(DocumentFormat::from_path(Path::new("noext")), None);
    }

    #[test]
    fn test_from_path_with_directory() {
        assert_eq!(
            DocumentFormat::from_path(Path::new("/home/user/books/novel.epub")),
            Some(DocumentFormat::Epub)
        );
    }

    #[test]
    fn test_display_name() {
        assert_eq!(DocumentFormat::Pdf.display_name(), "PDF");
        assert_eq!(DocumentFormat::Epub.display_name(), "EPUB");
        assert_eq!(DocumentFormat::Fb2.display_name(), "FB2");
        assert_eq!(DocumentFormat::Djvu.display_name(), "DJVU");
    }

    #[test]
    fn test_supported_extensions() {
        let exts = DocumentFormat::supported_extensions();
        assert!(exts.contains(&"pdf"));
        assert!(exts.contains(&"epub"));
        assert!(exts.contains(&"fb2"));
        assert!(exts.contains(&"djvu"));
        assert!(exts.contains(&"djv"));
    }
}
