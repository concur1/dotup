mod sync;
mod filedata;
use clap::Parser;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::collections::HashMap;
use std::path::PathBuf;
use std::path::Path;
use std::fs;
use std::fs::OpenOptions;

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

fn main() {
    let repo_path = PathBuf::from("../dotup_test_repo");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Cli::parse();
    match args.action.as_ref() {
        "track" => track(args),
        "untrack" => untrack(args),
        "run" => sync::sync::sync(&repo_path).expect("Syncing failed."),
        _ => println!("other action:")

    }
}



