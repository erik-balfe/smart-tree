use super::state::DisplayState;
use super::utils::{format_metadata, sort_entries};
use crate::types::{DirectoryEntry, DisplayConfig};
use anyhow::Result;

pub fn format_tree(root: &DirectoryEntry, config: &DisplayConfig) -> Result<String> {
    let mut state = DisplayState::new(config.max_lines, config);

    state.output.push_str(".\n");
    state.lines_remaining -= 1;

    let mut children = root.children.clone();
    sort_entries(&mut children, config);

    state.show_items(&children, "");

    Ok(state.output)
}

#[allow(dead_code)]
fn format_single_entry(
    entry: &DirectoryEntry,
    prefix: &str,
    is_last: bool,
    _config: &DisplayConfig, // Renamed to _config to avoid unused warning
) -> String {
    let connector = if is_last { "└── " } else { "├── " };

    let mut output = format!("{}{}{}", prefix, connector, entry.name);

    if entry.is_gitignored && entry.is_dir {
        output.push_str(&format!(" {} [folded: system]\n", format_metadata(entry)));
        return output;
    }

    output.push_str(&format!(" {}\n", format_metadata(entry)));
    output
}
