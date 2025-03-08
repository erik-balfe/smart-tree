# Smart Tree Development Plan

## Current Project Status

Smart Tree is a modern directory tree viewer that intelligently displays file hierarchies with an emphasis on readability. Unlike traditional `tree` commands that output everything (often producing thousands of lines), Smart Tree makes smart decisions about what to show and what to fold.

### Implemented Features

- **Core Scanning Engine**: Traverses directories and collects metadata
- **Intelligent Folding**: Automatically folds known system directories (.git, node_modules)
- **Line Budget Management**: Respects terminal limitations while preserving context
- **Head/Tail Display Pattern**: Shows both beginning and end of large listings
- **Sorting Options**: By name, size, modified time, creation time
- **GitIgnore Integration**: Basic folding based on gitignore patterns
- **Metadata Display**: File sizes, modification times, directory statistics
- **CLI Interface**: Configurable options via command line arguments
- **Color and Emoji Support**: Rich visual enhancements for improved readability

### Known Issues

- ✅ ~~Gitignore pattern matching could be further improved~~
- ✅ ~~Performance concerns for large directory structures~~
- ✅ ~~Full directory tree kept in memory (could be problematic for large trees)~~
- ✅ ~~Limited terminal width consideration~~
- ✅ ~~T-shape vs L-shape connectors display incorrectly for the last items~~
- ✅ ~~Program collapses lines where only 1 item is hidden (inefficient)~~
- Contents of .git (and other system directories) cannot be viewed even when explicitly requested
- Does not properly respect nested .gitignore files in subdirectories
- No option to explicitly show system directories that are normally hidden

## Development Roadmap

### Critical Fixes (Immediate Priority)

1. ✅ **Fix System Directory Exclusion Logic**
   - ✅ Add a flag to explicitly show contents of system directories
   - ✅ Modify `display/state.rs` to respect this flag when rendering directories
   - ✅ Update state logic to process gitignored directories when explicitly requested

2. ✅ **Improve .gitignore Detection and Handling**
   - ✅ Implement recursive .gitignore detection to respect nested .gitignore files
   - ✅ Load and apply .gitignore rules at each directory level
   - ✅ Cache gitignore patterns to avoid redundant file reads
   - ✅ Add option to toggle .gitignore consideration

3. **Implement Smart Filtering Rules System**
   - Integrate the rule-based filtering architecture
   - Convert gitignore handling to use the rule system
   - Implement project type detection
   - Create specialized rules for different project types and contexts
   - Add configuration for enabling/disabling rules

4. **Explicit Directory Inclusion Options**
   - Add command-line parameter to explicitly include directories that would otherwise be folded
   - Support glob patterns for directory inclusion/exclusion
   - Fix logic handling of included directories to properly scan and display their contents

### Phase 1: Code Cleanup and Bug Fixes (Completed) ✅

1. **Fix Test Suite Errors** ✅
   - Remove duplicated test_utils module and ExpectedContent struct
   - Fix duplicated test functions
   - Ensure tests pass with `cargo test`

2. **Fix Code Quality Issues** ✅
   - Address all compiler warnings
   - Apply consistent formatting with `cargo fmt`
   - Run clippy and fix linting issues

3. **Improve Error Handling** ✅
   - Review use of unwrap/expect and replace with proper error handling
   - Improve error messages and recovery

### Phase 2: Core Functionality Enhancement (Current)

1. **Improved GitIgnore Integration** 🚧
   - ✅ Implement proper gitignore pattern matching
   - ✅ Support standard gitignore syntax with globs
   - Add proper recursive .gitignore file detection and pattern application
   - Add option to toggle .gitignore consideration

2. **Memory Optimization** ✅
   - Implement streaming/lazy loading approach for large directories
   - Reduce memory usage for very large directory structures

3. **Performance Improvements** 🚧
   - Optimize directory traversal for large directory structures
   - Implement metadata caching for frequently accessed directories
   - Consider parallel scanning for improved performance

