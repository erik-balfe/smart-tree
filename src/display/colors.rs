use crate::types::{ColorTheme, DirectoryEntry, DisplayConfig, FileType};
use colored::{Color, Colorize, ColoredString};

// Tree connectors with padding
pub const TREE_BRANCH: &str = "├── ";  // T-shape connector
pub const TREE_CORNER: &str = "└── ";  // L-shape corner connector
pub const TREE_VERTICAL: &str = "│   "; // Vertical line with spacing
pub const TREE_SPACE: &str = "    ";    // Empty space for indentation

// Special strings and emoji for file types
pub const EMOJI_DIRECTORY: &str = "📁 ";
pub const EMOJI_FILE: &str = "📄 ";
pub const EMOJI_IMAGE: &str = "🖼️ ";
pub const EMOJI_VIDEO: &str = "🎬 ";
pub const EMOJI_AUDIO: &str = "🎵 ";
pub const EMOJI_ARCHIVE: &str = "📦 ";
pub const EMOJI_CODE: &str = "📝 ";
pub const EMOJI_LINK: &str = "🔗 ";
pub const EMOJI_HIDDEN: &str = "👁️ ";
pub const EMOJI_LOCK: &str = "🔒 ";

/// Determines whether to use colors based on config and terminal capabilities
pub fn should_use_colors(config: &DisplayConfig) -> bool {
    if !config.use_colors || config.color_theme == ColorTheme::None {
        return false;
    }
    
    colored::control::SHOULD_COLORIZE.should_colorize()
}

/// Returns whether to use emoji based on config
pub fn should_use_emoji(config: &DisplayConfig) -> bool {
    config.use_emoji && should_use_colors(config)
}

/// Determine the file type from extension and metadata
pub(super) fn determine_file_type(entry: &DirectoryEntry) -> FileType {
    if entry.is_dir {
        return FileType::Directory;
    }
    
    if entry.path.is_symlink() {
        return FileType::Symlink;
    }
    
    if entry.name.starts_with('.') {
        return FileType::Hidden;
    }
    
    let path = &entry.path;
    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            let ext = ext_str.to_lowercase();
            
            // Images
            if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "webp" | "svg") {
                return FileType::Image;
            }
            
            // Videos
            if matches!(ext.as_str(), "mp4" | "mov" | "avi" | "mkv" | "webm" | "flv" | "wmv") {
                return FileType::Video;
            }
            
            // Audio
            if matches!(ext.as_str(), "mp3" | "wav" | "ogg" | "flac" | "aac" | "m4a") {
                return FileType::Audio;
            }
            
            // Archives
            if matches!(ext.as_str(), "zip" | "rar" | "tar" | "gz" | "7z" | "bz2" | "xz") {
                return FileType::Archive;
            }
            
            // Code files
            if matches!(ext.as_str(), 
                "rs" | "py" | "js" | "ts" | "c" | "cpp" | "h" | "hpp" | "java" | "go" | 
                "rb" | "php" | "html" | "css" | "scss" | "jsx" | "tsx" | "swift" | "kt" | 
                "scala" | "sh" | "bash" | "pl" | "exs" | "clj") {
                return FileType::Code;
            }
            
            // Documents
            if matches!(ext.as_str(), "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "md" | "rst") {
                return FileType::Document;
            }
            
            // Executables
            if matches!(ext.as_str(), "exe" | "dll" | "so" | "dylib" | "bin") {
                return FileType::Executable;
            }
        }
    }
    
    // Check if file is executable (only works on Unix-like systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = path.metadata() {
            let permissions = metadata.permissions();
            if permissions.mode() & 0o111 != 0 {
                return FileType::Executable;
            }
        }
    }
    
    FileType::Regular
}

/// Get emoji for file type
pub(super) fn get_file_emoji(file_type: FileType) -> &'static str {
    match file_type {
        FileType::Directory => EMOJI_DIRECTORY,
        FileType::Symlink => EMOJI_LINK,
        FileType::Image => EMOJI_IMAGE,
        FileType::Video => EMOJI_VIDEO,
        FileType::Audio => EMOJI_AUDIO,
        FileType::Archive => EMOJI_ARCHIVE,
        FileType::Code => EMOJI_CODE,
        FileType::Document => EMOJI_FILE,
        FileType::Executable => EMOJI_LOCK,
        FileType::Hidden => EMOJI_HIDDEN,
        FileType::Regular => EMOJI_FILE,
    }
}

/// Get the appropriate color for a file name based on its type
pub(super) fn get_name_color(entry: &DirectoryEntry, config: &DisplayConfig) -> Color {
    let file_type = determine_file_type(entry);
    
    match config.color_theme {
        ColorTheme::Light => match file_type {
            FileType::Directory => Color::Blue,
            FileType::Symlink => Color::Cyan,
            FileType::Image => Color::Magenta,
            FileType::Video => Color::Magenta,
            FileType::Audio => Color::Yellow,
            FileType::Archive => Color::Red,
            FileType::Code => Color::Green,
            FileType::Document => Color::Blue,
            FileType::Executable => Color::Red,
            FileType::Hidden => Color::BrightBlack,
            FileType::Regular => Color::Black,
        },
        ColorTheme::Dark => match file_type {
            FileType::Directory => Color::BrightBlue,
            FileType::Symlink => Color::BrightCyan,
            FileType::Image => Color::BrightMagenta,
            FileType::Video => Color::BrightMagenta,
            FileType::Audio => Color::BrightYellow,
            FileType::Archive => Color::BrightRed,
            FileType::Code => Color::BrightGreen,
            FileType::Document => Color::BrightBlue,
            FileType::Executable => Color::BrightRed,
            FileType::Hidden => Color::BrightBlack,
            FileType::Regular => Color::White,
        },
        _ => match file_type {
            // Auto mode - use system settings or dark by default
            FileType::Directory => Color::BrightBlue,
            FileType::Symlink => Color::BrightCyan,
            FileType::Image => Color::BrightMagenta,
            FileType::Video => Color::BrightMagenta,
            FileType::Audio => Color::BrightYellow,
            FileType::Archive => Color::BrightRed,
            FileType::Code => Color::BrightGreen,
            FileType::Document => Color::BrightBlue,
            FileType::Executable => Color::BrightRed,
            FileType::Hidden => Color::BrightBlack,
            FileType::Regular => Color::White,
        }
    }
}

