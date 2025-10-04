use clap::Parser;
fn main() {
    let rrm = Args::parse();

    match rrm.sub_args {
        SubArgs::App { function } => {
            println!("Modificando App");
            match function {
                Crud::Add { name } => {
                    println!("Novo App adicionado => {}", name);
                }
                Crud::Delete { name } => {
                    println!("App removido => {}", name);
                }
                Crud::Update { name, new_name } => {
                    println!("App atualizado => {} para {}", name, new_name);
                }
                Crud::List => {
                    println!("Lista de apps => app1,app2,app3, etc...")
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
