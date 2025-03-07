use crate::types::{ColorTheme, DirectoryEntry, DisplayConfig};
use colored::{Color, Colorize, ColoredString};

// Tree connectors with padding
pub const TREE_BRANCH: &str = "├── ";  // T-shape connector
pub const TREE_CORNER: &str = "└── ";  // L-shape corner connector
pub const TREE_VERTICAL: &str = "│   "; // Vertical line with spacing
pub const TREE_SPACE: &str = "    ";    // Empty space for indentation

/// Determines whether to use colors based on config and terminal capabilities
pub fn should_use_colors(config: &DisplayConfig) -> bool {
    if !config.use_colors || config.color_theme == ColorTheme::None {
        return false;
    }
    
    colored::control::SHOULD_COLORIZE.should_colorize()
}

/// Get the appropriate color for a file name based on its type
pub(super) fn get_name_color(entry: &DirectoryEntry, config: &DisplayConfig) -> Color {
    match config.color_theme {
        ColorTheme::Light => {
            if entry.is_dir {
                Color::Blue
            } else if entry.path.is_symlink() {
                Color::Cyan
            } else {
                Color::Black
            }
        },
        ColorTheme::Dark => {
            if entry.is_dir {
                Color::BrightBlue
            } else if entry.path.is_symlink() {
                Color::BrightCyan
            } else {
                Color::White
            }
        },
        _ => {
            // Auto mode - use system settings or dark by default
            if entry.is_dir {
                Color::BrightBlue
            } else if entry.path.is_symlink() {
                Color::BrightCyan
            } else {
                Color::White
            }
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