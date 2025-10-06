use crate::json;

#[derive(clap::Subcommand)]
pub enum Crud {
    Add {
        #[arg(long = "name", short = 'n')]
        name: String,
    },
    List,
    Update {
        #[arg(long = "name", short = 'n')]
        name: String,
        #[arg(long = "new-name")]
        new_name: String,
    },
    Delete {
        #[arg(long = "name", short = 'n')]
        name: String,
    },
}

pub fn add(name: String) {
    let mut data = json::read_json();

    if data.apps.iter().any(|x| x.name == name) {
        println!("App já existe!");
        return;
    }

    data.apps.push(json::App::from(name.clone()));
    match json::json_write(data) {
        Ok(_) => println!("Novo App {} adicionado", name),
        Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
    };
}

pub fn delete(name: String) {
    let mut data = json::read_json();

    if !data.apps.iter().any(|x| x.name == name) {
        println!("App não existe!");
        return;
    }

    if let Some(pos) = data.apps.iter().position(|x| x.name == name) {
        data.apps.remove(pos);
    }
    match json::json_write(data) {
        Ok(_) => println!("App {} removido", name),
        Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
    };
}

pub fn update(name: String, new_name: String) {
    let mut data = json::read_json();

    if !data.apps.iter().any(|x| x.name == name) {
        println!("App original não existe!");
        return;
    }

    if let Some(pos) = data.apps.iter().position(|x| x.name == name) {
        data.apps[pos].name = new_name.clone();
    }
    match json::json_write(data) {
        Ok(_) => println!("App {} atualizado para {}", name, new_name),
        Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
    };
}

pub fn list() {
    println!("Lista de Apps");
    let data = json::read_json();
    for app in data.apps {
        println!("{}", app.name);
    }
}
