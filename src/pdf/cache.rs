use egui::TextureHandle;
use lru::LruCache;
use std::num::NonZeroUsize;

#[derive(Hash, Eq, PartialEq, Clone)]
struct CacheKey {
    book_hash: String,
    page: u32,
    dpi: u32,
}

pub struct PageCache {
    cache: LruCache<CacheKey, TextureHandle>,
}

impl PageCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::MIN)),
        }
    }

    pub fn get(&mut self, book_hash: &str, page: u32, dpi: u32) -> Option<&TextureHandle> {
        let key = CacheKey {
            book_hash: book_hash.to_string(),
            page,
            dpi,
        };
        self.cache.get(&key)
    }

    pub fn insert(&mut self, book_hash: &str, page: u32, dpi: u32, texture: TextureHandle) {
        let key = CacheKey {
            book_hash: book_hash.to_string(),
            page,
            dpi,
        };
        self.cache.put(key, texture);
    }

    #[allow(dead_code)]
    pub fn clear_book(&mut self, book_hash: &str) {
        // LruCache doesn't support selective removal by predicate,
        // so we collect keys to remove first
        let keys_to_remove: Vec<_> = self
            .cache
            .iter()
            .filter(|(k, _)| k.book_hash == book_hash)
            .map(|(k, _)| k.clone())
            .collect();

        for key in keys_to_remove {
            self.cache.pop(&key);
        }
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}
