use std::fs;
use std::time::UNIX_EPOCH;
use serde::{Serialize, Deserialize};
use notify;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event};
use std::path::{Path, PathBuf};
use crate::filedata::filedata;

#[derive(Debug, Serialize, Deserialize)]
struct T {
    system_file_path: String,
}

// Returns the 'dest_path'. If the system path is supplied as 'path' then a repository path
// will be returned in the dest_path. If a repo path is supplied then a system path will eb returned.
//
// # Arguments
//
// * `path` - The supplied path, either a system filepath or a repo filepath.
// * `repo_path` - The path to the git repository that is being used to abckup files.
fn dest_path(path: &Path, repo_path: &Path) -> PathBuf {
    if path.starts_with(repo_path) {
        abs_repo_to_system(path, repo_path)
    } else {
        system_to_repo(path, repo_path)
    }
}

pub fn sync_all(config: filedata::Config, repo_path: &Path) {
    let abs_repo_path = fs::canonicalize(&repo_path).expect("Error getting absolute path.");
    let files_to_track = config.files.get("nixos").expect("get nixos files error:");
    for (local_path, system_path) in files_to_track.clone().into_iter() {
        let file_repo_abs_path = repo_to_abs_repo(&local_path, &abs_repo_path);
        sync_files(&system_path, &file_repo_abs_path).expect("Error with initial file syncing");
    }
}

fn system_to_repo(path: &Path, repo_path: &Path) -> PathBuf {
    let dest = repo_path.to_path_buf();
    dest.join(path.strip_prefix("/").expect("Not a prefix."))
}

fn repo_to_abs_repo(path: &Path, repo_path: &Path) -> PathBuf {
    let path = path.strip_prefix("/").expect("strip prefix error:");
    repo_path.join(path)
}

fn abs_repo_to_system(path: &Path, repo_path: &Path) -> PathBuf {
    let dest = Path::new("/").join(path.strip_prefix(repo_path).expect("Not a prefix.")).to_path_buf();
    dest
}

// Syncs the files specified in the configureation with the supplied repository path.
//
// # Arguments
//
// * 'repo_path' The path of the repo to sync with.
pub fn sync(repo_path: &Path, tracking_data_path: &Path) -> Result<(), serde_json::Error> {
    let abs_repo_path = fs::canonicalize(&repo_path).expect("Error getting absolute path.");
    let data = filedata::get_config(); 
    let files_to_track = data.files.get("nixos").expect("get nixos files error:");
    sync_all(filedata::get_config(), repo_path);

    let (tx, rx) = std::sync::mpsc::channel();
    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default()).expect("Error:");


    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(tracking_data_path, RecursiveMode::Recursive).expect("Error:");
    for (local_path, system_path) in files_to_track.clone().into_iter() {
        let file_repo_abs_path = repo_to_abs_repo(&local_path, &repo_path);
        watcher.watch(Path::new(&system_path),RecursiveMode::Recursive).expect("Error:");
        // println!("(key_path {key_path:?})");
        // println!("(dest_path_final {dest_path_final:?})");
        watcher.watch(Path::new(&file_repo_abs_path), RecursiveMode::Recursive).expect("Error (repo_path {abs_repo_path:?})");
    }
    for res in rx {
        match res {
            Ok(event) => if event.paths.contains(&tracking_data_path.to_owned()) {
              break;
            } else {
              //let updated_paths = event.paths.clone();
              event_handler(event, abs_repo_path.as_ref());
            },
            Err(error) => log::error!("Error: {error:?}"),
        }
        //match res {
        //    Ok(event) => event_handler(event, abs_repo_path.as_ref()),
        //    Err(error) => log::error!("Error: {error:?}"),
        //}
    }
    let _ = sync(repo_path, tracking_data_path);

    Ok(())
}


// Runs on an event triggered by the notify watcher.
//
// # Arguments
//
// * `event` - the event type supplied by the notify watcher.
// * `repo_path` - the path to the git repository that is used for backup.
fn event_handler(event: Event, repo_path: &Path) {
    // println!("event: {:?}", event.kind);
    // println!("paths: {:?}", event.paths);
    if event.kind.is_modify() {
        for path in event.paths {
            let dest_path = dest_path(&path, repo_path);
            //println!("event path:{path:?}, dest_path: {dest_path:?}");
            sync_files(&path, &dest_path).expect("Error:");    
            //println!("Files syncing: {path:?} -> {dest_path:?}");
        }
    }
}

// Creates a directory and all required parents before copying a file
//
// # Arguments
//
// * `source_file_path` - The filepath to copy from.
// * `dest_file_path` - The filepath to copy to.
fn copy_file(source_file_path: &Path, dest_file_path: &Path) -> std::io::Result<()> {
    let prefix = dest_file_path.parent().unwrap();
    fs::create_dir_all(prefix).unwrap();
    fs::copy(source_file_path, dest_file_path)?;
    Ok(())
}

// Copies files if a hash determines they are unequal.
//
// # Arguments
//
// * `source_file_path` - The filepath to copy from.
// * `dest_file_path` - The filepath to copy to.
fn copy_file_if_unequal(source_file_path: &Path, dest_file_path: &Path) -> std::io::Result<()> {
    let source_hash = blake3::hash(fs::read_to_string(source_file_path).expect("Could not read file:{source_file_path:?}").as_bytes());
    let dest_hash = blake3::hash(fs::read_to_string(dest_file_path).expect("Could not read file:{source_file_path:?}").as_bytes());
    //let hashes_match = source_hash == dest_hash;
    // println!("hashes match: {hashes_match:?}");
    if source_hash != dest_hash {
        copy_file(source_file_path, dest_file_path).expect("Error:");
    }
    Ok(())
}

// Sync the two supplied files.
//
// # Arguments
//
// * `system_path` - The filepath in the system to sync.
// * `repo_path` - The filepath in the repo to sync.
pub fn sync_files(system_path: &Path, repo_path: &Path) -> std::io::Result<()> {
    if !!!system_path.exists() {
        // println!("file does not exist in repo, creating file...");
        copy_file(repo_path, system_path).expect("Error:");
    }
    let system_metadata = fs::metadata(system_path)?;
    let system_modified = system_metadata.modified().expect("Creation time unsupported");
    let system_seconds = system_modified.duration_since(UNIX_EPOCH)
                  .expect("File A thinks it was created before Epoch")
                  .as_secs();
    if !!!repo_path.exists() {
        // println!("file does not exist in repo, creating file...");
        copy_file(system_path, repo_path).expect("Error:");
    }

    let repo_metadata = fs::metadata(repo_path)?;
    let repo_modified = repo_metadata.modified().expect("Creation time unsupported");
    let repo_seconds = repo_modified.duration_since(UNIX_EPOCH)
                  .expect("File A thinks it was created before Epoch")
                  .as_secs();
    // println!("{repo_seconds:?}  {system_seconds:?}");
    if system_seconds>repo_seconds {
        copy_file_if_unequal(system_path, repo_path).expect("Error:");
        // println!("overwrite repo file");
    } else {
        copy_file_if_unequal(repo_path, system_path).expect("Error:");
        // println!("overwrite system files");
    }
    Ok(())
}


