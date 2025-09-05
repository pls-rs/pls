use crate::git_cache::GitStatusCache;
use std::path::Path;

/// Manages git status information with caching to avoid repeated git calls
pub struct GitMan {
    /// The git status cache
    cache: GitStatusCache,
}

impl Default for GitMan {
    fn default() -> Self {
        Self {
            cache: GitStatusCache::new(),
        }
    }
}

impl GitMan {
    /// Get the git status for a specific file path, using cached results when possible
    pub fn get_status(&mut self, file_path: &Path) -> Option<String> {
        self.cache.get_status(file_path)
    }

    /// Clear the git status cache (useful when repository state might have changed)
    #[allow(dead_code)]
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}