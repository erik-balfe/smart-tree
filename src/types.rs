use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    #[allow(dead_code)]
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub metadata: EntryMetadata,
    pub children: Vec<DirectoryEntry>,
    pub is_gitignored: bool,
}

#[derive(Debug, Clone)]
pub struct EntryMetadata {
    pub size: u64,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub files_count: usize,
}

#[derive(Debug, Clone)]
pub struct DisplayConfig {
    pub max_lines: usize,
    pub dir_limit: usize,
    pub sort_by: SortBy,
    pub dirs_first: bool,
    pub use_colors: bool,
    pub color_theme: ColorTheme,
    pub use_emoji: bool,           // Whether to use emoji icons
    pub size_colorize: bool,       // Whether to colorize sizes by value
    pub date_colorize: bool,       // Whether to colorize dates by recency
    pub detailed_metadata: bool,   // Whether to show detailed metadata
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColorTheme {
    Auto,
    Light,
    Dark,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortBy {
    Name,
    Size,
    Modified,
    Created,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Directory,
    Symlink,
    Regular,
    Image,
    Video,
    Audio,
    Archive,
    Code,
    Document,
    Executable,
    Hidden,
}
