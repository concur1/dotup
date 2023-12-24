mod sync;
mod filedata;
use clap::Parser;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;
use std::process::{Command, Stdio};
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
    let mut read_file_data = filedata::filedata::get_file_data();
    if read_file_data.paths.contains_key(&abs_path) {
        println!("{abs_path:?} is already tracked.");
        return
    }
    let generic_path = abs_path.clone();
    read_file_data.paths.insert(abs_path, generic_path);
    let file_data = filedata::filedata::FileData { paths: read_file_data.paths };
    filedata::filedata::write_file_data(file_data);
    println!("File tracked.");
    }

fn untrack(args: Cli) {
    let abs_path = fs::canonicalize(&args.path).expect("Error getting absolute path.");
    let mut read_file_data = filedata::filedata::get_file_data();
    if !!!read_file_data.paths.contains_key(&abs_path) {
        println!("{abs_path:?} is not tracked.");
        return
    }
    read_file_data.paths.remove(&abs_path);
    let file_data = filedata::filedata::FileData { paths: read_file_data.paths };
    filedata::filedata::write_file_data(file_data);
    println!("File untracked.");
    }

fn run(command: String, repo_path: PathBuf) {

    let tracking_data_path = fs::canonicalize(PathBuf::from(r"data.json")).expect("Error:");
    let repo_path = fs::canonicalize(&repo_path).expect("Error getting absolute path.");

    let repo_path_clone = repo_path.clone();
    thread::spawn(move || {
        //println!("test {repo_path:?}")
        sync::sync::sync(&repo_path_clone, &tracking_data_path ).expect("Syncing failed.");
        });
    let arg_string = format!("{}", repo_path.display());
    println!("{arg_string}");
    let status = Command::new("gitui")
        .arg("-d")
        .arg(arg_string)
        .status()
        .expect("Failed to execute command");
    println!("Exited with status code: {}", status.code().unwrap());
}

fn main() {
    let repo_path = PathBuf::from("../dotup_test_repo");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Cli::parse();
    match args.action.as_ref() {
        "track" => track(args),
        "untrack" => untrack(args),
        "run" => run("test".to_string(), repo_path),
        _ => println!("other action:")
    }
}



