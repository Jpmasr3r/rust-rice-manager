use crate::json;

#[derive(clap::Subcommand)]
pub enum Crud {
    Add {
        #[arg(long = "path", short = 'p')]
        path: String,
    },
    List,
    Update {
        #[arg(long = "path", short = 'p')]
        path: String,
        #[arg(long = "new-path")]
        new_path: String,
    },
    Delete {
        #[arg(long = "path", short = 'p')]
        path: String,
    },
}

pub fn add(app_name: String, path: String) {
    let mut data = json::read_json();

    if let Some(index) = data.apps.iter().position(|app| app.name == app_name) {
        data.files.push(json::File::from(path.clone(), index));
        match json::json_write(data) {
            Ok(_) => println!("Novo File {} adicionado", path),
            Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
        };
    } else {
        println!("App {} não encontrado", app_name);
    }
}

pub fn update(app_name: String, path: String, new_path: String) {
    let mut data = json::read_json();

    if let Some(index) = data.apps.iter().position(|app| app.name == app_name) {
        if let Some(file) = data
            .files
            .iter_mut()
            .find(|file| file.path == path && file.app_index == index)
        {
            file.path = new_path.clone();
            match json::json_write(data) {
                Ok(_) => println!("File {} atualizado", path),
                Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
            };
        } else {
            println!("File {} não encontrado para o App {}", path, app_name);
        }
    } else {
        println!("App {} não encontrado", app_name);
    }
}

pub fn delete(app_name: String, path: String) {
    let mut data = json::read_json();

    if let Some(app_index) = data.apps.iter().position(|app| app.name == app_name) {
        if let Some(file_index) = data
            .files
            .iter()
            .position(|file| file.path == path && file.app_index == app_index)
        {
            data.files.remove(file_index);

            match json::json_write(data) {
                Ok(_) => println!("File {} removido do App {}", path, app_name),
                Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
            };
        } else {
            println!("File {} não encontrado para o App {}", path, app_name);
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
            "Path => {}  ---  App => {}",
            file.path, data.apps[file.app_index].name
        )
    }
}
