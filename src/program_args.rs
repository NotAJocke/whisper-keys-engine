use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Option<SubCommands>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommands {
    #[command(about = "Start the application (default)")]
    Run,

    #[command(about = "Translate a mechvibes pack to a whisper one")]
    Translate {
        #[arg(help = "Folder's pack path")]
        path: String,
    },
    #[command(about = "Generate the file and folder structure to create a pack")]
    Generate {
        #[arg(help = "Where to generate folder")]
        path: String,
    },
    #[command(about = "Start the grpc server for UIs")]
    Server,
}
