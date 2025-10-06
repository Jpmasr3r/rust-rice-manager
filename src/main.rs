use clap::Parser;

use crate::json::{json_write, read_json};

mod json;
fn main() {
    let rrm = Args::parse();

    match rrm.sub_args {
        SubArgs::App { function } => {
            println!("Entrando em App");
            match function {
                CrudApp::Add { name } => {
                    let mut data = read_json();

                    if data.apps.iter().any(|x| x.name == name) {
                        println!("App já existe!");
                        return;
                    }

                    data.apps.push(json::App::from(name.clone()));
                    match json_write(data) {
                        Ok(_) => println!("Novo App {} adicionado", name),
                        Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
                    };
                }
                CrudApp::Delete { name } => {
                    let mut data = read_json();

                    if !data.apps.iter().any(|x| x.name == name) {
                        println!("App não existe!");
                        return;
                    }

                    if let Some(pos) = data.apps.iter().position(|x| x.name == name) {
                        data.apps.remove(pos);
                    }
                    match json_write(data) {
                        Ok(_) => println!("App {} removido", name),
                        Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
                    };
                }
                CrudApp::Update { name, new_name } => {
                    let mut data = read_json();

                    if !data.apps.iter().any(|x| x.name == name) {
                        println!("App original não existe!");
                        return;
                    }

                    if let Some(pos) = data.apps.iter().position(|x| x.name == name) {
                        data.apps[pos].name = new_name.clone();
                    }
                    match json_write(data) {
                        Ok(_) => println!("App {} atualizado para {}", name, new_name),
                        Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
                    };
                }
                CrudApp::List => {
                    println!("Lista de Apps");
                    let data = read_json();
                    for app in data.apps {
                        println!("{}", app.name);
                    }
                }
            }
        }
        SubArgs::File { function, app_name } => {
            println!("Entrando em File");
            match function {
                CrudFile::Add { path } => {
                    let mut data = read_json();

                    if let Some(index) = data.apps.iter().position(|app| app.name == app_name) {
                        data.files.push(json::File::from(path.clone(), index));
                        match json_write(data) {
                            Ok(_) => println!("Novo File {} adicionado", path),
                            Err(e) => eprintln!("Erro ao salvar JSON: {}", e),
                        };
                    } else {
                        println!("App {} não encontrado", app_name);
                    }
                }
                CrudFile::Update { path, new_path } => {
                    let mut data = read_json();

                    if let Some(index) = data.apps.iter().position(|app| app.name == app_name) {
                        if let Some(file) = data
                            .files
                            .iter_mut()
                            .find(|file| file.path == path && file.app_index == index)
                        {
                            file.path = new_path.clone();
                            match json_write(data) {
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
                CrudFile::Delete { path } => {
                    let mut data = read_json();

                    if let Some(app_index) = data.apps.iter().position(|app| app.name == app_name) {
                        if let Some(file_index) = data
                            .files
                            .iter()
                            .position(|file| file.path == path && file.app_index == app_index)
                        {
                            data.files.remove(file_index);

                            match json_write(data) {
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
                CrudFile::List => {
                    let data = read_json();

                    println!("Lista de Files");
                    for file in data.files {
                        println!(
                            "Path => {}  ---  App => {}",
                            file.path, data.apps[file.app_index].name
                        )
                    }
                }
            }
        }
        SubArgs::Change { rice } => {
            println!("Entrando em change");
        }
    }
}

#[derive(clap::Parser)]
#[command(name = "rrm")]
#[command(about = "RRM is a CLI rice manager")]
#[command(version = "0.0.1")]
#[command(author = "Jpmasr3r")]
struct Args {
    #[command(subcommand)]
    sub_args: SubArgs,
}

#[derive(clap::Subcommand)]
enum SubArgs {
    App {
        #[command(subcommand)]
        function: CrudApp,
    },
    File {
        #[command(subcommand)]
        function: CrudFile,
        #[arg(short, long = "app-name")]
        app_name: String,
    },
    Change {
        rice: u32,
    },
}

#[derive(clap::Subcommand)]
enum CrudApp {
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

#[derive(clap::Subcommand)]
enum CrudFile {
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
