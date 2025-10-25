use clap::Parser;

mod cli;
mod file;
mod json;
mod rice;
mod symlink;

fn main() {
    let rrm = cli::Args::parse();

    json::set_dev_mode(rrm.dev);

    match rrm.sub_args {
        cli::SubArgs::File { crud } => match crud {
            file::Crud::Add { id, path } => {
                file::add(id, path);
            }
            file::Crud::List => {
                file::list();
            }
            file::Crud::Remove { id } => {
                file::remove(id);
            }
            file::Crud::Update {
                id,
                new_id,
                new_path,
            } => {
                file::update(id, new_id, new_path);
            }
        },
        cli::SubArgs::Rice { crud } => match crud {
            rice::Crud::Add { id } => {
                rice::add(id);
            }
            rice::Crud::List => {
                rice::list();
            }
            rice::Crud::Remove { id } => {
                rice::remove(id);
            }
            rice::Crud::File { crud } => match crud {
                symlink::Crud::Add {
                    rice_id,
                    file_id,
                    symlink_path,
                } => {
                    symlink::add(rice_id, file_id, symlink_path);
                }
                symlink::Crud::List { rice_id } => {
                    symlink::list(rice_id);
                }
                symlink::Crud::Remove { rice_id, file_id } => {
                    symlink::remove(rice_id, file_id);
                }
                symlink::Crud::Update {
                    rice_id,
                    file_id,
                    symlink_path,
                } => {
                    symlink::update(rice_id, file_id, symlink_path);
                }
            },
            rice::Crud::Change { id } => {
                rice::change(id);
            }
            rice::Crud::Update { id, new_id } => {
                rice::update(id, new_id);
            }
        },
    }
}
