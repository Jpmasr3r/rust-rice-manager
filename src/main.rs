use clap::Parser;

mod cli;
mod file;
mod json;
mod rice;
mod symlink;

fn main() {
    let rrm = cli::Args::parse();

    match rrm.sub_args {
        cli::SubArgs::File { crud } => match crud {
            file::Crud::Add { id, path } => {
                file::add(id, path);
            }
        },
        cli::SubArgs::Rice { crud } => match crud {
            rice::Crud::Add { id } => {
                rice::add(id);
            }
            rice::Crud::File { crud } => match crud {
                symlink::Crud::Add {
                    rice_id,
                    file_id,
                    symlink_path,
                } => {
                    symlink::add(rice_id, file_id, symlink_path);
                }
            },
            rice::Crud::Change { id } => {
                rice::change(id);
            }
        },
    }
}
