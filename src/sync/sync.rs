use std::fs;
use std::time::UNIX_EPOCH;
use serde::{Serialize, Deserialize};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, Event};
use std::path::{Path, PathBuf};
use crate::filedata::filedata;

#[derive(Debug, Serialize, Deserialize)]
struct T {
    system_file_path: String,
}



// Syncs the files specified in the configureation with the supplied repository path.
//
// # Arguments
//
// * 'repo_path' The path of the repo to sync with.
pub fn sync(repo_path: &Path, tracking_data_path: &Path) -> Result<(), serde_json::Error> {
    let abs_repo_path = fs::canonicalize(&repo_path).expect("Error getting absolute path.");

    // println!("repo path: {repo_path:?}");
    let data = filedata::get_file_track_data(); 
    let files_to_track = data.paths;

    let (tx, rx) = std::sync::mpsc::channel();
    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).expect("Error:");

    for (key_path, _) in files_to_track.clone().into_iter() {
        let dest_path_final = Path::new(&dest_path(&key_path, &abs_repo_path)).to_owned();
        sync_files(&key_path, &dest_path_final).expect("Error with initial file syncing");
    }

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(tracking_data_path, RecursiveMode::Recursive).expect("Error:");
    for (key_path, _) in files_to_track.into_iter() {
        watcher.watch(Path::new(&key_path),RecursiveMode::Recursive).expect("Error:");
        let dest_path_final = Path::new(&dest_path(&key_path, &abs_repo_path)).to_owned();
        // println!("(key_path {key_path:?})");
        // println!("(dest_path_final {dest_path_final:?})");
        watcher.watch(Path::new(&dest_path_final), RecursiveMode::Recursive).expect("Error (repo_path {abs_repo_path:?})");
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

// Returns the 'dest_path'. If the system path is supplied as 'path' then a repository path
// will be returned in the dest_path. If a repo path is supplied then a system path will eb returned.
//
// # Arguments
//
// * `path` - The supplied path, either a system filepath or a repo filepath.
// * `repo_path` - The path to the git repository that is being used to abckup files.
fn dest_path(path: &Path, repo_path: &Path) -> PathBuf {
    if path.starts_with(repo_path) {
        let dest = Path::new("/").join(path.strip_prefix(repo_path).expect("Not a prefix.")).to_path_buf();
        // println!("starts with repo. {repo_path:?}, {path:?}, {dest:?}");
        dest.to_path_buf()
    } else {
        let dest = repo_path.join(path.strip_prefix("/").expect("No prefix."));
        // println!("doesn't starts with repo. {repo_path:?}, {path:?}, {dest:?}");
        dest.to_path_buf()
    }
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


