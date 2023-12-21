use std::path::Path;
mod sync;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    action: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Cli::parse();
    let action = args.action;
    let path = args.path;
    println!("ation: {action:?}, path: {path:?}");
    sync::sync(Path::new("/home/o/projects/file-backup/output/")).expect("Failed to sync:");
}


