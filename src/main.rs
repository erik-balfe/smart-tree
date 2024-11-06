use clap::Parser;
use std::path::PathBuf;
mod display;
mod gitignore;
mod scanner;
mod types;

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
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = types::DisplayConfig {
        max_lines: args.max_lines,
        dir_limit: args.dir_limit,
        sort_by: match args.sort_by.as_str() {
            "size" => types::SortBy::Size,
            "modified" => types::SortBy::Modified,
            "created" => types::SortBy::Created,
            _ => types::SortBy::Name,
        },
        dirs_first: args.dirs_first,
    };

    // We'll implement these modules next
    let gitignore = gitignore::GitIgnore::load(&args.path)?;
    let root = scanner::scan_directory(&args.path, &gitignore, args.max_depth)?;
    let output = display::format_tree(&root, &config)?;

    println!("{}", output);
    Ok(())
}
