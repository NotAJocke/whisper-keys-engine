use anyhow::{Context, Ok, Result};
use home::home_dir;
use std::{
    env::{self, consts::OS},
    fs,
    path::Path,
};

use whisper_keys_engine::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>().split_off(1);

    init()?;

    if !args.is_empty() {
        match args[0].as_str() {
            "--translate" | "-t" => {
                if args.len() < 2 {
                    println!("Please specify the path to the pack folder.");
                    return Ok(());
                }
                commands::translate_config(&args[1])?;
            }
            "--generate-template" | "-g" => commands::generate_template("./")?,
            "-v" | "--version" => println!("{} v{}", APP_NAME, env!("CARGO_PKG_VERSION")),
            "--rpc" | "--grpc" => server::serve().await.unwrap(),
            _ => commands::run()?,
        }
    } else {
        commands::run()?;
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
