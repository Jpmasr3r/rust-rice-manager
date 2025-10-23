use std::path::PathBuf;

use crate::json;

#[derive(clap::Subcommand)]
pub enum Crud {
    Add {
        #[arg(long = "rice-id", short = 'r')]
        rice_id: String,
        #[arg(long = "file-id", short = 'f')]
        file_id: String,
        #[arg(long = "symlink-path", short = 'p')]
        symlink_path: String,
    },
    List {
        #[arg(long = "rice-id", short = 'r')]
        rice_id: String,
    },
    Remove {
        #[arg(long = "rice-id", short = 'r')]
        rice_id: String,
        #[arg(long = "file-id", short = 'f')]
        file_id: String,
    },
}

pub fn add(rice_id: String, file_id: String, symlink_path: String) {
    let mut data: json::Data = json::read_json();

    let rice_index = data
        .rices
        .iter()
        .position(|r| r.id == rice_id.clone())
        .expect("Rice ID not found");

    if data.rices[rice_index]
        .symlinks
        .iter()
        .any(|s| data.files[s.file].id == file_id)
    {
        eprintln!(
            "Symlink for file id '{}' in rice '{}' already exists.",
            file_id, rice_id
        );
        return;
    }

    let file_index = data
        .files
        .iter()
        .position(|f| f.id == file_id.clone())
        .expect("File ID not found");

    let file_path = json::get_config_file_path()
        .join("rices")
        .join(rice_id.clone())
        .join(file_id.clone());

    match copy_file_to_rice_directory(&file_path, &symlink_path) {
        Ok(_) => (),
        Err(e) => panic!("Failed to copy file: {}", e),
    }

    let symlink = json::Symlink::new(
        file_index,
        file_path
            .join(symlink_path.split('/').last().unwrap())
            .to_string_lossy()
            .to_string(),
    );
    data.rices[rice_index].symlinks.push(symlink);

    match json::json_write(&data) {
        Ok(_) => println!(
            "Symlink for file '{}' in rice '{}' successfully added.",
            file_id, rice_id
        ),
        Err(e) => eprintln!("Failed to add symlink: {}", e),
    }
}

pub fn list(rice_id: String) {
    let data: json::Data = json::read_json();

    let rice = data
        .rices
        .iter()
        .find(|r| r.id == rice_id)
        .expect("Rice ID not found");

    println!("----- Symlinks for Rice: {} -----", rice.id);

    for symlink in &rice.symlinks {
        let file = &data.files[symlink.file];
        println!("File ID: {}, Symlink Path: {}", file.id, symlink.path);
    }
}

pub fn remove(rice_id: String, file_id: String) {
    let mut data: json::Data = json::read_json();

    let rice_index = data
        .rices
        .iter()
        .position(|r| r.id == rice_id.clone())
        .expect("Rice ID not found");

    let symlink = data.rices[rice_index]
        .symlinks
        .iter()
        .find(|s| data.files[s.file].id == file_id.clone())
        .expect("Symlink for given File ID not found");

    std::fs::remove_file(&symlink.path).expect("Failed to remove symlink file");

    data.rices[rice_index]
        .symlinks
        .retain(|s| data.files[s.file].id != file_id);

    match json::json_write(&data) {
        Ok(_) => println!(
            "Symlink for file '{}' in rice '{}' successfully removed.",
            file_id, rice_id
        ),
        Err(e) => eprintln!("Failed to remove symlink: {}", e),
    }
}

fn copy_file_to_rice_directory(path_dest: &PathBuf, symlink_path: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(&path_dest)?;

    std::fs::copy(
        symlink_path,
        path_dest.join(symlink_path.split('/').last().unwrap()),
    )
    .expect("Failed to copy file");

    Ok(())
}
