use clap::Parser;

use crate::json::{json_write, read_json};

mod json;
fn main() {
    let rrm = Args::parse();

    match rrm.sub_args {
        SubArgs::App { function } => {
            println!("Entrando em App");
            match function {
                Crud::Add { name } => {
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
                Crud::Delete { name } => {
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
                Crud::Update { name, new_name } => {
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
                Crud::List => {
                    println!("Lista de Apps");
                    let data = read_json();
                    for app in data.apps {
                        println!("{}", app.name);
                    }
                }
            }
        }
        _ => {}
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
        function: Crud,
    },
    File,
    Change,
    List,
}

#[derive(clap::Subcommand)]
enum Crud {
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
