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
