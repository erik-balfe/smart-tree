//! Smart tree display library

mod display;
mod gitignore;
mod log_macros;
mod scanner;
mod tests;
mod types;

// Re-export public items
pub use display::{format_tree, should_use_colors};
pub use gitignore::GitIgnore;
pub use scanner::scan_directory;
pub use types::{ColorTheme, DirectoryEntry, DisplayConfig, EntryMetadata, SortBy};

// Convenience wrapper for backward compatibility 
#[deprecated(since = "0.2.1", note = "Use scan_directory with show_system_dirs parameter instead")]
pub fn scan_directory_simple(
    root: &std::path::Path, 
    gitignore: &GitIgnore, 
    max_depth: usize
) -> anyhow::Result<DirectoryEntry> {
    scanner::scan_directory(root, gitignore, max_depth, None)
}
