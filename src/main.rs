use clap::Parser;

use crate::json::{json_write, read_json};

mod app;
mod change;
mod file;
mod json;
fn main() {
    let rrm = Args::parse();

    match rrm.sub_args {
        SubArgs::App { function } => {
            println!("Entrando em App");
            match function {
                app::Crud::Add { name } => app::add(name),
                app::Crud::Delete { name } => app::delete(name),
                app::Crud::Update { name, new_name } => app::update(name, new_name),
                app::Crud::List => app::list(),
            }
        }
        SubArgs::File { function, app_name } => {
            println!("Entrando em File");
            match function {
                file::Crud::Add { path } => file::add(app_name, path),
                file::Crud::Update { path, new_path } => file::update(app_name, path, new_path),
                file::Crud::Delete { path } => file::delete(app_name, path),
                file::Crud::List => file::list(),
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
        function: app::Crud,
    },
    File {
        #[command(subcommand)]
        function: file::Crud,
        #[arg(short, long = "app-name")]
        app_name: String,
    },
    Change {
        rice: u32,
    },
}
