use crate::gitignore::GitIgnoreContext;
use crate::rules::{FilterContext, FilterRegistry};
use crate::types::{DirectoryEntry, EntryMetadata};
use anyhow::Result;
use log::{debug, warn};
use std::fs;
use std::path::Path;

pub fn scan_directory(
    root: &Path,
    gitignore_ctx: &mut GitIgnoreContext,
    rule_registry: Option<&FilterRegistry>,
    max_depth: usize,
    show_system_dirs: Option<bool>,
    show_filtered: Option<bool>,
) -> Result<DirectoryEntry> {
    // Default settings
    let show_system = show_system_dirs.unwrap_or(false);
    let show_hidden = show_filtered.unwrap_or(false);

    let root_metadata = fs::metadata(root)?;
    let root_name = root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| root.to_string_lossy().to_string());

    // Process this directory to load any .gitignore file before checking ignore status
    if let Err(e) = gitignore_ctx.process_directory(root) {
        warn!("Error processing gitignore in {}: {}", root.display(), e);
    }

    // Get parent path for context
    let parent_path = root.parent().unwrap_or(root);

    // Check filtering rules if provided
    let is_gitignored = gitignore_ctx.is_ignored(root);
    let mut filtered_by = None;
    let mut filter_annotation = None;

    // Apply rules if registry is provided
    if let Some(registry) = rule_registry {
        // Create context for this path
        let mut context = FilterContext::new(
            root,
            parent_path,
            root, // Using root as project root for now
            0,    // Depth will be set correctly in recursive calls
        );

        // Detect project types
        context.detect_project_types();

        // Evaluate rules
        if let Some((_, annotation)) = registry.should_hide(&context) {
            filtered_by = Some(String::from("rule")); // Would ideally track specific rule ID
            filter_annotation = Some(String::from(annotation));
        }
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
            is_gitignored,
            filtered_by,
            filter_annotation,
        });
    }

    // Check if this entry should be filtered based on rules
    let should_filter = (is_gitignored && !show_system) || (filtered_by.is_some() && !show_hidden);

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
        is_gitignored,
        filtered_by,
        filter_annotation,
    };

    // For filtered directories, decide whether to traverse or just provide basic metadata
    // If this is the root path that was explicitly specified, never skip it regardless of filter rules
    let is_direct_path = root.canonicalize().unwrap_or_else(|_| root.to_path_buf())
        == Path::new(&root).canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let should_skip = should_filter && !is_direct_path;

    if should_skip {
        debug!(
            "Skipping deep traversal of filtered directory: {}",
            root.display()
        );
        // Do a quick scan to get file counts without deep traversal
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
    // If we're showing filtered directories, we'll continue with the normal traversal

    let mut entries = Vec::new();

    // Read the directory and process entries
    for dir_entry in fs::read_dir(root)? {
        let dir_entry = dir_entry?;
        let path = dir_entry.path();
        let metadata = dir_entry.metadata()?;
        let name = dir_entry.file_name().to_string_lossy().to_string();

        // Check if this specific entry is gitignored
        let is_gitignored = gitignore_ctx.is_ignored(&path);

        // Apply filtering rules if available
        let mut filtered_by = None;
        let mut filter_annotation = None;

        if let Some(registry) = rule_registry {
            // Create context for this path
            let mut context = FilterContext::new(
                &path, root, root,      // Using root as project root
                max_depth, // Current depth level
            );

            // Detect project types
            context.detect_project_types();

            // Evaluate rules
            if let Some((_, annotation)) = registry.should_hide(&context) {
                filtered_by = Some(String::from("rule"));
                filter_annotation = Some(String::from(annotation));
            }
        }

        if metadata.is_dir() {
            // Recursively scan subdirectories if depth allows
            if max_depth > 1 {
                match scan_directory(
                    &path,
                    gitignore_ctx,
                    rule_registry,
                    max_depth - 1,
                    Some(show_system),
                    Some(show_hidden),
                ) {
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
                    filtered_by,
                    filter_annotation,
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
                filtered_by,
                filter_annotation,
            });
        }
    }

    // Set the children
    root_entry.children = entries;

    Ok(root_entry)
}
