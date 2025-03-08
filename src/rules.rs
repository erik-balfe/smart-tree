//! Smart filtering rules system
//! 
//! This module defines the plugin architecture for intelligent filtering
//! of directories and files based on detected project context.
//! 
//! # Design Philosophy
//! 
//! The filtering system aims to make smart decisions about what to show
//! and what to hide based on the detected project type, file patterns,
//! and other contextual information. 
//! 
//! Rules are evaluated within a context that includes:
//! - Project type detection (Rust, Node.js, Python, etc.)
//! - Directory structure and patterns
//! - File presence and counts
//! 
//! Each rule returns a score between 0.0 and 1.0, with higher scores
//! indicating higher confidence that a path should be hidden/folded.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Supported project types for specialized filtering
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectType {
    /// Rust project (detected by Cargo.toml)
    Rust,
    /// Node.js project (detected by package.json)
    NodeJs,
    /// Python project (detected by setup.py, pyproject.toml)
    Python,
    /// Java project (detected by pom.xml, build.gradle)
    Java,
    /// Go project (detected by go.mod)
    Go,
    /// Ruby project (detected by Gemfile)
    Ruby,
    /// Generic project (no specific type detected)
    Generic,
}

/// Context provided to filter rules during evaluation
pub struct FilterContext<'a> {
    /// Detected project types for the root directory
    pub project_types: Vec<ProjectType>,
    
    /// Current path being evaluated
    pub path: &'a Path,
    
    /// Parent directory path
    pub parent_path: &'a Path,
    
    /// Directory tree depth from root
    pub depth: usize,
    
    /// Cache of file existence tests (path -> exists)
    pub has_file: HashMap<String, bool>,
    
    /// Statistics on file extensions (ext -> count)
    pub extension_counts: HashMap<String, usize>,
    
    /// Root directory of the project
    pub root_path: &'a Path,
}

impl<'a> FilterContext<'a> {
    /// Create a new filter context
    pub fn new(
        path: &'a Path,
        parent_path: &'a Path,
        root_path: &'a Path,
        depth: usize,
    ) -> Self {
        Self {
            project_types: Vec::new(),
            path,
            parent_path,
            depth,
            has_file: HashMap::new(),
            extension_counts: HashMap::new(),
            root_path,
        }
    }
    
    /// Detect project types for the given path
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
        if self.root_path.join("setup.py").exists() 
            || self.root_path.join("pyproject.toml").exists() {
            self.project_types.push(ProjectType::Python);
        }
        
        // Check for Java project
        if self.root_path.join("pom.xml").exists() 
            || self.root_path.join("build.gradle").exists() {
            self.project_types.push(ProjectType::Java);
        }
        
        // Check for Go project
        if self.root_path.join("go.mod").exists() {
            self.project_types.push(ProjectType::Go);
        }
        
        // Check for Ruby project
        if self.root_path.join("Gemfile").exists() {
            self.project_types.push(ProjectType::Ruby);
        }
        
        // If no specific type detected, mark as generic
        if self.project_types.is_empty() {
            self.project_types.push(ProjectType::Generic);
        }
    }
    
    /// Check if file exists in the current directory
    pub fn has_file_in_dir(&mut self, filename: &str) -> bool {
        let key = filename.to_string();
        
        if let Some(&exists) = self.has_file.get(&key) {
            return exists;
        }
        
        let exists = self.path.join(filename).exists();
        self.has_file.insert(key, exists);
        exists
    }
    
    /// Check if the current directory contains a file matching a pattern
    pub fn has_file_matching(&self, pattern: &str) -> bool {
        // Simple glob-style matching
        use std::fs;
        
        if let Ok(entries) = fs::read_dir(self.path) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if !file_type.is_file() {
                        continue;
                    }
                    
                    if let Some(name) = entry.file_name().to_str() {
                        if glob_match(pattern, name) {
                            return true;
                        }
                    }
                }
            }
        }
        
        false
    }
    
    /// Check if path is a specific project artifact based on project type
    pub fn is_project_artifact(&self, name: &str) -> bool {
        match name {
            "target" => self.project_types.contains(&ProjectType::Rust),
            "node_modules" => self.project_types.contains(&ProjectType::NodeJs),
            "__pycache__" => self.project_types.contains(&ProjectType::Python),
            "build" | "dist" => self.project_types.contains(&ProjectType::NodeJs) || 
                                self.project_types.contains(&ProjectType::Java),
            "venv" | ".venv" => self.project_types.contains(&ProjectType::Python),
            _ => false,
        }
    }
}

