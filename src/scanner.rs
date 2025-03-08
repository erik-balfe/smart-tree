use crate::gitignore::GitIgnoreContext;
use crate::types::{DirectoryEntry, EntryMetadata};
use anyhow::Result;
use log::{debug, warn};
use std::fs;
use std::path::Path;

pub fn scan_directory(
    root: &Path,
    gitignore_ctx: &mut GitIgnoreContext,
    max_depth: usize,
    show_system_dirs: Option<bool>,
) -> Result<DirectoryEntry> {
    // Default to not showing system directories if not specified
    let show_system = show_system_dirs.unwrap_or(false);
    let root_metadata = fs::metadata(root)?;
    let root_name = root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| root.to_string_lossy().to_string());

    // Process this directory to load any .gitignore file before checking ignore status
    if let Err(e) = gitignore_ctx.process_directory(root) {
        warn!("Error processing gitignore in {}: {}", root.display(), e);
    }

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
            is_gitignored: gitignore_ctx.is_ignored(root),
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
        is_gitignored: gitignore_ctx.is_ignored(root),
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
    // If we're showing system directories, we'll continue with the normal traversal

    let mut entries = Vec::new();

    // Read the directory and process entries
    for dir_entry in fs::read_dir(root)? {
        let dir_entry = dir_entry?;
        let path = dir_entry.path();
        let metadata = dir_entry.metadata()?;
        let name = dir_entry.file_name().to_string_lossy().to_string();
        
        // Check if this specific entry is gitignored
        let is_gitignored = gitignore_ctx.is_ignored(&path);

        if metadata.is_dir() {
            // Recursively scan subdirectories if depth allows
            if max_depth > 1 {
                match scan_directory(&path, gitignore_ctx, max_depth - 1, Some(show_system)) {
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
