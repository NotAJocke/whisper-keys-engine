use std::{
    fs,
    path::Path,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::{keylogger, mechvibes, packs, player, APP_NAME};
use anyhow::{Context, Ok, Result};
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};
use home::home_dir;
use rodio::OutputStream;

pub fn run() -> Result<()> {
    let available_packs = packs::list_available_local().context("Couln't get local packs")?;
    let home_dir = home_dir().context("Couldn't find home directory")?;
    let packs_folder = Path::new(&home_dir).join(APP_NAME);

    if available_packs.is_empty() {
        println!("No custom pack found.");
        return Ok(());
    }

    let pack_idx = Select::with_theme(&ColorfulTheme::default())
        .items(&available_packs)
        .default(0)
        .interact_on(&Term::stderr())
        .unwrap_or(0);
    let pack = available_packs[pack_idx].clone();

    let config =
        packs::load_pack(packs_folder, &pack).context("Selected pack couldn't be loaded")?;
    let (tx, rx) = mpsc::channel();
    let (_stream, stream_handle) =
        OutputStream::try_default().context("Couln't find an audio output channel")?;

    keylogger::listen(tx)?;

    let sound_level: Arc<Mutex<f32>> = Arc::new(Mutex::new(config.keys_default_volume));
    let sound_level_lock = Arc::clone(&sound_level);
    println!("Volume set to {}%", config.keys_default_volume);
    thread::spawn(move || loop {
        let new_sound_level: f32 = Input::new()
            .with_prompt("Enter the new volume:")
            .interact_text()
            .unwrap();
        println!("Volume set to {}%", new_sound_level);
        *sound_level_lock.lock().unwrap() = new_sound_level;
    });

    for msg in rx.iter() {
        if cfg!(debug_assertions) {
            dbg!(&msg);
        }
        let buf = config.keys.get(&msg).unwrap_or_else(|| {
            config
                .keys
                .get("unknown")
                .context("Couln't get proprety 'unknown' in config file.")
                .unwrap()
        });
        player::play_sound(
            stream_handle.clone(),
            buf.clone(),
            *sound_level.lock().unwrap(),
        )?;
    }

    Ok(())
}

pub fn generate_template(path: &str) -> Result<()> {
    let pack_path = Path::new(path).join("Pack_Name");
    fs::create_dir_all(&pack_path)?;
    let template = include_str!("config_template.json");
    fs::write(pack_path.join("config.json"), template)?;

    Ok(())
}

pub fn translate_config(path: &str) -> Result<()> {
    mechvibes::translate_config(path)?;

    println!("Config translated at location: {}", path);

    Ok(())
}
