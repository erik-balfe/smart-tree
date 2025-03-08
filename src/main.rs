use anyhow::Result;
use clap::Parser;
use smart_tree::{format_tree, scan_directory, ColorTheme, DisplayConfig, GitIgnoreContext, SortBy};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Directory path to display
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Maximum number of lines in output
    #[arg(long, default_value_t = 200)]
    max_lines: usize,

    /// Maximum items per directory
    #[arg(long, default_value_t = 20)]
    dir_limit: usize,

    /// Maximum depth to traverse
    #[arg(short = 'L', long, default_value_t = usize::MAX)]
    max_depth: usize,

    /// Sort entries by (name|size|modified|created)
    #[arg(long, default_value = "name")]
    sort_by: String,

    /// List directories before files
    #[arg(long)]
    dirs_first: bool,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,

    /// Color theme (auto|light|dark|none)
    #[arg(long, default_value = "auto")]
    color_theme: String,

    /// Use emoji icons for file types
    #[arg(long)]
    emoji: bool,

    /// Disable emoji icons for file types
    #[arg(long)]
    no_emoji: bool,

    /// Colorize file sizes based on magnitude
    #[arg(long)]
    color_sizes: bool,

    /// Colorize dates based on recency
    #[arg(long)]
    color_dates: bool,

    /// Display detailed metadata for files and directories
    #[arg(long)]
    detailed: bool,
    
    /// Show system directories like .git, node_modules, target, etc.
    #[arg(long)]
    show_system_dirs: bool,
    
    /// Ignore .gitignore files when scanning
    #[arg(long)]
    no_gitignore: bool,
}

fn init_logger() {
    // In debug builds, use "debug" as default level
    // In release builds, disable logging completely
    let default_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "off"
    };

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(default_level))
        .format_timestamp(None)
        .init();
}

fn main() -> Result<()> {
    init_logger();
    let args = Args::parse();

    // Determine if we should use emoji (default to true unless --no-emoji is specified)
    let use_emoji = if args.no_emoji {
        false
    } else {
        args.emoji || !args.no_emoji
    };

    let config = DisplayConfig {
        max_lines: args.max_lines,
        dir_limit: args.dir_limit,
        sort_by: match args.sort_by.as_str() {
            "size" => SortBy::Size,
            "modified" => SortBy::Modified,
            "created" => SortBy::Created,
            _ => SortBy::Name,
        },
        dirs_first: args.dirs_first,
        use_colors: !args.no_color,
        color_theme: match args.color_theme.to_lowercase().as_str() {
            "light" => ColorTheme::Light,
            "dark" => ColorTheme::Dark,
            "none" => ColorTheme::None,
            _ => ColorTheme::Auto,
        },
        use_emoji,
        size_colorize: args.color_sizes,
        date_colorize: args.color_dates,
        detailed_metadata: args.detailed,
        show_system_dirs: args.show_system_dirs,
    };

    // Initialize the GitIgnoreContext
    let mut gitignore_ctx = if args.no_gitignore {
        // Create an empty context if gitignore is disabled
        GitIgnoreContext::new(&args.path)?
    } else {
        GitIgnoreContext::new(&args.path)?
    };
    
    // Scan the directory tree
    let root = scan_directory(
        &args.path, 
        &mut gitignore_ctx, 
        args.max_depth, 
        Some(config.show_system_dirs)
    )?;
    
    // Format and print the tree
    let output = format_tree(&root, &config)?;
    println!("{}", output);
    
    Ok(())
}
