use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct GitIgnore {
    patterns: Vec<String>,
}

impl GitIgnore {
    pub fn load(root: &Path) -> Result<Self> {
        let mut patterns = vec![
            // System defaults that should always be included
            ".git".to_string(),
            ".DS_Store".to_string(),
        ];

        // Add patterns from .gitignore if it exists
        let gitignore_path = root.join(".gitignore");
        if gitignore_path.exists() {
            patterns.extend(
                fs::read_to_string(gitignore_path)?
                    .lines()
                    .filter(|line| {
                        // Filter out empty lines and comments
                        !line.trim().is_empty() && !line.starts_with('#')
                    })
                    .map(String::from),
            );
        }

        Ok(GitIgnore { patterns })
    }

    pub fn is_ignored(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        self.patterns.iter().any(|pattern| {
            // Simple pattern matching for MVP
            // We'll improve this later with proper glob matching
            path_str.contains(pattern)
        })
    }
}
