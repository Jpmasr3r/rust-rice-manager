use std::path::PathBuf;

use crate::json;

#[derive(clap::Subcommand)]
pub enum Crud {
    Add {
        #[arg(long = "path", short = 'p')]
        path: String,
        #[arg(long = "id", short = 'i')]
        id: String,
    },
    List,
    Remove {
        #[arg(long = "id", short = 'i')]
        id: String,
    },
    Update {
        #[arg(long = "id", short = 'i')]
        id: String,
        #[arg(long = "new-id", short = 'n')]
        new_id: Option<String>,
        #[arg(long = "new-path", short = 'p')]
        new_path: Option<String>,
    },
}

pub fn add(id: String, path: String) {
    let mut data: json::Data = json::read_json();

    if data.files.iter().any(|f| f.id == id) {
        eprintln!("File with id '{}' already exists.", id);
        return;
    }

    let file = json::File::new(path, id.clone());
    data.files.push(file);

    match json::json_write(&data) {
        Ok(_) => {
            println!("----- Adding File -----");
            println!("File {} successfully added.", id)
        }
        Err(e) => eprintln!("Failed to add file: {}", e),
    }
}

pub fn list() {
    let data: json::Data = json::read_json();

    println!("----- Listing Files -----");
    for file in data.files {
        println!("ID: {}, Path: {}", file.id, file.path);
    }
}

pub fn remove(id: String) {
    let mut data: json::Data = json::read_json();

    let file_index = match data.files.iter().position(|f| f.id == id) {
        Some(index) => index,
        None => {
            eprintln!("No file found with id '{}'.", id);
            return;
        }
    };

    for rice in data.rices.iter_mut() {
        for symlink in rice.symlinks.iter() {
            if symlink.file == file_index {
                eprintln!(
                    "Cannot remove file '{}': it is still linked in rice '{}'.",
                    id, rice.id
                );
                return;
            }
        }
    }

    data.files.remove(file_index);

    match json::json_write(&data) {
        Ok(_) => {
            println!("----- Removing File -----");
            println!("File {} successfully removed.", id)
        }
        Err(e) => eprintln!("Failed to remove file: {}", e),
    }
}

pub fn update(id: String, new_id: Option<String>, new_path: Option<String>) {
    if new_id.is_none() && new_path.is_none() {
        eprintln!("No new values provided for update.");
        return;
    }

    let mut data: json::Data = json::read_json();

    let file = match data.files.iter_mut().find(|f| f.id == id) {
        Some(file) => file,
        None => {
            eprintln!("No file found with id '{}'.", id);
            return;
        }
    };

    if let Some(new_id) = &new_id {
        for rice in data.rices.iter_mut() {
            for symlink in rice.symlinks.iter_mut() {
                let symlink_path = PathBuf::from(&symlink.path);
                let symlink_parent_path = symlink_path.parent().expect(&format!(
                    "Failed to determine parent directory for symlink '{}' in rice '{}'.",
                    symlink.path, rice.id
                ));

                if symlink_parent_path
                    .to_string_lossy()
                    .split("/")
                    .last()
                    .expect("Failed to extract the last component of the symlink's parent path.")
                    == id
                {
                    let new_symlink_parent_path = symlink_parent_path
                        .parent()
                        .expect(
                            "Failed to determine the parent directory of the symlink's parent path.",
                        )
                        .join(new_id);
                    std::fs::rename(symlink_parent_path, &new_symlink_parent_path).expect(
                        &format!(
                            "Failed to rename symlink parent directory '{}' to '{}'.",
                            symlink_parent_path.display(),
                            new_symlink_parent_path.display()
                        ),
                    );

                    symlink.path =
                        new_symlink_parent_path
                            .join(symlink_path.to_string_lossy().split("/").last().expect(
                                "Failed to extract the last component of the symlink path.",
                            ))
                            .to_string_lossy()
                            .to_string();
                }
            }
        }
    }

    if let Some(new_id) = new_id {
        file.id = new_id;
    }

    if let Some(new_path) = new_path {
        file.path = new_path;
    }

    match json::json_write(&data) {
        Ok(_) => {
            println!("----- Updating File -----");
            println!("File {} successfully updated.", id)
        }
        Err(e) => eprintln!("Failed to update file: {}", e),
    }
}