### Phase 3: UI and UX Enhancements (In Progress)

1. **Terminal Integration**
   - Add terminal width consideration for improved display
   - ✅ Implement color support for different file types and metadata
     - ✅ Use different colors for directories, files, symlinks
     - ✅ Highlight gitignored entries in a distinct color
     - ✅ Color code metadata based on file type and size
     - ✅ Add config option to enable/disable colors
     - ✅ Support for light/dark themes
     - ✅ Fix tree connector characters for consistent rendering
   - Add interactive navigation features

2. **Advanced Display Options**
   - ✅ Implement customizable display templates
   - ✅ Add file type recognition and specialized displays
   - ✅ Support different view modes (compact, detailed)
     - ✅ Add detailed metadata display option
     - ✅ Add size-based color gradients
     - ✅ Add date-based color gradients

3. **Visual Enhancements**
   - ✅ Implement icons for file types (emoji support)
   - Add progress indicators for scanning large directories
   - ✅ Improve the visual hierarchy indicators
   
4. **Usability Improvements**
   - Implement shorter command-line aliases (e.g., -n for --max-lines)
   - Evaluate program name and consider alternatives (e.g., "st" for Smart Tree)
   - Improve help messages with clearer examples and option descriptions
   - Add bash/zsh completions for improved user experience

### Phase 4: Distribution and Integration (Medium Term)

1. **Package and Distribution**
   - Create installation script (install.sh) for simple cross-platform installation
   - Package for Homebrew (brew) installation on macOS
   - Implement continuous integration/deployment pipeline
   - Set up automated GitHub releases with versioned branches (e.g., 'v0.2.0')

2. **Integration Features**
   - VCS integration (show modified files since last commit)
   - Build system integration for project-specific views
   - Configuration file support for project defaults
   - Add support for config file in multiple formats (.json, .toml)

### Phase 5: Advanced Filtering and Extensibility

1. **Context-Aware Smart Filtering System**
   - Implement a plugin-based filtering system for intelligent directory/file visibility rules
   - Detect project types and apply specialized filtering rules (Rust, JavaScript, Python, etc.)
   - Create a rule registry with context detection and conditional application
   - Support weighted rule scoring for complex decision making
   
2. **Smart Filtering Architecture**
   ```rust
   // Rule evaluation context containing project and environmental information
   pub struct FilterContext<'a> {
       project_types: Vec<ProjectType>,         // Detected project type(s)
       path: &'a Path,                          // Path being evaluated
       parent_path: &'a Path,                   // Parent directory
       depth: usize,                            // Depth in directory tree
       has_file: HashMap<String, bool>,         // Cache of file existence tests
       extension_counts: HashMap<String, usize>, // Statistics on file types
       root_path: &'a Path,                     // Root directory being scanned
       // Additional context that rules might need
   }
   
   // Filter rule trait for extensibility
   pub trait FilterRule: Send + Sync {
       // Unique identifier for the rule
       fn id(&self) -> &str;
       
       // Rule priority (higher numbers = higher priority)
       fn priority(&self) -> i32;
       
       // Whether this rule applies to the given context
       fn applies_to(&self, context: &FilterContext) -> bool;
       
       // Evaluate the rule, returning a score between 0.0 and 1.0
       // Higher scores indicate higher confidence in hiding
       fn evaluate(&self, context: &FilterContext) -> f32;
       
       // Custom display annotation for when this rule triggers
       fn annotation(&self) -> &str { "[filtered]" }
   }
   
   // Registry for all filter rules
   pub struct FilterRegistry {
       rules: Vec<Box<dyn FilterRule>>,
       threshold: f32,                       // Minimum score to hide an item
   }
   ```

