use anyhow::{Context, Ok, Result};
use clap::Parser;
use home::home_dir;
use std::{env::consts::OS, fs, path::Path};

use whisper_keys_engine::{program_args::SubCommands, APP_NAME, commands, program_args, server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init()?;

    let args = program_args::Args::parse();

    match args.subcommand {
        None | Some(SubCommands::Run) => commands::run()?,
        Some(SubCommands::Server) => server::serve().await,
        Some(SubCommands::Translate { path }) => commands::translate_config(&path)?,
        Some(SubCommands::Generate { path }) => commands::generate_template(&path)?,
    }

    Ok(())
}

fn init() -> Result<()> {
    let local_dir = home_dir().context("Couldn't find home directory")?;
    let path = match OS {
        "windows" => Path::new(&local_dir)
            .join("AppData")
            .join("Roaming")
            .join(APP_NAME),
        _ => Path::new(&local_dir).join(APP_NAME),
    };

    if fs::read_dir(&path).is_err() {
        fs::create_dir_all(&path)?;
    }

    Ok(())
}
