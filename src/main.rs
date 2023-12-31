mod sync;

mod filedata;
use clap::ArgMatches;
use filedata::filedata::get_config;
use std::{collections::HashMap, ffi::OsString};
use std::path::PathBuf;
use std::fs;
use std::process::Command;
use clap;
use std::env;
use clap::{arg, command};
use std::thread;
use gethostname;

fn get_hostname() -> String {
    let hostname = gethostname::gethostname().into_string();
    let hostname = hostname.expect("hostname");
    hostname

}

// Add suplied path to the list of files to track
// * `path` - The path that is tracked.
fn track(path: PathBuf) {
    let hostname = get_hostname();
    let abs_local_path = fs::canonicalize(&path).expect("Error getting absolute path.");
    let mut config = filedata::filedata::get_config();
    let files_map = config.files.get(&hostname).expect("get files map error:");
    let local_files: Vec<PathBuf> = files_map.values().cloned().collect();
    if local_files.contains(&abs_local_path) {
        return
    }
    let generic_path = filedata::filedata::generalize_directory(abs_local_path.clone());
    let files_map = config.files.get_mut(&hostname).expect("get nixos.");

    files_map.insert(generic_path, abs_local_path);
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
    let hostname = get_hostname();
    let abs_path = fs::canonicalize(&path).expect("Error getting absolute path.");
    let mut config = filedata::filedata::get_config();
    let files_map = config.files.get(&hostname).expect("get files map error:");
    let local_files: Vec<PathBuf> = files_map.values().cloned().collect();
    if !!!local_files.contains(&abs_path) {
        println!("{abs_path:?} is not tracked.");
        return
    }
    let repo_abs_path = find_key_for_value(files_map.clone(), &abs_path).expect("get repo path error:");
    let files_map = config.files.get_mut(&hostname).expect("get nixos.");
    files_map.remove(&repo_abs_path);
    filedata::filedata::write_config(config);
    println!("File untracked.");
    }

// Start the ui.
// * `repo_path` - The path to the repo.
fn launch_ui(repo_path: PathBuf, program: String) {
    let config = filedata::filedata::get_config();
    let error_message = format!("Error getting the details for program: {program}. Does a section in the config.toml exist for [program.{program}]?");
    let uiconfig: HashMap<String, String> = config.program.get(&program).expect(&error_message).to_owned();
    let additional_args = uiconfig.get("additional_args")
        .expect("Additional_args have not been supplied in the config. Set to an empty string if there are no args to be supplied.")
        .split(" ")
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    let program_name = uiconfig.get("name").expect("get program name:");
    let additional_args: Vec<&String> = additional_args.iter().collect();
    
    let mut args = additional_args;
    if args[0] == "" {
        args.remove(0);
    }

    let _ = Command::new(&program_name)
            .current_dir(repo_path)
            .args(args)
            .status()
            .expect("Failed to execute command");
}

// Start a new thread that will use the sync function to sync as well as starting the default ui.
// * `repo_path` - The path to the repo.
fn run_ui(repo_path: PathBuf, program: String) {
    let tracking_data_path = fs::canonicalize(filedata::filedata::get_config_path()).expect("Error:");
    let repo_path = fs::canonicalize(&repo_path).expect("Error getting absolute path.");
    let repo_path_clone = repo_path.clone();
    thread::spawn(move || {
        sync::sync::sync(&repo_path_clone, &tracking_data_path ).expect("Syncing failed.");
        });
    launch_ui(repo_path, program);
}

// Call git based on the repo path an supplied args for git
// * `repo_path` - The path to the repo.
// * `git_args` - The args that have been supplied for git.
fn git(repo_path: PathBuf, git_args: Vec<&OsString>) {
    let arg_string = format!("{}", repo_path.display());
    //println!("repo path to create {repo_path:?}");
    let _ = Command::new("git")
            .arg("-C")
            .arg(arg_string)
            .args(git_args)
            .status()
            .expect("Failed to execute command");
}

// Use clap to create the cli, returning the matches.
fn get_cli(repo_path: PathBuf) -> ArgMatches {
    let mut git_data_path = repo_path.clone();
    git_data_path.push(".git");
    //println!("config_path: {git_data_path:?}");
    if git_data_path.exists() {
        sync::sync::sync_all(get_config(), &repo_path);
    }
    let cmd = command!()
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
            clap::Command::new("run")
                .about("Run the specified program (the program must be listed in the config). If no program is specified then the 'default' program will be run (if it exists in the config.toml file).")
                .arg(arg!([PROGRAM] "The UI program specified in metadata that will be launched.").default_value("default"))
                .arg_required_else_help(false),
        );
    cmd.get_matches()  
}

// Initialise the repo if it has not already been initialized.
// * `repo_path` - The path to the repo.
fn init_repo_dir(repo_path: PathBuf) {
    let mut mut_repo_path = repo_path.clone();
    mut_repo_path.push(".git");
    if mut_repo_path.exists() {
        return
    }
    let _ = fs::create_dir_all(repo_path.clone());
    }


fn main() {
    let repo_path = filedata::filedata::get_repo_path();
    init_repo_dir(repo_path.clone());
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    match get_cli(repo_path.clone()).subcommand() {
        Some(("track", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").expect("fail");
            let path = PathBuf::from(path);
            println!("running track {path:?}");
            track(path.to_owned());
        },
        Some(("untrack", sub_matches)) => {
            let path = sub_matches.get_one::<String>("PATH").expect("fail");
            let path = PathBuf::from(path);
            println!("running untrack {path:?}");
            untrack(path.to_owned());
        },
        Some(("run", sub_matches)) => {
            let program = sub_matches.get_one::<String>("PROGRAM").expect("fail").clone();
            println!("running track");
            run_ui(repo_path, program);
        },
        Some((ext, sub_matches)) => {
            let args = &mut sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            let ext = OsString::from(ext);
            let mut all_args = vec![&ext];
            all_args.append(args);
            git(repo_path, all_args);
           
        },
        _ => unreachable!(),
        }
    ;
}




