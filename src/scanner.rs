use crate::gitignore::GitIgnore;
use crate::types::{DirectoryEntry, EntryMetadata};
use anyhow::Result;
use std::collections::VecDeque;
use std::fs;
use std::path::Path;

pub fn scan_directory(
    root: &Path,
    gitignore: &GitIgnore,
    max_depth: usize,
) -> Result<DirectoryEntry> {
    let metadata = fs::metadata(root)?;
    let name = root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| root.to_string_lossy().to_string());

    // Create root entry
    let mut root_entry = DirectoryEntry {
        path: root.to_path_buf(),
        name,
        is_dir: metadata.is_dir(),
        metadata: EntryMetadata {
            size: metadata.len(),
            created: metadata.created()?,
            modified: metadata.modified()?,
            files_count: 0,
        },
        children: Vec::new(),
        is_gitignored: gitignore.is_ignored(root),
    };

    if !root_entry.is_dir || max_depth == 0 {
        return Ok(root_entry);
    }

    // Queue will contain paths and their parent indices
    let mut queue = VecDeque::new();
    queue.push_back((root.to_path_buf(), &mut root_entry));

    while let Some((current_path, current_entry)) = queue.pop_front() {
        let current_depth = current_path.components().count();
        if current_depth > max_depth {
            continue;
        }

        let mut current_children = Vec::new();

        for dir_entry in fs::read_dir(&current_path)? {
            let dir_entry = dir_entry?;
            let path = dir_entry.path();
            let metadata = dir_entry.metadata()?;
            let name = dir_entry.file_name().to_string_lossy().to_string();

            let child = DirectoryEntry {
                path: path.clone(),
                name,
                is_dir: metadata.is_dir(),
                metadata: EntryMetadata {
                    size: metadata.len(),
                    created: metadata.created()?,
                    modified: metadata.modified()?,
                    files_count: 0,
                },
                children: Vec::new(),
                is_gitignored: gitignore.is_ignored(&path),
            };

            if metadata.is_dir() {
                // Recursively scan subdirectories
                if let Ok(scanned_dir) =
                    scan_directory(&path, gitignore, max_depth.saturating_sub(1))
                {
                    current_entry.metadata.files_count += scanned_dir.metadata.files_count;
                    current_entry.metadata.size += scanned_dir.metadata.size;
                    current_children.push(scanned_dir);
                }
            } else {
                current_entry.metadata.files_count += 1;
                current_entry.metadata.size += metadata.len();
                current_children.push(child);
            }
        }

        current_entry.children = current_children;
    }

    Ok(root_entry)
}
