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
    Update {
        #[arg(long = "new-path")]
        new_path: String,
        #[arg(long = "id", short = 'i')]
        id: String,
    },
    Delete {
        #[arg(long = "id", short = 'i')]
        id: String,
    },
}

pub fn add(app_name: String, path: String, id: String) {
    let mut data = json::read_json();

    if let Some(index) = data.apps.iter().position(|app| app.name == app_name) {
        data.files.push(json::File::new(path.clone(), index, id));
        match json::json_write(&data) {
            Ok(_) => println!("Novo File {} adicionado", path),
            Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
        };
    } else {
        println!("App {} não encontrado", app_name);
    }
}

pub fn update(app_name: String, new_path: String, id: String) {
    let mut data = json::read_json();

    if let Some(index) = data.apps.iter().position(|app| app.name == app_name) {
        if let Some(file) = data
            .files
            .iter_mut()
            .find(|file| file.id == id && file.app_index == index)
        {
            file.path = new_path.clone();
            match json::json_write(&data) {
                Ok(_) => println!("File {} atualizado", id),
                Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
            };
        } else {
            println!("File {} não encontrado para o App {}", id, app_name);
        }
    } else {
        println!("App {} não encontrado", app_name);
    }
}

pub fn delete(app_name: String, id: String) {
    let mut data = json::read_json();

    if let Some(app_index) = data.apps.iter().position(|app| app.name == app_name) {
        if let Some(file_index) = data
            .files
            .iter()
            .position(|file| file.id == id && file.app_index == app_index)
        {
            data.files.remove(file_index);

            match json::json_write(&data) {
                Ok(_) => println!("File {} removido do App {}", id, app_name),
                Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
            };
        } else {
            println!("File {} não encontrado para o App {}", id, app_name);
        }
    } else {
        println!("App {} não encontrado", app_name);
    }
}

pub fn list() {
    let data = json::read_json();

    println!("Lista de Files");
    for file in data.files {
        println!(
            "Path => {}  --- Id => {} ---  App => {}",
            file.path, file.id, data.apps[file.app_index].name
        )
    }
}
