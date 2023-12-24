mod sync;
mod filedata;
use clap::Parser;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use std::process::Command;
use std::thread;



#[derive(Debug, Serialize, Deserialize)]
struct T {
    system_file_path: String,
}

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    action: String,
    /// The path to the file to read
    path: PathBuf,
}

fn track(args: Cli) {
    let abs_path = fs::canonicalize(&args.path).expect("Error getting absolute path.");
    let mut read_file_data = filedata::filedata::get_file_track_data();
    if read_file_data.paths.contains_key(&abs_path) {
        println!("{abs_path:?} is already tracked.");
        return
    }
    let generic_path = abs_path.clone();
    read_file_data.paths.insert(abs_path, generic_path);
    let file_data = filedata::filedata::FileData { paths: read_file_data.paths };
    filedata::filedata::write_file_track_data(file_data);
    println!("File tracked.");
    }

fn untrack(args: Cli) {
    let abs_path = fs::canonicalize(&args.path).expect("Error getting absolute path.");
    let mut read_file_data = filedata::filedata::get_file_track_data();
    if !!!read_file_data.paths.contains_key(&abs_path) {
        println!("{abs_path:?} is not tracked.");
        return
    }
    read_file_data.paths.remove(&abs_path);
    let file_data = filedata::filedata::FileData { paths: read_file_data.paths };
    filedata::filedata::write_file_track_data(file_data);
    println!("File untracked.");
    }

fn launch_ui(repo_path: PathBuf) {

    filedata::filedata::get_config();
    let config = filedata::filedata::get_config();
    let default_ui = config.default_ui;
    let uiconfig: HashMap<String, String> = config.ui_config.get(&default_ui).expect("Get nested hashmap:").to_owned();
    println!("{default_ui:?}");
    println!("{uiconfig:?}");
    let repo_path_arg_name = uiconfig.get("repo_path_arg_name").expect("fail get repo path arg name:");
    let additional_args = uiconfig.get("additional_args")
        .expect("Additional_args have not been supplied in the config. Set to an empty string if there are no args to be supplied.")
        .split(" ")
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    let mut additional_args: Vec<&String> = additional_args.iter().collect();
    
    let arg_string = format!("{}", repo_path.display());
    let mut arg_vec = vec![repo_path_arg_name, &arg_string];
    let arg_vec = arg_vec.append(&mut additional_args);
    println!("additional_args - {additional_args:?}");
    let _ = Command::new(default_ui)
            .arg(repo_path_arg_name)
            .arg(arg_string.clone())
            .args(additional_args)
            .status()
            .expect("Failed to execute command");
}

fn run(repo_path: PathBuf) {
    let tracking_data_path = fs::canonicalize(PathBuf::from(r"data.json")).expect("Error:");
    let repo_path = fs::canonicalize(&repo_path).expect("Error getting absolute path.");
    let repo_path_clone = repo_path.clone();
    thread::spawn(move || {
        //println!("test {repo_path:?}")
        sync::sync::sync(&repo_path_clone, &tracking_data_path ).expect("Syncing failed.");
        });
    launch_ui(repo_path);
}

//fn git(repo_path: PathBuf, git_args: Vec<String>) {
//    let arg_string = format!("{}", repo_path.display());
//    let _ = Command::new("git")
//            .arg("-C")
//            .arg(arg_string)
//            .status()
//            .expect("Failed to execute command");
//}

fn main() {
    let repo_path = PathBuf::from("../dotup_test_repo");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Cli::parse();
    match args.action.as_ref() {
        "track" => track(args),
        "untrack" => untrack(args),
        "run" => run(repo_path),
        _ => println!("other action:")
    }
}



