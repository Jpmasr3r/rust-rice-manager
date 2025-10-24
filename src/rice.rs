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

        std::fs::remove_file(&file.path).expect("Failed to remove existing symlink file");

        if let Err(e) = std::os::unix::fs::symlink(&symlink.path, &file.path) {
            eprintln!("Erro ao criar symlink {}: {}", file.path, e);
        } else {
            println!("✓ Symlink criado: {} -> {}", file.path, symlink.path);
        }
    }
}
