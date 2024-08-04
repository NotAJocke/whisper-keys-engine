use anyhow::{Context, Ok, Result};
use clap::Parser;
use home::home_dir;
use std::{env::consts::OS, fs, path::Path};

use whisper_keys_engine::{
    cli, mechvibes,
    program_args::{self, SubCommands},
    server, APP_NAME,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init()?;

    let args = program_args::Args::parse();

    match args.subcommand {
        None | Some(SubCommands::Run) => cli::run()?,
        Some(SubCommands::Server) => server::serve().await,
        Some(SubCommands::Translate { path }) => {
            mechvibes::translate_config(&path)?;

            println!("Config translated at location: {path}");
        }
        Some(SubCommands::Generate { name, path }) => {
            let pack_path = Path::new(&path).join(name);
            fs::create_dir_all(&pack_path).context("Failed to create the pack directory")?;
            let template = include_str!("config_template.json5");
            fs::write(pack_path.join("config.json5"), template)?;

            println!(
                "Generated template at location: {}",
                pack_path.to_str().unwrap()
            );
        }
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
