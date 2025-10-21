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
}

pub fn add(rice_id: String, file_id: String, symlink_path: String) {
    let mut data: json::Data = json::read_json();

    let rice_index = data
        .rices
        .iter()
        .position(|r| r.id == rice_id)
        .expect("Rice ID not found");

    let file_index = data
        .files
        .iter()
        .position(|f| f.id == file_id)
        .expect("File ID not found");

    let file_path = json::get_config_file_path()
        .join("rices")
        .join(rice_id)
        .join(file_id);

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

    json::json_write(&data).unwrap();
}

fn copy_file_to_rice_directory(path_dest: &PathBuf, symlink_path: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(&path_dest)?;

    std::fs::copy(
        symlink_path,
        path_dest.join(symlink_path.split('/').last().unwrap()),
    )?;

    Ok(())
}