/// Very simple glob pattern matching (for basic cases only)
fn glob_match(pattern: &str, name: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    
    if pattern.starts_with('*') && pattern.ends_with('*') {
        let inner = &pattern[1..pattern.len()-1];
        return name.contains(inner);
    }
    
    if pattern.starts_with('*') {
        let suffix = &pattern[1..];
        return name.ends_with(suffix);
    }
    
    if pattern.ends_with('*') {
        let prefix = &pattern[..pattern.len()-1];
        return name.starts_with(prefix);
    }
    
    pattern == name
}

/// Interface for all filter rules
pub trait FilterRule: Send + Sync {
    /// Unique identifier for the rule
    fn id(&self) -> &str;
    
    /// Rule priority (higher numbers = higher priority)
    fn priority(&self) -> i32;
    
    /// Whether this rule applies to the given context
    fn applies_to(&self, context: &FilterContext) -> bool;
    
    /// Evaluate the rule, returning a score between 0.0 and 1.0
    /// Higher scores indicate higher confidence in hiding
    fn evaluate(&self, context: &FilterContext) -> f32;
    
    /// Custom display annotation for when this rule triggers
    fn annotation(&self) -> &str {
        "[filtered]"
    }
}

/// Collection of filter rules with evaluation logic
pub struct FilterRegistry {
    rules: Vec<Box<dyn FilterRule>>,
    threshold: f32,
}

impl FilterRegistry {
    /// Create a new empty registry with default threshold
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            threshold: 0.5, // Default threshold is 0.5
        }
    }
    
    /// Add a rule to the registry
    pub fn add_rule<R: FilterRule + 'static>(&mut self, rule: R) {
        self.rules.push(Box::new(rule));
        // Sort rules by priority (highest first)
        self.rules.sort_by(|a, b| b.priority().cmp(&a.priority()));
    }
    
    /// Set the threshold score for hiding
    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold.clamp(0.0, 1.0);
    }
    
    /// Evaluate if a path should be hidden based on all applicable rules
    pub fn should_hide(&self, context: &FilterContext) -> Option<(bool, &str)> {
        let mut max_score = 0.0;
        let mut annotation = "[filtered]";
        
        for rule in &self.rules {
            if rule.applies_to(context) {
                let score = rule.evaluate(context);
                if score > max_score {
                    max_score = score;
                    annotation = rule.annotation();
                }
            }
        }
        
        if max_score >= self.threshold {
            Some((true, annotation))
        } else {
            None
        }
    }
}

/// Built-in rule for hiding build output directories
pub struct BuildOutputRule;

impl FilterRule for BuildOutputRule {
    fn id(&self) -> &str {
        "build_output"
    }
    
    fn priority(&self) -> i32 {
        100 // High priority
    }
    
    fn applies_to(&self, context: &FilterContext) -> bool {
        let file_name = context.path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
            
        match file_name {
            "target" => context.project_types.contains(&ProjectType::Rust),
            "build" | "dist" => context.project_types.contains(&ProjectType::NodeJs) || 
                                context.project_types.contains(&ProjectType::Java),
            "__pycache__" => context.project_types.contains(&ProjectType::Python),
            _ => false,
        }
    }
    
    fn evaluate(&self, context: &FilterContext) -> f32 {
        // High confidence for build directories
        0.9
    }
    
