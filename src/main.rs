use std::path::Path;
mod sync;


fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    sync::sync(Path::new("/home/o/projects/file-backup/output/")).expect("Failed to sync:");
}


