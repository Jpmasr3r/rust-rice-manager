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
        Ok(_) => println!("File {} successfully added.", id),
        Err(e) => eprintln!("Failed to add file: {}", e),
    }
}

pub fn list() {
    let data: json::Data = json::read_json();

    println!("----- Files -----");

    for file in data.files {
        println!("ID: {}, Path: {}", file.id, file.path);
    }
}

pub fn remove(id: String) {
    let mut data: json::Data = json::read_json();

    let file_index = data
        .files
        .iter()
        .position(|f| f.id == id)
        .expect("File ID not found.");
    let initial_len = data.files.len();

    data.files.retain(|f| f.id != id);
    data.rices.iter_mut().for_each(|rice| {
        rice.symlinks.retain(|symlink| symlink.file != file_index);
    });

    if data.files.len() == initial_len {
        eprintln!("No file found with id '{}'.", id);
        return;
    }

    match json::json_write(&data) {
        Ok(_) => println!("File {} successfully removed.", id),
        Err(e) => eprintln!("Failed to remove file: {}", e),
    }
}
