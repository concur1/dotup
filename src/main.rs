mod sync;

mod filedata;
use clap::ArgMatches;
use serde::{Serialize, Deserialize};
use std::{collections::HashMap, ffi::OsString};
use std::path::PathBuf;
use std::fs;
use std::process::Command;
use std::thread;
use clap;
use std::env;
use clap::{arg, command};

#[derive(Debug, Serialize, Deserialize)]
struct T {
    system_file_path: String,
}

// Add suplied path to the list of files to track
// * `path` - The path that is tracked.
fn track(path: PathBuf) {
    let abs_path = fs::canonicalize(&path).expect("Error getting absolute path.");
    let mut config = filedata::filedata::get_config();
    let files_map = config.files.get("nixos").expect("get files map error:");
    let local_files: Vec<PathBuf> = files_map.values().cloned().collect();
    if local_files.contains(&abs_path) {
        println!("{abs_path:?} is already tracked.");
        return
    }
    let generic_path = abs_path.clone();
    let files_map = config.files.get_mut("nixos").expect("get nixos.");
    files_map.insert(abs_path, generic_path);
    filedata::filedata::write_config(config);
    println!("File tracked.");
    }

fn find_key_for_value<'a>(map: HashMap<PathBuf, PathBuf>, value: &PathBuf) -> Option<PathBuf> {
    map.iter()
        .find_map(|(key, val)| if val == value { Some(key.clone()) } else { None })
}

// Remove supplied path from the list of files to track
// * `path` - The path that is tracked.
fn untrack(path: PathBuf) {
    let abs_path = fs::canonicalize(&path).expect("Error getting absolute path.");
    let mut config = filedata::filedata::get_config();
    let files_map = config.files.get("nixos").expect("get files map error:");
    let repo_abs_path = find_key_for_value(files_map.clone(), &abs_path).expect("get repo path error:");
    let local_files: Vec<PathBuf> = files_map.values().cloned().collect();
    if !!!local_files.contains(&abs_path) {
        println!("{abs_path:?} is not tracked.");
        return
    }
    let files_map = config.files.get_mut("nixos").expect("get nixos.");
    files_map.remove(&repo_abs_path);
    filedata::filedata::write_config(config);
    println!("File tracked.");
    }

// Start the ui.
// * `repo_path` - The path to the repo.
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
    let additional_args: Vec<&String> = additional_args.iter().collect();
    
    let arg_string = format!("{}", repo_path.display());
    let mut args = additional_args;
    if args[0] == "" {
        args.remove(0);
    }
    args.push(repo_path_arg_name);
    args.push(&arg_string);

    //arg_vec.append(&mut additional_args);
    println!("args - {args:?}");
    //println!("arg_vec - {arg_vec:?}");
    let _ = Command::new(default_ui)
            .args(args)
            .status()
            .expect("Failed to execute command");
}

// Start a new thread that will use the sync function to sync as well as starting the default ui.
// * `repo_path` - The path to the repo.
fn run_ui(repo_path: PathBuf) {
    let tracking_data_path = fs::canonicalize(PathBuf::from(r"config3.toml")).expect("Error:");
    let repo_path = fs::canonicalize(&repo_path).expect("Error getting absolute path.");
    let repo_path_clone = repo_path.clone();
    thread::spawn(move || {
        sync::sync::sync(&repo_path_clone, &tracking_data_path ).expect("Syncing failed.");
        });
    launch_ui(repo_path);
}

// Call git based on the repo path an supplied args for git
// * `repo_path` - The path to the repo.
// * `git_args` - The args that have been supplied for git.
fn git(repo_path: PathBuf, git_args: Vec<&OsString>) {
    let arg_string = format!("{}", repo_path.display());
    println!("git args: {git_args:?}");
    println!("repo_path: {repo_path:?}");
    let _ = Command::new("git")
            .arg("-C")
            .arg(arg_string)
            .args(git_args)
            .status()
            .expect("Failed to execute command");
}

// Use clap to create the cli, returning the matches.
fn get_cli() -> ArgMatches {
    let matches = command!()
        //.version(crate_version!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            clap::Command::new("track")
                .about("Add a file to the list of files to be tacked.")
                .arg(arg!(<PATH> "The path of the file."))
                .arg_required_else_help(true),
        )
        .subcommand(
            clap::Command::new("untrack")
                .about("Remove a file to the list of files to be tacked.")
                .arg(arg!(<PATH> "The path of the file."))
                .arg_required_else_help(true),
        )
        .subcommand(
            clap::Command::new("ui")
                .about("Run the git ui that is set to 'default' in the config.toml file.")
        ).get_matches();
    matches   
}

fn main() {
    let config = filedata::filedata::get_config();
    let profile = config.repo_name;
    let all_repos_path = PathBuf::from("../dotup_test_repo/");
    let mut repo_path = all_repos_path;
    repo_path.push(profile);
    let _ = fs::create_dir_all(repo_path.clone());
    
    
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    match get_cli().subcommand() {
        Some(("track", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").expect("fail");
            let path = PathBuf::from(path);
            println!("running untrack {path:?}");
            track(path.to_owned());
        },
        Some(("untrack", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").expect("fail");
            let path = PathBuf::from(path);
            println!("running untrack {path:?}");
            untrack(path.to_owned());
        },
        Some(("ui", _)) => {
            println!("running track");
            run_ui(repo_path);
        },
        Some((ext, sub_matches)) => {
            println!("none");
            let args = &mut sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("ext:{ext:?}");
            println!("args:{args:?}");
            let ext = OsString::from(ext);
            let mut all_args = vec![&ext];
            all_args.append(args);

            println!("ext:{ext:?}");
            println!("all_args:{all_args:?}");

            git(repo_path, all_args);
           
        },
        _ => unreachable!(),
        }
    ;
}




