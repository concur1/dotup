mod sync;
use clap::Parser;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::io::Read;

use std::collections::HashMap;
#[derive(Debug, Serialize, Deserialize)]
struct T {
    system_file_path: String,
}

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    action: String,
    /// The path to the file to read
    path: Option<std::path::PathBuf>,
}

#[derive(Serialize, Deserialize)]
struct FileData {
    /// The path to the file to read
    paths: Vec<std::option::Option<std::path::PathBuf>>,
}

fn get_file_data () -> FileData {
    let mut read_data = String::new();
    let mut read_file = File::open("./data.json").expect("Unable to open file");
    read_file.read_to_string(&mut read_data).expect("Error converting file contents to string."); 
    let read_file_data: FileData = match serde_json::from_str(&read_data) {
        Ok(v) => v,
        Err(_) => FileData { paths : vec![] }
    };
    read_file_data
}

fn write_file_data (file_data: FileData) {
    let json_output_data = serde_json::to_string(&file_data).expect("egewfg"); 
    let mut write_file = File::create("./data.json").expect("Unable to create file");
    write_file.write_all(json_output_data.as_bytes()).expect("Unable to write data");
}

fn track(args: Cli) {
    println!("Track command!");
    let mut read_file_data = get_file_data();
    if read_file_data.paths.contains(&args.path) {
        ()
    } else
    {
    read_file_data.paths.push(args.path);
    let file_data = FileData { paths: read_file_data.paths };
    write_file_data(file_data);
    }
    }

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Cli::parse();
    match args.action.as_ref() {
        "track" => track(args),
        _ => println!("other action:")
    }

    //sync::sync(Path::new("/home/o/projects/file-backup/output/")).expect("Failed to sync:");
}


