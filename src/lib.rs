//! Smart tree display library

mod display;
mod gitignore;
mod log_macros;
mod scanner;
mod types;

// Re-export public items
pub use display::format_tree;
pub use gitignore::GitIgnore;
pub use scanner::scan_directory;
pub use types::{DirectoryEntry, DisplayConfig, EntryMetadata, SortBy};
