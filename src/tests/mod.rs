//! Integration and system tests for Smart Tree
//! This module contains comprehensive tests that create real directory structures
//! and run the application against them.

use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// A utility struct for building test file structures
pub struct TestFileBuilder {
    /// The root directory for this test
    pub root_dir: TempDir,
    /// Track created files for verification
    pub created_files: Vec<PathBuf>,
    /// Track created directories for verification
    pub created_dirs: Vec<PathBuf>,
}

impl TestFileBuilder {
    /// Create a new test file builder with a temporary root directory
    pub fn new() -> Self {
        let root_dir = tempfile::tempdir().expect("Failed to create temp directory");
        Self {
            root_dir,
            created_files: Vec::new(),
            created_dirs: Vec::new(),
        }
    }

    /// Get the root path
    pub fn root_path(&self) -> &Path {
        self.root_dir.path()
    }

    /// Create a directory at the given path relative to the root
    pub fn create_dir(&mut self, rel_path: &str) -> &mut Self {
        let path = self.root_dir.path().join(rel_path);
        
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent directory");
        }
        
        fs::create_dir_all(&path).expect("Failed to create directory");
        self.created_dirs.push(path);
        self
    }

    /// Create a file with the given content at the given path relative to the root
    pub fn create_file(&mut self, rel_path: &str, content: &str) -> &mut Self {
        let path = self.root_dir.path().join(rel_path);
        
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent directory");
            // Add parent to created_dirs if not already present
            if !self.created_dirs.contains(&parent.to_path_buf()) {
                self.created_dirs.push(parent.to_path_buf());
            }
        }
        
        let mut file = File::create(&path).expect("Failed to create file");
        file.write_all(content.as_bytes()).expect("Failed to write file content");
        self.created_files.push(path);
        self
    }

    /// Create a .gitignore file with the given patterns at the given path relative to the root
    pub fn create_gitignore(&mut self, rel_dir: &str, patterns: &[&str]) -> &mut Self {
        let content = patterns.join("\n");
        let gitignore_path = if rel_dir.is_empty() { ".gitignore".to_string() } else { format!("{rel_dir}/.gitignore") };
        self.create_file(&gitignore_path, &content)
    }

    /// Create a git-like directory structure (to test system directory handling)
    pub fn create_git_dir(&mut self, rel_path: &str) -> &mut Self {
        // Create basic .git structure
        let git_path = if rel_path.is_empty() { ".git".to_string() } else { format!("{rel_path}/.git") };
        
        self.create_dir(&git_path)
            .create_dir(&format!("{}/objects", git_path))
            .create_dir(&format!("{}/refs", git_path))
            .create_file(&format!("{}/HEAD", git_path), "ref: refs/heads/main\n")
            .create_file(&format!("{}/config", git_path), "[core]\n\trepositoryformatversion = 0\n\tfilemode = true\n")
    }

    /// Create a node_modules-like directory with many files
    pub fn create_node_modules(&mut self, rel_path: &str) -> &mut Self {
        let node_modules_path = if rel_path.is_empty() { 
            "node_modules".to_string() 
        } else { 
            format!("{rel_path}/node_modules") 
        };
        
        self.create_dir(&node_modules_path)
            .create_dir(&format!("{}/lodash", node_modules_path))
            .create_file(&format!("{}/lodash/package.json", node_modules_path), "{}")
            .create_dir(&format!("{}/react", node_modules_path))
            .create_file(&format!("{}/react/package.json", node_modules_path), "{}")
    }

    /// Create a nested project structure with multiple .gitignore files
    pub fn create_nested_project(&mut self) -> &mut Self {
        // Root project
        self.create_file("README.md", "# Root Project")
            .create_file("package.json", "{}")
            .create_gitignore("", &["*.log", "dist/", "build/"])
            .create_git_dir("")
            .create_node_modules("")
            
            // Main source code
            .create_dir("src")
            .create_file("src/main.js", "console.log('Hello');")
            .create_file("src/index.js", "import './main.js';")
            
            // Nested project with its own .gitignore
            .create_dir("projects/webapp")
            .create_file("projects/webapp/README.md", "# Web App")
            .create_gitignore("projects/webapp", &["*.tmp", "node_modules/"])
            .create_git_dir("projects/webapp")
            .create_node_modules("projects/webapp")
            .create_file("projects/webapp/app.js", "// Main app")
            
            // Another nested project
            .create_dir("projects/api")
            .create_file("projects/api/README.md", "# API")
            .create_gitignore("projects/api", &["*.bak", "logs/"])
            .create_git_dir("projects/api")
            .create_file("projects/api/server.js", "// API server")
            
            // Create some log files that should be ignored
            .create_file("error.log", "Error log content")
            .create_file("projects/webapp/debug.tmp", "Temp file")
            .create_dir("projects/api/logs")
            .create_file("projects/api/logs/api.log", "API log content")
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::gitignore::GitIgnore;
    use crate::scanner::scan_directory;
    use crate::types::{DisplayConfig, SortBy, ColorTheme};
    use crate::format_tree;

    /// Test for correctly marking system directories as gitignored
    #[test]
    fn test_system_dir_marking() {
        let mut builder = TestFileBuilder::new();
        builder.create_nested_project();
        
        let root_path = builder.root_path();
        let gitignore = GitIgnore::load(root_path).unwrap();
        
        let root = scan_directory(root_path, &gitignore, usize::MAX).unwrap();
        
        // Find .git directory in the scanned result
        let git_dir = root.children.iter()
            .find(|c| c.name == ".git")
            .expect(".git directory should be in the result");
            
        // Check that it's marked as gitignored
        assert!(git_dir.is_gitignored, ".git should be marked as gitignored");
        
        // Check that node_modules is also marked
        let node_modules = root.children.iter()
            .find(|c| c.name == "node_modules")
            .expect("node_modules directory should be in the result");
            
        assert!(node_modules.is_gitignored, "node_modules should be marked as gitignored");
    }

    /// Test for correctly handling gitignore patterns
    #[test]
    fn test_gitignore_patterns() {
        let mut builder = TestFileBuilder::new();
        builder.create_nested_project();
        
        let root_path = builder.root_path();
        let gitignore = GitIgnore::load(root_path).unwrap();
        
        // Test root gitignore patterns
        assert!(gitignore.is_ignored(&root_path.join("error.log")), "*.log should be ignored");
        assert!(!gitignore.is_ignored(&root_path.join("README.md")), "README.md should not be ignored");
        
        // Note: In the current implementation, nested .gitignore files are not loaded
        // This test verifies current behavior - will need to be updated once we implement
        // recursive gitignore handling
        assert!(!gitignore.is_ignored(&root_path.join("projects/webapp/debug.tmp")), 
            "Currently nested .gitignore files are not loaded, so .tmp files are not ignored");
    }

    /// Test for the folding of single items
    #[test]
    fn test_no_collapse_single_item() {
        let mut builder = TestFileBuilder::new();
        
        // Create a directory with 3 files, where we'll hide one of them
        builder.create_dir("test_dir")
            .create_file("test_dir/file1.txt", "File 1")
            .create_file("test_dir/file2.txt", "File 2")
            .create_file("test_dir/file3.txt", "File 3");
        
        let root_path = builder.root_path().join("test_dir");
        let gitignore = GitIgnore::load(&root_path).unwrap();
        let root = scan_directory(&root_path, &gitignore, usize::MAX).unwrap();
        
        // Configure to only show 2 items in directory (2 lines + collapsed indicator)
        let config = DisplayConfig {
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
        };
        
        let output = format_tree(&root, &config).unwrap();
        
        // We should NOT see a "1 item hidden" message, since it doesn't save space
        assert!(!output.contains("1 item hidden"), 
            "Should not show '1 item hidden' message since it doesn't save space");
        
        // Now create a directory with 4 files to verify proper collapse of multiple items
        let mut builder = TestFileBuilder::new();
        builder.create_dir("test_dir2")
            .create_file("test_dir2/file1.txt", "File 1")
            .create_file("test_dir2/file2.txt", "File 2")
            .create_file("test_dir2/file3.txt", "File 3")
            .create_file("test_dir2/file4.txt", "File 4");
        
        let root_path = builder.root_path().join("test_dir2");
        let gitignore = GitIgnore::load(&root_path).unwrap();
        let root = scan_directory(&root_path, &gitignore, usize::MAX).unwrap();
        
        let output = format_tree(&root, &config).unwrap();
        
        // We SHOULD see an "items hidden" message
        assert!(output.contains("items hidden"), 
            "Should show 'items hidden' message when multiple items are hidden");
    }

    /// Test for tree connector shapes
    #[test]
    fn test_tree_connectors() {
        let mut builder = TestFileBuilder::new();
        
        // Create a simple directory structure
        builder.create_dir("test_dir")
            .create_file("test_dir/file1.txt", "File 1")
            .create_file("test_dir/file2.txt", "File 2")
            .create_file("test_dir/file3.txt", "File 3");
        
        let root_path = builder.root_path().join("test_dir");
        let gitignore = GitIgnore::load(&root_path).unwrap();
        let root = scan_directory(&root_path, &gitignore, usize::MAX).unwrap();
        
        let config = DisplayConfig {
            max_lines: 10,
            dir_limit: 10,
            sort_by: SortBy::Name,
            dirs_first: false,
            use_colors: false,
            color_theme: ColorTheme::None,
            use_emoji: false,
            size_colorize: false,
            date_colorize: false,
            detailed_metadata: false,
        };
        
        let output = format_tree(&root, &config).unwrap();
        
        // Check that the last file uses L-shape connector
        let lines: Vec<_> = output.lines().collect();
        
        // Find the line with file3.txt
        let last_file_line = lines.iter()
            .find(|l| l.contains("file3.txt"))
            .expect("Should find file3.txt in output");
        
        // Check that it has the corner (L-shape) connector
        assert!(
            last_file_line.contains("└──"),
            "Last item should use L-shape connector: {}",
            last_file_line
        );
        
        // Find the line with file2.txt (middle item)
        let middle_file_line = lines.iter()
            .find(|l| l.contains("file2.txt"))
            .expect("Should find file2.txt in output");
        
        // Check that it has the T-shape connector
        assert!(
            middle_file_line.contains("├──"),
            "Middle item should use T-shape connector: {}",
            middle_file_line
        );
    }
}