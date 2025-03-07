use crate::types::{DirectoryEntry, DisplayConfig, SortBy};
use std::time::{SystemTime, UNIX_EPOCH};
use super::colors;

pub(super) fn format_metadata(entry: &DirectoryEntry) -> String {
    if entry.is_dir {
        format_directory_metadata(entry)
    } else {
        format_file_metadata(entry)
    }
}

pub(super) fn format_directory_metadata(entry: &DirectoryEntry) -> String {
    let files_count = entry.metadata.files_count.to_string();
    let size = format_size(entry.metadata.size);
    let modified = format_time(entry.metadata.modified);
    
    format!(
        "({} files, {}, modified {})",
        files_count, size, modified
    )
}

pub(super) fn format_file_metadata(entry: &DirectoryEntry) -> String {
    let size = format_size(entry.metadata.size);
    let modified = format_time(entry.metadata.modified);
    
    format!("({}, modified {})", size, modified)
}

pub(super) fn format_colorized_metadata(entry: &DirectoryEntry, config: &DisplayConfig) -> String {
    if !colors::should_use_colors(config) {
        return format_metadata(entry);
    }
    
    // Get the time difference in seconds for coloring
    let duration = entry.metadata.modified
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let modified_secs = duration.as_secs();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let time_diff = now.saturating_sub(modified_secs);
    
    if entry.is_dir {
        let files_text = format!("{} files", entry.metadata.files_count);
        let files_colored = if config.size_colorize {
            colors::colorize(
                &files_text,
                colors::get_size_color(entry.metadata.size, config),
                config
            )
        } else {
            colors::colorize(
                &files_text,
                colors::get_metadata_color(config),
                config
            )
        };
        
        let size_text = format_size(entry.metadata.size);
        let size_colored = if config.size_colorize {
            colors::colorize(
                &size_text,
                colors::get_size_color(entry.metadata.size, config),
                config
            )
        } else {
            colors::colorize(
                &size_text,
                colors::get_metadata_color(config),
                config
            )
        };
        
        let date_text = format!("modified {}", format_time(entry.metadata.modified));
        let date_colored = if config.date_colorize {
            colors::colorize(
                &date_text,
                colors::get_date_color(time_diff, config),
                config
            )
        } else {
            colors::colorize(
                &date_text,
                colors::get_metadata_color(config),
                config
            )
        };
        
        format!("({}, {}, {})", files_colored, size_colored, date_colored)
    } else {
        let size_text = format_size(entry.metadata.size);
        let size_colored = if config.size_colorize {
            colors::colorize(
                &size_text,
                colors::get_size_color(entry.metadata.size, config),
                config
            )
        } else {
            colors::colorize(
                &size_text,
                colors::get_metadata_color(config),
                config
            )
        };
        
        let date_text = format!("modified {}", format_time(entry.metadata.modified));
        let date_colored = if config.date_colorize {
            colors::colorize(
                &date_text,
                colors::get_date_color(time_diff, config),
                config
            )
        } else {
            colors::colorize(
                &date_text,
                colors::get_metadata_color(config),
                config
            )
        };
        
        format!("({}, {})", size_colored, date_colored)
    }
}

pub(super) fn format_detailed_metadata(entry: &DirectoryEntry, config: &DisplayConfig) -> String {
    if !config.detailed_metadata {
        return format_colorized_metadata(entry, config);
    }
    
    // Get the time difference in seconds for coloring
    let duration = entry.metadata.modified
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let modified_secs = duration.as_secs();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let time_diff = now.saturating_sub(modified_secs);
    
    let created_duration = entry.metadata.created
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let created_secs = created_duration.as_secs();
    let created_diff = now.saturating_sub(created_secs);
    
    let file_type = colors::determine_file_type(entry);
    let type_str = format!("{:?}", file_type);
    
    // Start building parts
    let mut parts = Vec::new();
    
    // Size info
    let size_text = format_size(entry.metadata.size);
    let size_colored = if config.size_colorize {
        colors::colorize(
            &size_text,
            colors::get_size_color(entry.metadata.size, config),
            config
        )
    } else {
        colors::colorize(
            &size_text,
            colors::get_metadata_color(config),
            config
        )
    };
    parts.push(size_colored);
    
    // Type info
    let type_colored = colors::colorize(
        &type_str,
        colors::get_name_color(entry, config),
        config
    );
    parts.push(type_colored);
    
    // Date info - modified
    let modified_text = format!("mod: {}", format_time(entry.metadata.modified));
    let modified_colored = if config.date_colorize {
        colors::colorize(
            &modified_text,
            colors::get_date_color(time_diff, config),
            config
        )
    } else {
        colors::colorize(
            &modified_text,
            colors::get_metadata_color(config),
            config
        )
    };
    parts.push(modified_colored);
    
    // Date info - created
    let created_text = format!("created: {}", format_time(entry.metadata.created));
    let created_colored = if config.date_colorize {
        colors::colorize(
            &created_text,
            colors::get_date_color(created_diff, config),
            config
        )
    } else {
        colors::colorize(
            &created_text,
            colors::get_metadata_color(config),
            config
        )
    };
    parts.push(created_colored);
    
    // Add files count for directories
    if entry.is_dir {
        let files_text = format!("{} files", entry.metadata.files_count);
        let files_colored = colors::colorize(
            &files_text,
            colors::get_metadata_color(config),
            config
        );
        parts.push(files_colored);
    }
    
    format!("({})", parts.join(", "))
}

pub(super) fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if size >= TB {
        format!("{:.2}TB", size as f64 / TB as f64)
    } else if size >= GB {
        format!("{:.2}GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.1}MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.1}KB", size as f64 / KB as f64)
    } else {
        format!("{}B", size)
    }
}

pub(super) fn format_time(time: SystemTime) -> String {
    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = duration.as_secs();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let diff = now.saturating_sub(secs);

    if diff < 60 {
        "just now".to_string()
    } else if diff < 3600 {
        format!("{}m ago", diff / 60)
    } else if diff < 86400 {
        format!("{}h ago", diff / 3600)
    } else if diff < 7 * 86400 {
        format!("{}d ago", diff / 86400)
    } else if diff < 30 * 86400 {
        format!("{}w ago", diff / (7 * 86400))
    } else if diff < 365 * 86400 {
        format!("{}mo ago", diff / (30 * 86400))
    } else {
        format!("{}y ago", diff / (365 * 86400))
    }
}

pub(super) fn sort_entries(entries: &mut [DirectoryEntry], config: &DisplayConfig) {
    entries.sort_by(|a, b| {
        if config.dirs_first {
            if a.is_dir && !b.is_dir {
                return std::cmp::Ordering::Less;
            }
            if !a.is_dir && b.is_dir {
                return std::cmp::Ordering::Greater;
            }
        }

        match config.sort_by {
            SortBy::Name => a.name.cmp(&b.name),
            SortBy::Size => b.metadata.size.cmp(&a.metadata.size),
            SortBy::Modified => b.metadata.modified.cmp(&a.metadata.modified),
            SortBy::Created => b.metadata.created.cmp(&a.metadata.created),
        }
    });
}
