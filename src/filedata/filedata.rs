use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct FileData {
    /// The path to the file to read
    pub paths: HashMap<PathBuf, PathBuf>
}


   
#[derive(Serialize, Deserialize)]
pub struct Config {
   pub default_ui: String,
   pub ui_config: HashMap<String, HashMap<String, String>>,
}



pub fn get_file_track_data () -> FileData {
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

pub fn write_file_track_data (file_data: FileData) {
    let tracking_data_path = PathBuf::from(r"data.json");
    let json_output_data = serde_json::to_string(&file_data).expect("Could not convert to json."); 
    let mut f = File::create(tracking_data_path).expect("Unable to create file");
    f.write_all(json_output_data.as_bytes()).expect("Unable to write data");
}


pub fn get_config () -> Config {
    let mut read_data = String::new();
    let mut read_file = File::open("config.toml").expect("Unable to open file");
    read_file.read_to_string(&mut read_data).expect("Error converting file contents to string."); 
    let toml_output_data: Config = toml::from_str(&read_data).expect("toml fail.");
    return toml_output_data
}

