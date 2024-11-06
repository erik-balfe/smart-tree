use crate::types::{DirectoryEntry, DisplayConfig, SortBy};
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn format_tree(root: &DirectoryEntry, config: &DisplayConfig) -> Result<String> {
    let mut output = String::new();
    let mut lines_remaining = config.max_lines;

    // Start with root
    output.push_str(".\n");
    lines_remaining -= 1;

    // First pass: count minimum required lines
    let min_lines = count_essential_lines(&root.children);
    let can_expand = lines_remaining > min_lines;

    // Process root's children with dynamic folding
    let mut children = root.children.clone();
    sort_entries(&mut children, config);

    let total_items = children.len();
    let mut current_index = 0;

    // First pass - show as many items as possible
    while current_index < children.len() && lines_remaining > 1 {
        // Keep one line for potential summary
        format_entry(
            &children[current_index],
            "",
            current_index == children.len() - 1,
            config,
            can_expand,
            &mut lines_remaining,
            &mut output,
        );
        current_index += 1;
    }

    // If we couldn't show all items and have a line remaining, show summary
    if current_index < total_items && lines_remaining > 0 {
        let hidden_count = total_items - current_index;
        if hidden_count > 1 {
            output.push_str(&format!(
                "├── ... {} more items (use --max-lines {} to see all)\n",
                hidden_count,
                total_items + 1
            ));
        } else if hidden_count == 1 {
            // Show the last item if possible
            format_entry(
                &children[current_index],
                "",
                true,
                config,
                false,
                &mut lines_remaining,
                &mut output,
            );
        }
    }

    Ok(output)
}
fn count_essential_lines(entries: &[DirectoryEntry]) -> usize {
    entries.len() + // one line per entry
    entries.iter().filter(|e| e.is_gitignored && e.is_dir).count() // folding messages
}

/// Formats directory contents with smart display logic:
/// When content doesn't fit within limits, shows both ends of the sorted list
/// to maintain sorting context (e.g., newest and oldest files when sorting by date)
fn format_directory_contents(
    children: &[DirectoryEntry],
    config: &DisplayConfig,
    prefix: &str,
    can_expand: bool,
    lines_remaining: &mut usize,
    output: &mut String,
) {
    let total = children.len();
    if total == 0 {
        return;
    }

    // Determine how many items to show
    let items_to_show = if can_expand {
        total
    } else {
        config.dir_limit.min(total)
    };

    let show_head_tail = !can_expand && total > items_to_show;

    if show_head_tail {
        let items_each_end = items_to_show / 2;
        let hidden_count = total - (items_each_end * 2);

        // Show first N items
        for (_i, child) in children.iter().take(items_each_end).enumerate() {
            if *lines_remaining == 0 {
                return;
            }
            format_entry(
                child,
                prefix,
                false,
                config,
                can_expand,
                lines_remaining,
                output,
            );
        }

        // Show hidden items count
        if *lines_remaining > 0 && hidden_count > 0 {
            if hidden_count > 1 {
                output.push_str(&format!(
                    "{}│   ... {} items folded (use --dir-limit {} to see all) ...\n",
                    prefix, hidden_count, total
                ));
                *lines_remaining -= 1;
            } else {
                // If only one item is hidden and we have space, try to show it
                if let Some(hidden_item) = children
                    .iter()
                    .skip(items_each_end)
                    .take(hidden_count)
                    .next()
                {
                    format_entry(
                        hidden_item,
                        prefix,
                        true,
                        config,
                        can_expand,
                        lines_remaining,
                        output,
                    );
                }
            }
        }

        // Show last N items
        for (i, child) in children.iter().skip(total - items_each_end).enumerate() {
            if *lines_remaining == 0 {
                return;
            }
            let is_last = i == items_each_end - 1;
            format_entry(
                child,
                prefix,
                is_last,
                config,
                can_expand,
                lines_remaining,
                output,
            );
        }
    } else {
        // Show all or first N items
        for (i, child) in children.iter().take(items_to_show).enumerate() {
            if *lines_remaining == 0 {
                return;
            }
            format_entry(
                child,
                prefix,
                i == items_to_show - 1,
                config,
                can_expand,
                lines_remaining,
                output,
            );
        }
    }
}

fn format_entry(
    entry: &DirectoryEntry,
    prefix: &str,
    is_last: bool,
    config: &DisplayConfig,
    can_expand: bool,
    lines_remaining: &mut usize,
    output: &mut String,
) {
    if *lines_remaining == 0 {
        return;
    }

    let connector = if is_last { "└── " } else { "├── " };
    let child_prefix = if is_last { "    " } else { "│   " };

    // Format the current entry
    let mut entry_line = format!("{}{}{}", prefix, connector, entry.name);

    // Handle ignored directories - always fold them regardless of can_expand
    if entry.is_gitignored && entry.is_dir {
        entry_line.push_str(&format!(" {} [folded: system]", format_metadata(entry)));
        output.push_str(&entry_line);
        output.push('\n');
        *lines_remaining -= 1;
        return; // Stop here, don't process children of ignored directories
    }

    // Add metadata
    entry_line.push_str(&format!(" {}", format_metadata(entry)));
    output.push_str(&entry_line);
    output.push('\n');
    *lines_remaining -= 1;

    // Process children if it's a directory and NOT ignored
    if entry.is_dir {
        let mut children = entry.children.clone();
        sort_entries(&mut children, config);

        format_directory_contents(
            &children,
            config,
            &format!("{}{}", prefix, child_prefix),
            // Only allow expansion for non-ignored directories
            can_expand && !entry.is_gitignored,
            lines_remaining,
            output,
        );
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