    fn annotation(&self) -> &str {
        "[build output]"
    }
}

/// Built-in rule for hiding dependency directories
pub struct DependencyRule;

impl FilterRule for DependencyRule {
    fn id(&self) -> &str {
        "dependencies"
    }
    
    fn priority(&self) -> i32 {
        90 // High priority but below build outputs
    }
    
    fn applies_to(&self, context: &FilterContext) -> bool {
        let file_name = context.path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
            
        match file_name {
            "node_modules" => context.project_types.contains(&ProjectType::NodeJs),
            "venv" | ".venv" => context.project_types.contains(&ProjectType::Python),
            _ => false,
        }
    }
    
    fn evaluate(&self, context: &FilterContext) -> f32 {
        // High confidence for dependency directories
        0.95
    }
    
    fn annotation(&self) -> &str {
        "[dependencies]"
    }
}

/// Built-in rule for hiding version control system directories
pub struct VCSRule;

impl FilterRule for VCSRule {
    fn id(&self) -> &str {
        "vcs"
    }
    
    fn priority(&self) -> i32 {
        80
    }
    
    fn applies_to(&self, context: &FilterContext) -> bool {
        let file_name = context.path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
            
        matches!(file_name, ".git" | ".svn" | ".hg" | ".jj")
    }
    
    fn evaluate(&self, context: &FilterContext) -> f32 {
        0.85
    }
    
    fn annotation(&self) -> &str {
        "[vcs]"
    }
}

/// Built-in rule for hiding IDE and editor config directories
pub struct DevEnvironmentRule;

impl FilterRule for DevEnvironmentRule {
    fn id(&self) -> &str {
        "dev_environment"
    }
    
    fn priority(&self) -> i32 {
        70
    }
    
    fn applies_to(&self, context: &FilterContext) -> bool {
        let file_name = context.path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
            
        matches!(file_name, ".vscode" | ".idea" | ".eclipse" | ".zed")
    }
    
    fn evaluate(&self, context: &FilterContext) -> f32 {
        0.8
    }
    
    fn annotation(&self) -> &str {
        "[dev config]"
    }
}

/// Create a registry with all default rules enabled
pub fn create_default_registry() -> FilterRegistry {
    let mut registry = FilterRegistry::new();
    registry.add_rule(BuildOutputRule);
    registry.add_rule(DependencyRule);
    registry.add_rule(VCSRule);
    registry.add_rule(DevEnvironmentRule);
    registry
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    fn test_build_output_rule() {
        let rule = BuildOutputRule;
        let path = PathBuf::from("/project/target");
        let parent = PathBuf::from("/project");
        let root = PathBuf::from("/project");
        
        let mut context = FilterContext::new(&path, &parent, &root, 1);
        context.project_types.push(ProjectType::Rust);
        
        assert!(rule.applies_to(&context));
        assert!(rule.evaluate(&context) > 0.5);
    }
    
    #[test]
    fn test_dependency_rule() {
        let rule = DependencyRule;
        let path = PathBuf::from("/project/node_modules");
        let parent = PathBuf::from("/project");
        let root = PathBuf::from("/project");
        
        let mut context = FilterContext::new(&path, &parent, &root, 1);
        context.project_types.push(ProjectType::NodeJs);
        
        assert!(rule.applies_to(&context));
        assert!(rule.evaluate(&context) > 0.5);
    }
    
    #[test]
    fn test_registry_evaluation() {
        let registry = create_default_registry();
        let path = PathBuf::from("/project/target");
        let parent = PathBuf::from("/project");
        let root = PathBuf::from("/project");
        
        let mut context = FilterContext::new(&path, &parent, &root, 1);
        context.project_types.push(ProjectType::Rust);
        
        let result = registry.should_hide(&context);
        assert!(result.is_some());
        let (should_hide, _) = result.unwrap();
        assert!(should_hide);
    }
}