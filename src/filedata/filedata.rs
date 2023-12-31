use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::path::PathBuf;
use dirs::{self, data_local_dir};

use crate::get_hostname;

#[derive(Serialize, Deserialize)]
pub struct FileData {
    /// The path to the file to read
    pub paths: HashMap<PathBuf, PathBuf>
}


   
#[derive(Serialize, Deserialize)]
pub struct Config {
   pub repo_name: String,
   pub program: HashMap<String, HashMap<String, String>>,
   pub files: HashMap<String, HashMap<PathBuf, PathBuf>>,
}

pub fn get_config_path() -> PathBuf {
    let mut path = get_repo_path();
    //path.push("dotup");
    path.push("config.toml");
    path
}

pub fn get_repo_path() -> PathBuf {
    let mut repo_path = dirs::data_dir().expect("No data directory found.");
    repo_path.push("dotup/");
    repo_path.push("dotup_test_repo/");
    repo_path.push("default".to_string());
    repo_path
}

pub fn generalize_directory(path: PathBuf) -> PathBuf {
    let general_home_dir= "/home/user/";
    let local_home_dir = dirs::home_dir().expect("home dir.");
    if path.starts_with(local_home_dir.clone()) {
        let path = path.strip_prefix(local_home_dir).expect("remove home prefix");
        let path_buf = PathBuf::from(general_home_dir).join(path);
        return path_buf
    }
    path
}

pub fn specify_directory(path: PathBuf) -> PathBuf {
    let general_home_dir= "/home/user/";
    let local_home_dir = dirs::home_dir().expect("home dir.");
    if path.starts_with(general_home_dir) {
        let path = path.strip_prefix(general_home_dir).expect("remove home prefix");
        let path_buf = PathBuf::from(local_home_dir).join(path);
        return path_buf
    }
    path
}

pub fn get_config () -> Config {
    let mut read_data = String::new();
    let mut git_data_path = get_repo_path();
    git_data_path.push(".git");
    if !!!get_config_path().exists() {
        let _ = fs::create_dir_all(get_repo_path());
        if git_data_path.exists() {
            create_default_config_file();
        }
    }
    let mut read_file = File::open(get_config_path()).expect("Unable to open file");
    read_file.read_to_string(&mut read_data).expect("Error converting file contents to string."); 
    let mut toml_output_data: Config = toml::from_str(&read_data).expect("toml fail.");
    if !!!toml_output_data.files.contains_key(&get_hostname()) {
        toml_output_data.files.insert(get_hostname(), HashMap::new());
        write_config(toml_output_data);
        get_config()
    }
    else {
        toml_output_data
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config2 {
   file_map : HashMap<String, String>,
   variables: HashMap<String, String>,
}

pub fn write_config (config: Config) {
    let config_path = get_config_path();
    let prefix: &Path = config_path.parent().expect("cant");
    fs::create_dir_all(prefix).unwrap();
    let toml_output_data: String = toml::to_string(&config).expect("toml fail.");
    fs::write(get_config_path(), toml_output_data).expect("Unable to write file");
}

fn create_default_config_file() {
    let nixos = HashMap::new();
    let file_config = HashMap::from([
    (get_hostname().to_owned(), nixos),
    ]);
    let gitui = HashMap::from([
    ("program".to_owned(), "gitui".to_owned()),
    ("repo_path_arg_name".to_owned(), "--directory".to_owned()),
    ("additional_args".to_owned(), "--watcher --logging".to_owned()),
    ]);
    let lazygit = HashMap::from([
    ("program".to_owned(), "lazygit".to_owned()),
    ("repo_path_arg_name".to_owned(), "--path".to_owned()),
    ("additional_args".to_owned(), "".to_owned()),
    ]);
    let gitkraken = HashMap::from([
    ("program".to_owned(), "gitkraken".to_owned()),
    ("repo_path_arg_name".to_owned(), "--path".to_owned()),
    ("additional_args".to_owned(), "".to_owned()),
    ]);
    let ui_config_map = HashMap::from([
    ("gitui".to_owned(), gitui),
    ("lazygit".to_owned(), lazygit),
    ("gitkraken".to_owned(), gitkraken),
    ]);

    let data =  Config  {
        repo_name: "default".to_owned(),
        program: ui_config_map,
        files: file_config,
        };
    write_config(data);
}
