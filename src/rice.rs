use std::path::PathBuf;

use crate::json::{self, get_config_file_path};

#[derive(clap::Subcommand)]
pub enum Crud {
    Add {
        #[arg(long = "id", short = 'i')]
        id: String,
    },
    List,
    File {
        #[command(subcommand)]
        crud: crate::symlink::Crud,
    },
    Change {
        #[arg(long = "id", short = 'i')]
        id: String,
    },
    Remove {
        #[arg(long = "id", short = 'i')]
        id: String,
    },
    Update {
        #[arg(long = "id", short = 'i')]
        id: String,
        #[arg(long = "new-id", short = 'n')]
        new_id: String,
    },
}

pub fn add(id: String) {
    let mut data: json::Data = json::read_json();

    if data.rices.iter().any(|r| r.id == id) {
        eprintln!("Rice with id '{}' already exists.", id);
        return;
    }

    let rice = json::Rice::new(id.clone());
    data.rices.push(rice);

    match json::json_write(&data) {
        Ok(_) => {
            println!("----- Creating Rice -----");
            println!("Rice {} successfully added.", id)
        }
        Err(e) => eprintln!("Failed to add rice: {}", e),
    }
}

pub fn list() {
    let data: json::Data = json::read_json();

    println!("----- Rices -----");
    for rice in data.rices {
        println!("ID: {}", rice.id);
    }
}

pub fn remove(id: String) {
    let mut data: json::Data = json::read_json();

    let rice_index = match data.rices.iter().position(|r| r.id == id) {
        Some(index) => index,
        None => {
            eprintln!("No rice found with id '{}'.", id);
            return;
        }
    };

    let rice_dir = get_config_file_path().join("rices").join(id.clone());
    if rice_dir.exists() {
        std::fs::remove_dir_all(&rice_dir).expect("Failed to remove rice directory");
    }
    data.rices.remove(rice_index);

    match json::json_write(&data) {
        Ok(_) => {
            println!("----- Removing Rice -----");
            println!("Rice {} successfully removed.", id)
        }
        Err(e) => eprintln!("Failed to remove rice: {}", e),
    }
}

pub fn change(id: String) {
    let data: json::Data = json::read_json();

    let rice_index = data
        .rices
        .iter()
        .position(|r| r.id == id)
        .expect("Rice ID not found");

    println!("----- Create Symlinks -----");

    for symlink in &data.rices[rice_index].symlinks {
        let file = &data.files[symlink.file];

        if PathBuf::from(&file.path).is_symlink() {
            std::fs::remove_file(&file.path).expect("Failed to remove existing symlink file");
        } else {
            println!("⚠️  The destination '{}' is not a symlink.", file.path);
            print!("Do you want to force replacement? [y/N]: ");

            use std::io::{self, Write};
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            if input != "y" && input != "yes" {
                println!("Skip '{}'", file.path);
                continue;
            }

            // Usuário confirmou, remove o arquivo
            if PathBuf::from(&file.path).exists() {
                if let Err(e) = std::fs::remove_file(&file.path) {
                    eprintln!("Error creating symlink {}: {}", file.path, e);
                    continue;
                }
            }
        }

        if let Err(e) = std::os::unix::fs::symlink(&symlink.path, &file.path) {
            eprintln!("Error creating symlink {}: {}", file.path, e);
        } else {
            println!("✓ Symlink created: {} -> {}", file.path, symlink.path);
        }
    }
}

pub fn update(id: String, new_id: String) {
    let mut data: json::Data = json::read_json();

    let rice_path = get_config_file_path().join("rices").join(id.clone());
    let new_rice_path = get_config_file_path().join("rices").join(new_id.clone());

    if rice_path.exists() {
        if let Err(e) = std::fs::rename(&rice_path, &new_rice_path) {
            eprintln!("Failed to rename rice directory: {}", e);
            return;
        }
    } else {
        eprintln!("Rice directory '{}' does not exist.", rice_path.display());
        return;
    }

    match data.rices.iter_mut().find(|r| r.id == id) {
        Some(rice) => rice.id = new_id.clone(),
        None => {
            eprintln!("No rice found with id '{}'.", id);
            return;
        }
    };

    match json::json_write(&data) {
        Ok(_) => {
            println!("----- Updating Rice -----");
            println!("Rice {} successfully updated to {}.", id, new_id);
        }
        Err(e) => eprintln!("Failed to update rice: {}", e),
    }
}
