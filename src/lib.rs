//! Smart tree display library

mod display;
mod gitignore;
mod log_macros;
mod scanner;
mod tests;
mod types;

// Re-export public items
pub use display::{format_tree, should_use_colors};
pub use gitignore::{GitIgnore, GitIgnoreContext};
pub use scanner::scan_directory;
pub use types::{ColorTheme, DirectoryEntry, DisplayConfig, EntryMetadata, SortBy};

// Convenience wrapper for backward compatibility 
#[deprecated(since = "0.2.1", note = "Use scan_directory with GitIgnoreContext instead")]
pub fn scan_directory_simple(
    root: &std::path::Path, 
    gitignore: &mut GitIgnoreContext, 
    max_depth: usize
) -> anyhow::Result<DirectoryEntry> {
    scanner::scan_directory(root, gitignore, max_depth, None)
}

// Another wrapper for backward compatibility with older GitIgnore API
#[deprecated(since = "0.3.0", note = "Use scan_directory with GitIgnoreContext instead")]
pub fn scan_directory_with_legacy_gitignore(
    root: &std::path::Path,
    gitignore: &GitIgnore,  // Using the old GitIgnore API
    max_depth: usize,
    show_system_dirs: Option<bool>
) -> anyhow::Result<DirectoryEntry> {
    use crate::types::{DirectoryEntry, EntryMetadata};
    use anyhow::Result;
    use log::{debug, warn};
    use std::fs;
    use std::path::Path;
    
    // Default to not showing system directories if not specified
    let show_system = show_system_dirs.unwrap_or(false);
    let root_metadata = fs::metadata(root)?;
    let root_name = root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| root.to_string_lossy().to_string());

    // Early return for non-directories or when max_depth is 0
    if !root_metadata.is_dir() || max_depth == 0 {
        return Ok(DirectoryEntry {
            path: root.to_path_buf(),
            name: root_name,
            is_dir: root_metadata.is_dir(),
            metadata: EntryMetadata {
                size: root_metadata.len(),
                created: root_metadata.created()?,
                modified: root_metadata.modified()?,
                files_count: 0,
            },
            children: Vec::new(),
            is_gitignored: gitignore.is_ignored(root),
        });
    }

    // Initialize the root entry with temporary metadata
    // We'll calculate accurate size and file count as we traverse
    let mut root_entry = DirectoryEntry {
        path: root.to_path_buf(),
        name: root_name,
        is_dir: true,
        metadata: EntryMetadata {
            size: 0,
            created: root_metadata.created()?,
            modified: root_metadata.modified()?,
            files_count: 0,
        },
        children: Vec::new(),
        is_gitignored: gitignore.is_ignored(root),
    };

    // For gitignored directories, decide whether to traverse or just provide basic metadata
    if root_entry.is_gitignored && !show_system {
        debug!("Skipping deep traversal of system directory: {}", root.display());
        // If not showing system directories, do a quick scan to get file counts without deep traversal
        let mut file_count = 0;
        let mut total_size = 0;

        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    total_size += metadata.len();
                    if !metadata.is_dir() {
                        file_count += 1;
                    } else {
                        // For directories, make a rough estimate
                        // This avoids traversing deeply into large system directories
                        file_count += 10; // Just a placeholder estimate
                    }
                }
            }
        }

        // If total size is still 0 but we know it's a directory, use a placeholder size
        if total_size == 0 && file_count > 0 {
            total_size = 1024 * 1024; // 1MB placeholder
        }

        // Update the metadata
        root_entry.metadata.files_count = file_count;
        root_entry.metadata.size = total_size;

        return Ok(root_entry);
    }
    
    let mut entries = Vec::new();

    // Read the directory and process entries
    for dir_entry in fs::read_dir(root)? {
        let dir_entry = dir_entry?;
        let path = dir_entry.path();
        let metadata = dir_entry.metadata()?;
        let name = dir_entry.file_name().to_string_lossy().to_string();
        
        // Check if this specific entry is gitignored
        let is_gitignored = gitignore.is_ignored(&path);

        if metadata.is_dir() {
            // Recursively scan subdirectories if depth allows
            if max_depth > 1 {
                match scan_directory_with_legacy_gitignore(&path, gitignore, max_depth - 1, Some(show_system)) {
                    Ok(dir_entry) => {
                        // Update parent metadata
                        root_entry.metadata.files_count += dir_entry.metadata.files_count;
                        root_entry.metadata.size += dir_entry.metadata.size;
                        entries.push(dir_entry);
                    }
                    Err(e) => {
                        warn!("Error scanning directory {}: {}", path.display(), e);
                    }
                }
            } else {
                // Just add the directory as a leaf node
                entries.push(DirectoryEntry {
                    path,
                    name,
                    is_dir: true,
                    metadata: EntryMetadata {
                        size: metadata.len(),
                        created: metadata.created()?,
                        modified: metadata.modified()?,
                        files_count: 0,
                    },
                    children: Vec::new(),
                    is_gitignored,
                });

                // Update parent size
                root_entry.metadata.size += metadata.len();
            }
        } else {
            // For files, update parent metadata and add to entries
            root_entry.metadata.files_count += 1;
            root_entry.metadata.size += metadata.len();

            entries.push(DirectoryEntry {
                path,
                name,
                is_dir: false,
                metadata: EntryMetadata {
                    size: metadata.len(),
                    created: metadata.created()?,
                    modified: metadata.modified()?,
                    files_count: 0,
                },
                children: Vec::new(),
                is_gitignored,
            });
        }
    }

    // Set the children
    root_entry.children = entries;

    Ok(root_entry)
}
