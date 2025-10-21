use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

const PATH_TO_CONFIG_FILE: &str = ".config/rrm";

pub fn get_config_file_path() -> PathBuf {
    PathBuf::from(std::env::var("HOME").expect("Variável HOME não encontrada"))
        .join(PATH_TO_CONFIG_FILE)
}

pub fn create_default_config() -> String {
    let config_path = get_config_file_path().join("config.json");
    let default_data = Data::default();
    let json = serde_json::to_string_pretty(&default_data).unwrap();

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    fs::write(&config_path, &json).unwrap();
    json
}

pub fn read_json() -> Data {
    let config_path = get_config_file_path().join("config.json");

    let content = fs::read_to_string(&config_path).unwrap_or_else(|_| create_default_config());

    serde_json::from_str(&content).unwrap_or_else(|_| {
        eprintln!("Erro ao parsear JSON, recriando arquivo padrão");
        create_default_config();
        Data::default()
    })
}

pub fn json_write(data: &Data) -> io::Result<()> {
    let config_path = get_config_file_path().join("config.json");
    let json = serde_json::to_string_pretty(&data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(config_path, json)
}

#[derive(Serialize, Deserialize, Default)]
pub struct Data {
    pub files: Vec<File>,
    pub rices: Vec<Rice>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct File {
    pub path: String,
    pub id: String,
}

impl File {
    pub fn new(path: String, id: String) -> Self {
        Self { path, id }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Rice {
    pub id: String,
    pub symlinks: Vec<Symlink>,
}

impl Rice {
    pub fn new(id: String) -> Self {
        Self {
            id,
            symlinks: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Symlink {
    pub file: usize,
    pub path: String,
}

impl Symlink {
    pub fn new(file: usize, path: String) -> Self {
        Self { file, path }
    }
}
