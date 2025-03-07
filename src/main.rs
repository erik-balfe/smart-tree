use anyhow::Result;
use clap::Parser;
use smart_tree::{format_tree, scan_directory, DisplayConfig, GitIgnore, SortBy, ColorTheme};
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
    };

    let gitignore = GitIgnore::load(&args.path)?;
    let root = scan_directory(&args.path, &gitignore, args.max_depth)?;
    let output = format_tree(&root, &config)?;

    println!("{}", output);
    Ok(())
}