3. **Built-in Smart Filter Rules**
   - **GitIgnoreRule**: Apply .gitignore rules in a context-aware manner
     - Load nested .gitignore files at each directory level
     - Respect negation patterns (!pattern)
     - Support all standard gitignore syntax
   - **BuildOutputRule**: Detect and hide build output directories by project type
     - `target/` in Rust projects (detected by Cargo.toml)
     - `dist/`, `build/` in JS/TS projects (detected by package.json)
     - `__pycache__/`, `*.pyc` in Python projects
     - `bin/`, `obj/` in .NET projects
   - **DependencyRule**: Hide dependency directories by project type
     - `node_modules/` in JavaScript projects
     - `.venv/`, `env/` in Python projects
     - `vendor/` in Go projects
   - **VCSRule**: Hide version control system directories
     - `.git/` for Git repositories
     - `.svn/` for Subversion repositories
     - `.hg/` for Mercurial repositories  
     - `.jj/` for Jujutsu repositories
   - **DevEnvironmentRule**: Hide editor/IDE configs (.vscode, .idea, .zed)
   - **GeneratedCodeRule**: Hide auto-generated code based on markers or patterns
   - **TestDataRule**: Conditionally hide large test data directories
   - **TemporaryFileRule**: Hide temporary files like backups, logs, and cache files
   - **DocumentationRule**: Context-aware handling of documentation directories and files

4. **User Configuration for Filtering**
   - Support custom rule definitions in `.smart-tree.toml`
   - Project-level configuration with inheritance
   - Global user configuration in home directory
   - Allow enabling/disabling/configuring specific rules
   - Support custom scoring thresholds
   - Provide detailed debugging for rule application
   - Example configuration:
     ```toml
     # .smart-tree.toml
     
     # Enable or disable all rules (true by default)
     enabled = true
     
     # Global threshold (0.0-1.0)
     threshold = 0.5
     
     # Configure specific rules
     [rules.vcs]
     enabled = true
     annotation = "[version control]"
     
     [rules.build_output]
     enabled = true
     annotation = "[build]"
     
     # Custom rule definition
     [rules.custom]
     patterns = ["*.tmp", "*.cache"]
     annotation = "[temp files]"
     priority = 50
     ```

5. **Other Advanced Filtering**
   - Add time-based filtering (recently modified, older than X)
   - Support regex-based content filtering

### Phase 6: Extended Features

1. **Export Capabilities**
   - JSON/XML export of directory structure
   - HTML visualization export
   - Integration with other tools via standardized output formats

2. **Advanced Integration**
   - Add plugin system for custom extensions
   - Implement extensions for popular development tools
   - Add language-specific statistics gathering

## Implementation Details for Upcoming Work

### Smart Filtering Rules Implementation Plan

1. **Integrate Rules System with Existing Scanner**
   ```rust
   // In scanner.rs
   pub fn scan_directory(
       root: &Path,
       rule_registry: &mut FilterRegistry,
       max_depth: usize,
       // Other parameters...
   ) -> Result<DirectoryEntry> {
       // Create context for this path
       let mut context = FilterContext::new(path, parent_path, root_path, depth);
       
       // Detect project types
       context.detect_project_types();
       
       // Check if this path should be hidden based on rules
       let (should_hide, annotation) = rule_registry.should_hide(&context);
       
       if should_hide && !show_hidden {
           // Create a collapsed representation
           // ...
           return Ok(collapsed_entry);
       }
       
       // Regular directory traversal
       // ...
   }
   ```

2. **Convert GitIgnore to Rule System**
   ```rust
   // GitIgnore rule implementation
   pub struct GitIgnoreRule {
       contexts: HashMap<PathBuf, GitIgnoreContext>,
   }
   
   impl FilterRule for GitIgnoreRule {
       fn id(&self) -> &str {
           "gitignore"
       }
       
       fn priority(&self) -> i32 {
           100 // High priority
       }
       
       fn applies_to(&self, context: &FilterContext) -> bool {
           true // Always check gitignore rules
       }
       
       fn evaluate(&self, context: &FilterContext) -> f32 {
           let path = context.path;
           
           // Get or create GitIgnoreContext for this directory
           let gitignore_context = self.get_context_for_path(path);
           
           // Check if path is ignored
           if gitignore_context.is_ignored(path) {
               0.95 // High confidence
           } else {
               0.0 // Not ignored
           }
       }
       
       fn annotation(&self) -> &str {
           "[gitignored]"
       }
   }
   ```

