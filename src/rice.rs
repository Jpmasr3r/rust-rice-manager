use crate::json;

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
        Ok(_) => println!("Rice {} successfully added.", id),
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

pub fn change(id: String) {
    let data: json::Data = json::read_json();

    let rice_index = data
        .rices
        .iter()
        .position(|r| r.id == id)
        .expect("Rice ID not found");

    for symlink in &data.rices[rice_index].symlinks {
        let file = &data.files[symlink.file];
        let file_path = std::path::Path::new(&file.path);

        if file_path.exists() {
            if let Err(e) = std::fs::remove_file(&file.path) {
                eprintln!("Erro ao remover {}: {}", file.path, e);
                continue;
            }
        }

        if let Err(e) = std::os::unix::fs::symlink(&symlink.path, &file.path) {
            eprintln!("Erro ao criar symlink {}: {}", file.path, e);
        } else {
            println!("✓ Symlink criado: {} -> {}", file.path, symlink.path);
        }
    }
}
