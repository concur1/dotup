mod sync;
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

#[derive(Serialize, Deserialize)]
struct FileData {
    /// The path to the file to read
    paths: HashMap<PathBuf, PathBuf>
}

fn get_file_data () -> FileData {
    let tracking_data_path = PathBuf::from(r"data.json");
    let mut read_data = String::new();
    let default = FileData { paths : HashMap::new() };
    if !!!tracking_data_path.exists() {
        return default
    }
    let mut read_file = File::open("data.json").expect("Unable to open file");
    read_file.read_to_string(&mut read_data).expect("Error converting file contents to string."); 
    let read_file_data: FileData = match serde_json::from_str(&read_data) {
        Ok(v) => v,
        Err(_) => default
    };
    read_file_data
}

fn write_file_data (file_data: FileData) {
    let tracking_data_path = PathBuf::from(r"data.json");
    let json_output_data = serde_json::to_string(&file_data).expect("Could not convert to json."); 
    let mut f = File::create(tracking_data_path).expect("Unable to create file");
    f.write_all(json_output_data.as_bytes()).expect("Unable to write data");
}

fn track(args: Cli) {
    println!("Track command!");
    let abs_path = fs::canonicalize(&args.path).expect("Error getting absolute path.");
    println!("converting to absolute path: {:?}", abs_path);
    let mut read_file_data = get_file_data();
    if read_file_data.paths.contains_key(&abs_path) {
        println!("{abs_path:?} is already tracked.");
        return
    }
    let generic_path = abs_path.clone();
    read_file_data.paths.insert(abs_path, generic_path);
    let file_data = FileData { paths: read_file_data.paths };
    write_file_data(file_data);
    println!("File tracked.")
    }

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args = Cli::parse();
    match args.action.as_ref() {
        "track" => track(args),
        _ => println!("other action:")
    }
}


