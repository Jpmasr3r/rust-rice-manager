use crate::json;

#[derive(clap::Subcommand)]
pub enum CrudRice {
    Add {
        #[arg(long = "id", short = 'i')]
        id: String,
    },
    List,
    Update {
        #[arg(long = "id", short = 'i')]
        id: String,
        #[arg(long = "new-id", short = 'n')]
        new_id: String,
    },
    Delete {
        #[arg(long = "id", short = 'i')]
        id: String,
    },
    File {
        #[command(subcommand)]
        function: CrudRiceFile,
    },
}

#[derive(clap::Subcommand)]
pub enum CrudRiceFile {
    //rice_id: String, id: String, file_id: String
    Add {
        #[arg(long = "rice-id", short = 'r')]
        rice_id: String,
        #[arg(long = "id", short = 'i')]
        id: String,
        #[arg(long = "file-id", short = 'f')]
        file_id: String,
    },
    //rice_id: String
    List {
        #[arg(long = "rice-id", short = 'r')]
        rice_id: String,
    },
    //rice_id: String, id: String, file_id: String, new_id: String
    Update {
        #[arg(long = "rice-id", short = 'r')]
        rice_id: String,
        #[arg(long = "id", short = 'i')]
        id: String,
        #[arg(long = "file-id", short = 'f')]
        file_id: String,
        #[arg(long = "new-id", short = 'n')]
        new_id: String,
    },
    //rice_id: String, id: String, file_id: String
    Delete {
        #[arg(long = "rice-id", short = 'r')]
        rice_id: String,
        #[arg(long = "id", short = 'i')]
        id: String,
        #[arg(long = "file-id", short = 'f')]
        file_id: String,
    },
}

//rice dirs functions
pub fn add_rice(id: String) {
    let mut data = json::read_json();

    if !data.rices.iter().any(|rice| rice.id == id) {
        let rice = json::Rice::new(id.clone());
        match rice.create_rice_dir() {
            Ok(_) => {
                data.rices.push(rice);

                match json::json_write(&data) {
                    Ok(_) => println!("Rice {} adicionado com sucesso", id),
                    Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
                };
            }
            Err(e) => eprintln!("Erro ao criar diretório para rice {}: {}", id, e),
        }
    } else {
        println!("Rice {} já existente", id);
    }
}
pub fn delete_rice(id: String) {
    let mut data = json::read_json();

    if let Some((rice_index, rice)) = data
        .rices
        .iter()
        .enumerate()
        .find(|(_, rice)| rice.id == id)
    {
        match json::Rice::delete_rice_dir(&rice) {
            Ok(_) => {
                data.rices.remove(rice_index);

                match json::json_write(&data) {
                    Ok(_) => println!("Rice {} removido com sucesso", id),
                    Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
                };
            }
            Err(e) => eprintln!("Erro ao remover diretório do rice {}: {}", id, e),
        }
    } else {
        println!("Rice {} não existente", id);
    }
}

pub fn update_rice(id: String, new_id: String) {
    let mut data = json::read_json();

    if let Some(rice) = data.rices.iter_mut().find(|rice| rice.id == id) {
        match json::Rice::rename_rice_dir(rice, &new_id) {
            Ok(_) => {
                rice.id = new_id.clone();

                match json::json_write(&data) {
                    Ok(_) => println!("Rice {} atualizado para {} com sucesso", id, new_id),
                    Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
                };
            }
            Err(e) => {
                eprintln!("Erro ao renomear diretório do rice {}: {}", id, e);
                return;
            }
        }
    } else {
        println!("Rice {} não existente", id);
    }
}

pub fn list_rice() {
    let data = json::read_json();

    for rice in data.rices {
        println!("Rice => {}", rice.id);
    }
}

//symlinks functions
pub fn add_rice_file(rice_id: String, id: String, file_id: String) {
    let mut data = json::read_json();

    if let Some(rice) = data.rices.iter_mut().find(|rice| rice.id == rice_id) {
        if let Some(file_index) = data.files.iter().position(|file| file.id == file_id) {
            if !rice
                .symlinks
                .iter()
                .any(|symlink| symlink.id == id && symlink.file_index == file_index)
            {
                rice.symlinks
                    .push(json::Symlink::new(file_index, id.clone()));

                match json::json_write(&data) {
                    Ok(_) => println!("Symlink {} adicionado para o Rice {}", id, rice_id),
                    Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
                };
            }
        }
    }
}
pub fn delete_rice_file(rice_id: String, id: String, file_id: String) {
    let mut data = json::read_json();

    if let Some(rice) = data.rices.iter_mut().find(|rice| rice.id == rice_id) {
        if let Some(file_index) = data.files.iter().position(|file| file.id == file_id) {
            if let Some(symlink_index) = rice
                .symlinks
                .iter()
                .position(|symlink| symlink.id == id && symlink.file_index == file_index)
            {
                rice.symlinks.remove(symlink_index);

                match json::json_write(&data) {
                    Ok(_) => println!("Symlink {} removido do Rice {}", id, rice_id),
                    Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
                };
            }
        }
    }
}
pub fn update_rice_file(rice_id: String, id: String, file_id: String, new_id: String) {
    let mut data = json::read_json();

    if let Some(rice) = data.rices.iter_mut().find(|rice| rice.id == rice_id) {
        if let Some(file_index) = data.files.iter().position(|file| file.id == file_id) {
            if let Some(symlink) = rice
                .symlinks
                .iter_mut()
                .find(|symlink| symlink.id == id && symlink.file_index == file_index)
            {
                symlink.id = new_id;

                match json::json_write(&data) {
                    Ok(_) => println!("Symlink {} atualizado para o Rice {}", id, rice_id),
                    Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
                };
            }
        }
    }
}

pub fn list_rice_file(rice_id: String) {
    let data = json::read_json();

    if let Some(rice) = data.rices.iter().find(|rice| rice.id == rice_id) {
        println!("Rice => {}", rice.id);
        for symlink in &rice.symlinks {
            if let Some(file) = data.files.get(symlink.file_index) {
                println!("Symlink => {} -> {}", symlink.id, file.id);
            }
        }
    }
}
