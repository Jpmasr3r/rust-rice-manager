use std::path::PathBuf;

use crate::json;

#[derive(clap::Subcommand)]
pub enum Crud {
    Add {
        #[arg(long = "rice", short = 'r')]
        rice_id: String,
        #[arg(long = "file", short = 'f')]
        file_id: String,
        #[arg(long = "path", short = 'p')]
        symlink_path: String,
    },
    List {
        #[arg(long = "rice", short = 'r')]
        rice_id: String,
    },
    Remove {
        #[arg(long = "rice", short = 'r')]
        rice_id: String,
        #[arg(long = "file", short = 'f')]
        file_id: String,
    },
    Update {
        #[arg(long = "rice", short = 'r')]
        rice_id: String,
        #[arg(long = "file", short = 'f')]
        file_id: String,
        #[arg(long = "new_path", short = 'p')]
        symlink_path: String,
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
        Ok(_) => {
            println!("----- Adding Symlink -----");
            println!(
                "Symlink for file '{}' in rice '{}' successfully added.",
                file_id, rice_id
            )
        }
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

    println!("----- List Symlinks for Rice: {} -----", rice.id);
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

    let mut current: PathBuf = PathBuf::from(&symlink.path);
    for _ in 0..3 {
        current = current
            .parent()
            .expect("Failed to get parent directory")
            .to_path_buf();
        match std::fs::remove_dir(&current) {
            Ok(_) => {}
            Err(_) => break,
        }
    }

    data.rices[rice_index]
        .symlinks
        .retain(|s| data.files[s.file].id != file_id);

    match json::json_write(&data) {
        Ok(_) => {
            println!("----- Removing Symlink -----");
            println!(
                "Symlink for file '{}' in rice '{}' successfully removed.",
                file_id, rice_id
            )
        }
        Err(e) => eprintln!("Failed to remove symlink: {}", e),
    }
}

pub fn update(rice_id: String, file_id: String, new_path: String) {
    let mut data: json::Data = json::read_json();

    let rice_index = data
        .rices
        .iter()
        .position(|r| r.id == rice_id.clone())
        .expect("Rice ID not found");

    let symlink = data.rices[rice_index]
        .symlinks
        .iter_mut()
        .find(|s| data.files[s.file].id == file_id.clone())
        .expect("Symlink for given File ID not found");

    std::fs::remove_file(&symlink.path).expect("Failed to remove existing symlink file");

    let file_path = json::get_config_file_path()
        .join("rices")
        .join(rice_id.clone())
        .join(file_id.clone());

    match copy_file_to_rice_directory(&file_path, &new_path) {
        Ok(_) => (),
        Err(e) => panic!("Failed to copy file: {}", e),
    }

    symlink.path = file_path
        .join(new_path.split('/').last().unwrap())
        .to_string_lossy()
        .to_string();

    match json::json_write(&data) {
        Ok(_) => {
            println!("----- Updating Symlink -----");
            println!(
                "Symlink for file '{}' in rice '{}' successfully updated.",
                file_id, rice_id
            )
        }
        Err(e) => eprintln!("Failed to update symlink: {}", e),
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
