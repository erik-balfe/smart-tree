use super::state::DisplayState;
use crate::types::{ColorTheme, DirectoryEntry, DisplayConfig, EntryMetadata, SortBy};
use std::path::PathBuf;
use std::time::SystemTime;

// Test utilities
mod test_utils {
    use super::*;

    pub fn create_test_entry(
        name: &str,
        is_dir: bool,
        children: Vec<DirectoryEntry>,
    ) -> DirectoryEntry {
        DirectoryEntry {
            path: PathBuf::from(name),
            name: name.to_string(),
            is_dir,
            metadata: EntryMetadata {
                size: 100,
                created: SystemTime::now(),
                modified: SystemTime::now(),
                files_count: if is_dir { children.len() } else { 0 },
            },
            children,
            is_gitignored: false,
        }
    }

    pub fn extract_directory_content(output: &str, dir_name: &str) -> String {
        output
            .lines()
            .skip_while(|l| !l.contains(dir_name))
            .take_while(|l| l.starts_with("│   ") || l.starts_with("├──") || l.starts_with("└──"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug)]
struct ExpectedContent {
    should_show_src: bool,
    should_show_src_contents: bool,
    min_visible_items: usize,
    should_show_head_tail: bool,
}

#[test]
fn test_basic_line_limit() {
    use test_utils::*;

    let files = (1..20)
        .map(|i| create_test_entry(&format!("file{}.rs", i), false, vec![]))
        .collect::<Vec<_>>();

    for max_lines in [3, 5, 7, 10] {
        let config = DisplayConfig {
            max_lines,
            dir_limit: 20,
            sort_by: SortBy::Name,
            dirs_first: false,
            use_colors: false,
            color_theme: ColorTheme::None,
            use_emoji: false,
            size_colorize: false,
            date_colorize: false,
            detailed_metadata: false,
        show_system_dirs: false,
        };

        let mut state = DisplayState::new(max_lines, &config);
        state.show_items(&files, "");

        let line_count = state.output.lines().count();
        assert!(
            line_count <= max_lines,
            "Output exceeded max_lines ({}) with {} lines:\n{}",
            max_lines,
            line_count,
            state.output
        );
    }
}

#[test]
fn test_head_tail_pattern() {
    use test_utils::*;

    let files = (1..10)
        .map(|i| create_test_entry(&format!("file{}.rs", i), false, vec![]))
        .collect::<Vec<_>>();

    let config = DisplayConfig {
        max_lines: 7,
        dir_limit: 20,
        sort_by: SortBy::Name,
        dirs_first: false,
        use_colors: false,
        color_theme: ColorTheme::None,
        use_emoji: false,
        size_colorize: false,
        date_colorize: false,
        detailed_metadata: false,
        show_system_dirs: false,
    };

    let mut state = DisplayState::new(config.max_lines, &config);
    state.show_items(&files, "");

    println!("Output:\n{}", state.output);

    let visible_lines: Vec<_> = state
        .output
        .lines()
        .filter(|l| !l.contains("items hidden"))
        .collect();

    // Should show at least first and last items
    assert!(
        visible_lines.iter().any(|l| l.contains("file1.rs")),
        "Should show first file"
    );
    assert!(
        visible_lines.iter().any(|l| l.contains("file9.rs")),
        "Should show last file"
    );

    // Should show hidden items indicator
    assert!(
        state.output.contains("items hidden"),
        "Should indicate hidden items"
    );
}

#[test]
fn test_nested_directory_budget() {
    use test_utils::*;

    let nested_files = (1..5)
        .map(|i| create_test_entry(&format!("nested{}.rs", i), false, vec![]))
        .collect::<Vec<_>>();

    let dirs = vec![
        create_test_entry("src", true, nested_files.clone()),
        create_test_entry("test", true, nested_files),
    ];

    let config = DisplayConfig {
        max_lines: 10,
        dir_limit: 20,
        sort_by: SortBy::Name,
        dirs_first: false,
        use_colors: false,
        color_theme: ColorTheme::None,
        use_emoji: false,
        size_colorize: false,
        date_colorize: false,
        detailed_metadata: false,
        show_system_dirs: false,
    };

    let mut state = DisplayState::new(config.max_lines, &config);
    state.show_items(&dirs, "");

    println!("Output:\n{}", state.output);

    // Check line limit
    assert!(
        state.output.lines().count() <= config.max_lines,
        "Should respect max_lines limit"
    );

    // Each shown directory should have visible content
    for dir in ["src", "test"] {
        if state.output.contains(dir) {
            let dir_content = extract_directory_content(&state.output, dir);
            assert!(
                !dir_content.is_empty(),
                "Directory {} should show some content",
                dir
            );
        }
    }
}

#[test]
fn test_real_project_structure() {
    use test_utils::*;

    // Create actual project structure
    let display_dir_contents = vec![
        create_test_entry("mod.rs", false, vec![]),
        create_test_entry("format.rs", false, vec![]),
        create_test_entry("state.rs", false, vec![]),
        create_test_entry("tests.rs", false, vec![]),
        create_test_entry("utils.rs", false, vec![]),
    ];

    let src_contents = vec![
        create_test_entry("display", true, display_dir_contents),
        create_test_entry("gitignore.rs", false, vec![]),
        create_test_entry("lib.rs", false, vec![]),
        create_test_entry("main.rs", false, vec![]),
        create_test_entry("scanner.rs", false, vec![]),
        create_test_entry("types.rs", false, vec![]),
    ];

    for max_lines in [10, 15, 20] {
        let config = DisplayConfig {
            max_lines,
            dir_limit: 20,
            sort_by: SortBy::Modified,
            dirs_first: false,
            use_colors: false,
            color_theme: ColorTheme::None,
            use_emoji: false,
            size_colorize: false,
            date_colorize: false,
            detailed_metadata: false,
        show_system_dirs: false,
        };

        let mut state = DisplayState::new(config.max_lines, &config);
        state.show_items(&src_contents, "");

        println!(
            "\nTesting with max_lines = {}:\n{}",
            max_lines, state.output
        );

        // Verify line limit
        let line_count = state.output.lines().count();
        assert!(
            line_count <= max_lines,
            "Output exceeded {} lines (got {})",
            max_lines,
            line_count
        );

        // Verify content visibility
        if max_lines >= 10 {
            assert!(
                state.output.contains("display") || state.output.contains("main.rs"),
                "Should show some top-level content"
            );

            let display_content = extract_directory_content(&state.output, "display");
            if !display_content.is_empty() {
                assert!(
                    display_content.contains("mod.rs") || display_content.contains("utils.rs"),
                    "Should show some display directory content"
                );
            }
        }
    }
}

#[test]
fn test_expanded_project_structure() {
    use test_utils::create_test_entry;

    // Create actual project structure
    let display_dir_contents = vec![
        create_test_entry("mod.rs", false, vec![]),
        create_test_entry("format.rs", false, vec![]),
        create_test_entry("state.rs", false, vec![]),
        create_test_entry("tests.rs", false, vec![]),
        create_test_entry("utils.rs", false, vec![]),
    ];

    let src_contents = vec![
        create_test_entry("display", true, display_dir_contents),
        create_test_entry("gitignore.rs", false, vec![]),
        create_test_entry("lib.rs", false, vec![]),
        create_test_entry("main.rs", false, vec![]),
        create_test_entry("scanner.rs", false, vec![]),
        create_test_entry("types.rs", false, vec![]),
    ];

    let debug_contents = vec![
        create_test_entry("deps", true, vec![]),
        create_test_entry("examples", true, vec![]),
        create_test_entry("incremental", true, vec![]),
        create_test_entry("smart-tree", false, vec![]),
        create_test_entry("smart-tree.d", false, vec![]),
    ];

    let target_contents = vec![
        create_test_entry("debug", true, debug_contents),
        create_test_entry("rust-analyzer", true, vec![]),
    ];

    let root_contents = vec![
        create_test_entry("Cargo.lock", false, vec![]),
        create_test_entry("Cargo.toml", false, vec![]),
        create_test_entry("README.MD", false, vec![]),
        create_test_entry("src", true, src_contents),
        create_test_entry("target", true, target_contents),
    ];

    // Test cases with expected content checks
    let test_cases = vec![
        (
            5,
            ExpectedContent {
                should_show_src: false,
                should_show_src_contents: false,
                min_visible_items: 2,
                should_show_head_tail: false,
            },
        ),
        (
            10,
            ExpectedContent {
                should_show_src: true,
                should_show_src_contents: true,
                min_visible_items: 4,
                should_show_head_tail: true,
            },
        ),
        (
            15,
            ExpectedContent {
                should_show_src: true,
                should_show_src_contents: true,
                min_visible_items: 6,
                should_show_head_tail: true,
            },
        ),
        (
            20,
            ExpectedContent {
                should_show_src: true,
                should_show_src_contents: true,
                min_visible_items: 8,
                should_show_head_tail: true,
            },
        ),
    ];

    for (max_lines, expected) in test_cases {
        println!("\nTesting with max_lines = {}", max_lines);
        println!("Expected: {:?}", expected);

        let config = DisplayConfig {
            max_lines,
            dir_limit: 20,
            sort_by: SortBy::Modified,
            dirs_first: false,
            use_colors: false,
            color_theme: ColorTheme::None,
            use_emoji: false,
            size_colorize: false,
            date_colorize: false,
            detailed_metadata: false,
        show_system_dirs: false,
        };

        let mut state = DisplayState::new(config.max_lines, &config);
        state.show_items(&root_contents, "");

        let output = state.output.clone();
        println!("Output:\n{}", output);

        // Debug section analysis
        println!("\nAnalyzing output structure:");
        let lines: Vec<_> = output.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            println!("Line {}: {}", i, line);
            if line.contains("src") {
                println!("Found src at line {}", i);
                // Print next few lines to see what follows
                for j in 1..3 {
                    if i + j < lines.len() {
                        println!("  Next {}: {}", j, lines[i + j]);
                    }
                }
            }
        }

        // Count visible items (non-hidden lines)
        let visible_items = output
            .lines()
            .filter(|l| !l.contains("items hidden"))
            .count();

        println!("Visible items: {}", visible_items);

        // Basic structure checks based on available space
        if expected.should_show_src {
            assert!(
                output.contains("src"),
                "Should show src directory with {} lines",
                max_lines
            );
        }

        if expected.should_show_src_contents {
            // For src directory
            assert!(
                output.contains("display") || output.contains("main.rs"),
                "Should show some src directory contents with {} lines",
                max_lines
            );

            // Verify head/tail pattern if expected
            if expected.should_show_head_tail {
                let src_section = output
                    .lines()
                    .skip_while(|l| !l.contains("src"))
                    .take_while(|l| {
                        l.starts_with("│   ") || l.starts_with("├──") || l.starts_with("└──")
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                // If we have hidden items, ensure we show at least some visible items
                if src_section.contains("items hidden") {
                    assert!(
                        src_section.matches("│   ├──").count() >= 1 ||
                        src_section.matches("│   └──").count() >= 1,
                        "Should show at least one visible item in directory when items are hidden with {} lines",
                        max_lines
                    );
                }
            }
        }

        // Ensure we're showing minimum number of items
        assert!(
            visible_items >= expected.min_visible_items,
            "Should show at least {} items with {} lines limit, got {}",
            expected.min_visible_items,
            max_lines,
            visible_items
        );

        // Line limit check
        let line_count = output.lines().count();
        assert!(
            line_count <= max_lines,
            "Output should not exceed max_lines. Got {} lines, expected <= {}",
            line_count,
            max_lines
        );
    }
}

#[test]
fn test_extended_head_tail_pattern() {
    use test_utils::create_test_entry;

    // Create a directory with many files
    let many_files: Vec<_> = (1..20)
        .map(|i| create_test_entry(&format!("file{}.rs", i), false, vec![]))
        .collect();

    let root_contents = vec![create_test_entry("src", true, many_files)];

    let config = DisplayConfig {
        max_lines: 10,
        dir_limit: 20,
        sort_by: SortBy::Name,
        dirs_first: false,
        use_colors: false,
        color_theme: ColorTheme::None,
        use_emoji: false,
        size_colorize: false,
        date_colorize: false,
        detailed_metadata: false,
        show_system_dirs: false,
    };

    let mut state = DisplayState::new(config.max_lines, &config);
    state.show_items(&root_contents, "");

    println!("Output:\n{}", state.output);

    let output = state.output;

    println!("\nContent analysis:");
    for line in output.lines() {
        println!("{}", line);
    }

    // Should show directory
    assert!(output.contains("src"), "Should show src directory");

    // Should show some files from beginning
    assert!(output.contains("file1.rs"), "Should show first file");

    // Should show hidden items indicator
    assert!(output.contains("items hidden"), "Should show hidden items");

    // Line limit verification
    assert!(
        output.lines().count() <= config.max_lines,
        "Should not exceed line limit"
    );
}

#[test]
fn test_last_item_connector() {
    use test_utils::create_test_entry;

    // Create a directory with a few files to test last item connector
    let files = vec![
        create_test_entry("file1.rs", false, vec![]),
        create_test_entry("file2.rs", false, vec![]),
        create_test_entry("file3.rs", false, vec![]),
    ];

    let root_contents = vec![create_test_entry("src", true, files)];

    let config = DisplayConfig {
        max_lines: 20,
        dir_limit: 20,
        sort_by: SortBy::Name,
        dirs_first: false,
        use_colors: false,
        color_theme: ColorTheme::None,
        use_emoji: false,
        size_colorize: false,
        date_colorize: false,
        detailed_metadata: false,
        show_system_dirs: false,
    };

    let mut state = DisplayState::new(config.max_lines, &config);
    state.show_items(&root_contents, "");

    let output = state.output;
    println!("Output:\n{}", output);

    // The last file should use L-shape (corner) connector
    let lines: Vec<_> = output.lines().collect();

    // Find the line with file3.rs (which should be the last item)
    let file3_line = lines
        .iter()
        .find(|l| l.contains("file3.rs"))
        .expect("file3.rs should be in output");

    // Check that it has the corner (L-shape) connector
    assert!(
        file3_line.contains("└──"),
        "Last item in directory should use L-shape connector: {}",
        file3_line
    );
}

#[test]
fn test_no_collapse_single_item() {
    use test_utils::create_test_entry;

    // Create a directory with three files to test collapsing behavior
    let files = vec![
        create_test_entry("file1.rs", false, vec![]),
        create_test_entry("file2.rs", false, vec![]), // This will be hidden
        create_test_entry("file3.rs", false, vec![]),
    ];

    let root_contents = vec![create_test_entry("src", true, files)];

    // Configure to only show 2 lines in directory (force 1 item to be hidden)
    let config = DisplayConfig {
        max_lines: 5, // Root + src + 2 files + maybe hidden indicator
        dir_limit: 2, // Only show 2 files in directory
        sort_by: SortBy::Name,
        dirs_first: false,
        use_colors: false,
        color_theme: ColorTheme::None,
        use_emoji: false,
        size_colorize: false,
        date_colorize: false,
        detailed_metadata: false,
        show_system_dirs: false,
    };

    let mut state = DisplayState::new(config.max_lines, &config);
    state.show_items(&root_contents, "");

    let output = state.output;
    println!("Output with 1 item hidden:\n{}", output);

    // We should NOT see a "1 item hidden" message, since it doesn't save space
    assert!(
        !output.contains("1 item hidden"),
        "Should not collapse when only 1 item would be hidden"
    );

    // Now try with 2 items hidden
    let more_files = vec![
        create_test_entry("file1.rs", false, vec![]),
        create_test_entry("file2.rs", false, vec![]), // These will be hidden
        create_test_entry("file3.rs", false, vec![]), // These will be hidden
        create_test_entry("file4.rs", false, vec![]),
    ];

    let more_root_contents = vec![create_test_entry("src", true, more_files)];

    let more_config = DisplayConfig {
        max_lines: 5,
        dir_limit: 2,
        sort_by: SortBy::Name,
        dirs_first: false,
        use_colors: false,
        color_theme: ColorTheme::None,
        use_emoji: false,
        size_colorize: false,
        date_colorize: false,
        detailed_metadata: false,
        show_system_dirs: false,
    };

    let mut more_state = DisplayState::new(more_config.max_lines, &more_config);
    more_state.show_items(&more_root_contents, "");

    let more_output = more_state.output;
    println!("Output with 2 items hidden:\n{}", more_output);

    // We SHOULD see "items hidden" message for multiple items
    assert!(
        more_output.contains("items hidden"),
        "Should collapse when 2 or more items would be hidden"
    );
}