3. **Project Type Detection**
   ```rust
   // In rules.rs
   impl<'a> FilterContext<'a> {
       pub fn detect_project_types(&mut self) {
           // Check for Rust project
           if self.root_path.join("Cargo.toml").exists() {
               self.project_types.push(ProjectType::Rust);
           }
           
           // Check for Node.js project
           if self.root_path.join("package.json").exists() {
               self.project_types.push(ProjectType::NodeJs);
           }
           
           // Check for Python project
           if self.root_path.join("pyproject.toml").exists() || 
              self.root_path.join("setup.py").exists() {
               self.project_types.push(ProjectType::Python);
           }
           
           // Additional project types...
       }
   }
   ```

4. **Rule Configuration System**
   ```rust
   // Configuration structs
   pub struct RuleConfig {
       enabled: bool,
       threshold: Option<f32>,
       annotation: Option<String>,
       // Rule-specific options
       options: HashMap<String, toml::Value>,
   }
   
   pub struct FilterConfig {
       enabled: bool,
       threshold: f32,
       rules: HashMap<String, RuleConfig>,
   }
   
   // Loading config
   impl FilterConfig {
       pub fn load() -> Self {
           // Load from .smart-tree.toml in current dir
           let project_config = Self::load_from_file(Path::new(".smart-tree.toml"));
           
           // Load from user home
           let home_config = Self::load_from_home();
           
           // Merge configs with project taking precedence
           Self::merge(home_config, project_config)
       }
   }
   ```

5. **CLI Interface Updates**
   ```
   smart-tree [OPTIONS] [PATH]
   
   Options:
     --show-hidden            Show items that would normally be hidden by rules
     --rule-debug             Show detailed information about rule application
     --disable-rule <RULE>    Disable specific rule (can be used multiple times)
     --enable-rule <RULE>     Enable specific rule (can be used multiple times)
     --list-rules             List all available rules
   ```

## Test Strategy

1. **Unit Tests for Rules System**
   - Test individual rule implementations
   - Test rule registry and priority system
   - Test project type detection for different project structures
   - Test rule configuration loading from TOML files
   
2. **Unit Tests for GitIgnore Rules**
   - Test recursive .gitignore file loading
   - Test pattern application from multiple .gitignore files
   - Test precedence rules between parent and child patterns

3. **Integration Tests for Project-Specific Rules**
   - Create test projects for each supported language/framework 
   - Test detection of project types and correct rule application
   - Test specific filtering of build directories based on project type

4. **Integration Tests with Nested Repositories**
   - Set up test directories with nested git repositories
   - Verify correct traversal behavior with and without explicit inclusion
   - Test with `--show-system-dirs` and `--show-hidden` flags

5. **End-to-End Testing**
   - Create a test suite that verifies real-world project scenarios
   - Test performance with large codebases
   - Benchmark rule evaluation performance

6. **Configuration Tests**
   - Test loading and merging of different configuration files
   - Test precedence of CLI options over configuration files
   - Test user configuration in home directory

## Implementation Notes

- Maintain modular architecture with clear separation of concerns
- Write tests for all new functionality
- Focus on backwards compatibility for existing command-line options
- Prioritize performance and memory efficiency throughout implementation
- Ensure cross-platform compatibility (Linux, macOS, Windows)

## Documentation Plan

1. Complete missing README.md sections
   - Key Features section
   - Complete Design Philosophy section
   - Development setup instructions

2. Create developer documentation
   - Architecture overview with diagrams
   - Key algorithms explanation
   - Performance considerations

3. Build comprehensive usage documentation
   - Command-line option reference
   - Advanced usage examples
   - Integration with other tools