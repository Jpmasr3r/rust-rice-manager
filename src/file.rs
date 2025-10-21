use crate::json;

#[derive(clap::Subcommand)]
pub enum Crud {
    Add {
        #[arg(long = "path", short = 'p')]
        path: String,
        #[arg(long = "id", short = 'i')]
        id: String,
    },
}

pub fn add(id: String, path: String) {
    let mut data: json::Data = json::read_json();

    let file = json::File::new(path, id);
    data.files.push(file);

    json::json_write(&data).unwrap();
}
