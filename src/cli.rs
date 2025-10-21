#[derive(clap::Parser)]
#[command(name = "rrm")]
#[command(about = "RRM is a CLI rice manager")]
#[command(version = "0.0.1")]
#[command(author = "Jpmasr3r")]
pub struct Args {
    #[command(subcommand)]
    pub sub_args: SubArgs,
}

#[derive(clap::Subcommand)]
pub enum SubArgs {
    File {
        #[command(subcommand)]
        crud: crate::file::Crud,
    },
    Rice {
        #[command(subcommand)]
        crud: crate::rice::Crud,
    },
}
