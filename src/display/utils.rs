use crate::types::{DirectoryEntry, DisplayConfig, SortBy};
use std::time::{SystemTime, UNIX_EPOCH};

pub(super) fn format_metadata(entry: &DirectoryEntry) -> String {
    let size = format_size(entry.metadata.size);
    let modified = format_time(entry.metadata.modified);

    if entry.is_dir {
        format!(
            "({} files, {}, modified {})",
            entry.metadata.files_count, size, modified
        )
    } else {
        format!("({}, modified {})", size, modified)
    }
}

pub(super) fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.1}GB", size as f64 / GB as f64)
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

    let diff = now - secs;

    if diff < 60 {
        "just now".to_string()
    } else if diff < 3600 {
        format!("{}m ago", diff / 60)
    } else if diff < 86400 {
        format!("{}h ago", diff / 3600)
    } else {
        format!("{}d ago", diff / 86400)
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
