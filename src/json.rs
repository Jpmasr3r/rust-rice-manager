use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

const PATH_TO_CONFIG_FILE: &str = ".config/rrm/config.json";

fn get_config_file_path() -> PathBuf {
    PathBuf::from(std::env::var("HOME").expect("Variável HOME não encontrada"))
        .join(PATH_TO_CONFIG_FILE)
}

fn create_default_config() -> String {
    let config_path = get_config_file_path();
    let default_data = Data::default();
    let json = serde_json::to_string_pretty(&default_data).unwrap();

    // Criar diretório pai se não existir
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    fs::write(&config_path, &json).unwrap();
    json
}

pub fn read_json() -> Data {
    let config_path = get_config_file_path();

    let content = fs::read_to_string(&config_path).unwrap_or_else(|_| create_default_config());

    serde_json::from_str(&content).unwrap_or_else(|_| {
        eprintln!("Erro ao parsear JSON, recriando arquivo padrão");
        create_default_config();
        Data::default()
    })
}

pub fn json_write(data: &Data) -> io::Result<()> {
    let config_path = get_config_file_path();
    let json = serde_json::to_string_pretty(&data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Garantir que o diretório existe
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(config_path, json)
}

#[derive(Serialize, Deserialize, Default)]
pub struct Data {
    pub apps: Vec<App>,
    pub files: Vec<File>,
    pub rices: Vec<Rice>,
    pub config: Config,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    pub name: String,
}

impl App {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub path: String,
    pub app_index: usize,
}

impl File {
    pub fn new(path: String, app_index: usize, id: String) -> Self {
        Self {
            id,
            path,
            app_index,
        }
    }
}

#[derive(Serialize, Deserialize)]
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

    pub fn create_rice_dir(&self) -> io::Result<()> {
        let path = Config::get_config_path().join("rices").join(&self.id);
        fs::create_dir_all(path)
    }

    pub fn delete_rice_dir(&self) -> io::Result<()> {
        let path = Config::get_config_path().join("rices").join(&self.id);
        fs::remove_dir_all(path)
    }

    pub fn rename_rice_dir(&self, new_id: &str) -> io::Result<()> {
        let old_path = Config::get_config_path().join("rices").join(&self.id);
        let new_path = Config::get_config_path().join("rices").join(new_id);
        fs::rename(old_path, new_path)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub rices_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rices_path: String::from("rices/"),
        }
    }
}

impl Config {
    pub fn get_config_path() -> PathBuf {
        PathBuf::from(std::env::var("HOME").expect("Variável HOME não encontrada"))
            .join(".config")
            .join("rrm")
    }

    pub fn create_config_dir() -> io::Result<()> {
        fs::create_dir_all(Self::get_config_path())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Symlink {
    pub file_index: usize,
    pub id: String,
}

impl Symlink {
    pub fn new(file_index: usize, id: String) -> Self {
        Self { file_index, id }
    }
}