/// Get the color for gitignored entries
pub(super) fn get_gitignored_color(config: &DisplayConfig) -> Color {
    match config.color_theme {
        ColorTheme::Light => Color::BrightBlack,  // Gray for light theme
        ColorTheme::Dark => Color::BrightBlack,   // Gray for dark theme
        _ => Color::BrightBlack,                  // Gray for auto
    }
}

/// Get color for file size based on size (gradient from small to large)
pub(super) fn get_size_color(size_bytes: u64, config: &DisplayConfig) -> Color {
    match config.color_theme {
        ColorTheme::Light => {
            if size_bytes < 1024 {  // < 1KB
                Color::Green
            } else if size_bytes < 1024 * 1024 {  // < 1MB
                Color::Blue
            } else if size_bytes < 100 * 1024 * 1024 {  // < 100MB
                Color::Yellow
            } else if size_bytes < 1024 * 1024 * 1024 {  // < 1GB
                Color::Red
            } else {  // >= 1GB
                Color::Magenta
            }
        },
        _ => {  // Dark or Auto
            if size_bytes < 1024 {  // < 1KB
                Color::BrightGreen
            } else if size_bytes < 1024 * 1024 {  // < 1MB
                Color::BrightBlue
            } else if size_bytes < 100 * 1024 * 1024 {  // < 100MB
                Color::BrightYellow
            } else if size_bytes < 1024 * 1024 * 1024 {  // < 1GB
                Color::BrightRed
            } else {  // >= 1GB
                Color::BrightMagenta
            }
        }
    }
}

/// Get color for date based on recency
pub(super) fn get_date_color(seconds_ago: u64, config: &DisplayConfig) -> Color {
    match config.color_theme {
        ColorTheme::Light => {
            if seconds_ago < 3600 {  // < 1 hour
                Color::Green
            } else if seconds_ago < 86400 {  // < 1 day
                Color::Blue
            } else if seconds_ago < 7 * 86400 {  // < 1 week
                Color::Yellow
            } else if seconds_ago < 30 * 86400 {  // < 1 month
                Color::Magenta
            } else {  // >= 1 month
                Color::BrightBlack
            }
        },
        _ => {  // Dark or Auto
            if seconds_ago < 3600 {  // < 1 hour
                Color::BrightGreen
            } else if seconds_ago < 86400 {  // < 1 day
                Color::BrightBlue
            } else if seconds_ago < 7 * 86400 {  // < 1 week
                Color::BrightYellow
            } else if seconds_ago < 30 * 86400 {  // < 1 month
                Color::BrightMagenta
            } else {  // >= 1 month
                Color::BrightBlack
            }
        }
    }
}

/// Get the color for metadata like size, date, etc.
pub(super) fn get_metadata_color(config: &DisplayConfig) -> Color {
    match config.color_theme {
        ColorTheme::Light => Color::BrightBlack,   // Gray for light theme
        ColorTheme::Dark => Color::BrightBlack,    // Gray for dark theme
        _ => Color::BrightBlack,                   // Gray for auto
    }
}

/// Get the color for tree connectors
pub(super) fn get_connector_color(config: &DisplayConfig) -> Color {
    match config.color_theme {
        ColorTheme::Light => Color::BrightBlack,   // Gray for light theme
        ColorTheme::Dark => Color::BrightBlack,    // Gray for dark theme
        _ => Color::BrightBlack,                   // Gray for auto
    }
}

/// Get the color for "hidden items" message
pub(super) fn get_hidden_items_color(config: &DisplayConfig) -> Color {
    match config.color_theme {
        ColorTheme::Light => Color::Yellow,   
        ColorTheme::Dark => Color::Yellow,    
        _ => Color::Yellow,                   
    }
}

/// Colorize a string if colors are enabled, otherwise return it as-is
pub(super) fn colorize(text: &str, color: Color, config: &DisplayConfig) -> String {
    if should_use_colors(config) {
        text.color(color).to_string()
    } else {
        text.to_string()
    }
}

/// Colorize with custom styling (bold, underline, etc.)
pub(super) fn colorize_styled(
    text: &str, 
    color: Color, 
    bold: bool,
    config: &DisplayConfig
) -> String {
    if !should_use_colors(config) {
        return text.to_string();
    }
    
    let mut colored_text: ColoredString = text.color(color);
    
    if bold {
        colored_text = colored_text.bold();
    }
    
    colored_text.to_string()
}

/// Format a file path for display with optional emoji
pub(super) fn format_name_with_emoji(
    entry: &DirectoryEntry,
    config: &DisplayConfig
) -> String {
    if !should_use_emoji(config) {
        return entry.name.clone();
    }
    
    let file_type = determine_file_type(entry);
    let emoji = get_file_emoji(file_type);
    
    format!("{}{}", emoji, entry.name)
}