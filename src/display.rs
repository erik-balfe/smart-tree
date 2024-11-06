use crate::types::{DirectoryEntry, DisplayConfig, SortBy};
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn format_tree(root: &DirectoryEntry, config: &DisplayConfig) -> Result<String> {
    let mut output = String::new();
    let mut lines_remaining = config.max_lines;

    // Start with root
    output.push_str(".\n");

    // Process root's children
    let mut children = root.children.clone();
    sort_entries(&mut children, config);

    let total = children.len();
    let mut processed = 0;

    for (i, child) in children.iter().enumerate() {
        if lines_remaining == 0 {
            break;
        }
        processed += 1;
        format_entry(
            child,
            "",
            i == total - 1,
            config,
            &mut lines_remaining,
            &mut output,
        );
    }

    if processed < total && lines_remaining > 0 {
        output.push_str(&format!("... {} more items\n", total - processed));
    }

    Ok(output)
}

fn format_entry(
    entry: &DirectoryEntry,
    prefix: &str,
    is_last: bool,
    config: &DisplayConfig,
    lines_remaining: &mut usize,
    output: &mut String,
) {
    if *lines_remaining == 0 {
        return;
    }

    let connector = if is_last { "└── " } else { "├── " };
    let child_prefix = if is_last { "    " } else { "│   " };

    // Format the current entry
    let mut entry_line = format!("{}{}{}", prefix, connector, entry.name,);

    if entry.is_gitignored {
        // Show the directory with its metadata
        entry_line.push_str(&format!(" {}", format_metadata(entry)));
        output.push_str(&entry_line);
        output.push('\n');
        *lines_remaining -= 1;

        // Add an indented note about why it's folded
        if entry.is_dir {
            output.push_str(&format!(
                "{}{}... contents folded (.gitignore)\n",
                prefix, child_prefix
            ));
            *lines_remaining -= 1;
        }
        return;
    } else {
        entry_line.push_str(&format!(" {}", format_metadata(entry)));
        output.push_str(&entry_line);
        output.push('\n');
        *lines_remaining -= 1;
    }
    // Process children if it's a directory and not ignored
    if entry.is_dir && !entry.is_gitignored {
        let mut children = entry.children.clone();
        sort_entries(&mut children, config);

        let to_show = children.len().min(config.dir_limit);
        let hidden = children.len() - to_show;

        for (i, child) in children.iter().take(to_show).enumerate() {
            if *lines_remaining == 0 {
                return;
            }
            format_entry(
                child,
                &format!("{}{}", prefix, child_prefix),
                i == to_show - 1 && hidden == 0,
                config,
                lines_remaining,
                output,
            );
        }

        if hidden > 0 && *lines_remaining > 0 {
            output.push_str(&format!(
                "{}{}... {} more items\n",
                prefix, child_prefix, hidden
            ));
            *lines_remaining -= 1;
        }
    }
}

fn format_metadata(entry: &DirectoryEntry) -> String {
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

fn format_size(size: u64) -> String {
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

fn format_time(time: SystemTime) -> String {
    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = duration.as_secs();

    // Get current time
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

fn sort_entries(entries: &mut Vec<DirectoryEntry>, config: &DisplayConfig) {
    entries.sort_by(|a, b| {
        if config.dirs_first {
            // Directories come before files
            if a.is_dir && !b.is_dir {
                return std::cmp::Ordering::Less;
            }
            if !a.is_dir && b.is_dir {
                return std::cmp::Ordering::Greater;
            }
        }

        // Then apply the selected sort criteria
        match config.sort_by {
            SortBy::Name => a.name.cmp(&b.name),
            SortBy::Size => b.metadata.size.cmp(&a.metadata.size),
            SortBy::Modified => b.metadata.modified.cmp(&a.metadata.modified),
            SortBy::Created => b.metadata.created.cmp(&a.metadata.created),
        }
    });
}
