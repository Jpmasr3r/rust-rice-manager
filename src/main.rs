use clap::Parser;

mod app;
mod file;
mod json;
mod rice;
fn main() {
    match json::Config::create_config_dir() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to create config directory: {}", e);
            return;
        }
    }

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
                file::Crud::Add { path, id } => file::add(app_name, path, id),
                file::Crud::Update { new_path, id } => file::update(app_name, new_path, id),
                file::Crud::Delete { id } => file::delete(app_name, id),
                file::Crud::List => file::list(),
            }
        }
        SubArgs::Rice { function } => {
            println!("Entrando em rice");
            match function {
                rice::CrudRice::Add { id } => {
                    rice::add_rice(id);
                }
                rice::CrudRice::Delete { id } => {
                    rice::delete_rice(id);
                }
                rice::CrudRice::List => {
                    rice::list_rice();
                }
                rice::CrudRice::Update { id, new_id } => {
                    rice::update_rice(id, new_id);
                }
                rice::CrudRice::File { function } => match function {
                    rice::CrudRiceFile::Add {
                        file_id,
                        id,
                        rice_id,
                    } => {
                        //rice_id: String, id: String, file_id: String
                        rice::add_rice_file(rice_id, id, file_id);
                    }
                    rice::CrudRiceFile::Delete {
                        file_id,
                        rice_id,
                        id,
                    } => {
                        //rice_id: String, id: String, file_id: String
                        rice::delete_rice_file(rice_id, id, file_id);
                    }
                    rice::CrudRiceFile::List { rice_id } => {
                        //rice_id: String
                        rice::list_rice_file(rice_id);
                    }
                    rice::CrudRiceFile::Update {
                        rice_id,
                        id,
                        file_id,
                        new_id,
                    } => {
                        //rice_id: String, id: String, file_id: String, new_id: String
                        rice::update_rice_file(rice_id, id, file_id, new_id);
                    }
                },
            }
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
    Rice {
        #[command(subcommand)]
        function: rice::CrudRice,
    },
}
