//JSON file manipulation functions

use std::{fs, io::Write};

use serde::{Deserialize, Serialize};

const PATH_TO_CONFIG_FILE: &str = "config.json";

pub fn read_json() -> Data {
    let content = fs::read_to_string(PATH_TO_CONFIG_FILE).unwrap_or_else(|_| {
        // se não existir, cria o padrão
        let default_data = Data::default();
        let json = serde_json::to_string_pretty(&default_data).unwrap();
        let mut file = std::fs::File::create(PATH_TO_CONFIG_FILE).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        json
    });

    // tenta converter o conteúdo JSON em Data
    serde_json::from_str(&content).unwrap_or_else(|_| {
        // se der erro ao parsear, recria o arquivo padrão
        let default_data = Data::default();
        let json = serde_json::to_string_pretty(&default_data).unwrap();
        let mut file = std::fs::File::create(PATH_TO_CONFIG_FILE).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        default_data
    })
}

pub fn json_write(data: Data) -> std::io::Result<()> {
    let path = &PATH_TO_CONFIG_FILE;
    let json = serde_json::to_string_pretty(&data).unwrap();
    let mut file = std::fs::File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub apps: Vec<App>,
    pub files: Vec<File>,
    pub rices: Vec<Rice>,
    pub config: Config,
}

impl Data {
    fn default() -> Self {
        Self {
            apps: Vec::new(),
            files: Vec::new(),
            rices: Vec::new(),
            config: Config::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct App {
    pub name: String,
}

impl App {
    pub fn from(name: String) -> Self {
        Self { name }
    }
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub path: String,
    pub app_index: usize,
}

impl File {
    pub fn from(path: String, app_index: usize) -> Self {
        Self { path, app_index }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Rice {
    pub name: String,
    pub files_paths: Vec<Path>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub rices_path: String,
}

impl Config {
    pub fn default() -> Self {
        Self {
            rices_path: String::from("rices"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Path {
    pub path: String,
}
