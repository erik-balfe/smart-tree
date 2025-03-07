use super::state::DisplayState;
use super::utils::{format_metadata, sort_entries};
use super::colors;
use crate::types::{DirectoryEntry, DisplayConfig};
use anyhow::Result;

pub fn format_tree(root: &DirectoryEntry, config: &DisplayConfig) -> Result<String> {
    let mut state = DisplayState::new(config.max_lines, config);

    // Colorize the root directory entry
    let root_dir = colors::colorize_styled(
        ".",
        colors::get_name_color(root, config),
        true, // Bold for directory
        config
    );
    state.output.push_str(&format!("{}\n", root_dir));
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
    config: &DisplayConfig,
) -> String {
    let connector_str = if is_last { colors::TREE_CORNER } else { colors::TREE_BRANCH };
    
    // Get colorized connector
    let connector = colors::colorize(
        connector_str, 
        colors::get_connector_color(config),
        config
    );
    
    // Get colorized prefix (tree lines)
    let colorized_prefix = colors::colorize(
        prefix, 
        colors::get_connector_color(config),
        config
    );
    
    // Get colorized name
    let name_color = if entry.is_gitignored {
        colors::get_gitignored_color(config)
    } else {
        colors::get_name_color(entry, config)
    };
    
    let name = colors::colorize_styled(
        &entry.name,
        name_color,
        entry.is_dir, // Bold directories
        config
    );
    
    // Format metadata with colors
    let metadata_str = format_metadata(entry);
    let metadata = colors::colorize(
        &metadata_str,
        colors::get_metadata_color(config),
        config
    );

    let mut output = format!("{}{}{}", colorized_prefix, connector, name);

    if entry.is_gitignored && entry.is_dir {
        let folded_text = colors::colorize(
            " [folded: system]",
            colors::get_gitignored_color(config),
            config
        );
        output.push_str(&format!(" {}{}\n", metadata, folded_text));
        return output;
    }

    output.push_str(&format!(" {}\n", metadata));
    output
}
