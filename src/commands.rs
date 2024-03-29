use std::{
    fs,
    path::{Path, PathBuf},
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::{
    keylogger, mechvibes,
    packs::{self},
    player, APP_NAME,
};
use anyhow::{Context, Result};
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};
use home::home_dir;
use rodio::OutputStream;

fn ask_for_pack(packs_folder: &PathBuf) -> Result<String> {
    let available_packs =
        packs::list_available(&packs_folder).context("Couln't get local packs")?;

    if available_packs.is_empty() {
        return Err(anyhow::anyhow!("No custom pack found."));
    }

    let pack_idx = Select::with_theme(&ColorfulTheme::default())
        .items(&available_packs)
        .default(0)
        .interact_on(&Term::stderr())
        .unwrap_or(0);
    let pack_name = available_packs[pack_idx].clone();

    Ok(pack_name)
}

pub fn run() -> Result<()> {
    let home_dir = home_dir().context("Couldn't find home directory")?;
    let packs_folder = Path::new(&home_dir).join(APP_NAME);

    let pack_name = ask_for_pack(&packs_folder)?;
    let pack =
        packs::load_pack(&packs_folder, &pack_name).context("Selected pack couldn't be loaded")?;

    let default_volume = pack.keys_default_volume;

    Term::stdout().clear_screen().unwrap();
    println!(
        "Pack loaded: {}\nVolume set to {}%",
        pack.name, default_volume
    );

    let current_pack = Arc::new(Mutex::new(pack));

    let (tx, rx) = mpsc::channel();
    let sound_level: Arc<Mutex<f32>> = Arc::new(Mutex::new(default_volume));

    let (_stream, stream_handle) =
        OutputStream::try_default().context("Couln't find an audio output channel")?;

    keylogger::listen(tx)?;

    let cloned_sound_level = Arc::clone(&sound_level);
    let cloned_current_pack = Arc::clone(&current_pack);
    thread::spawn(move || loop {
        println!();
        let action = Select::with_theme(&ColorfulTheme::default())
            .items(&["Change volume", "Change pack"])
            .default(0)
            .interact_on(&Term::stderr())
            .unwrap_or(0);

        match action {
            0 => {
                let current_sound = *cloned_sound_level.lock().unwrap();
                let input: f32 = Input::new()
                    .allow_empty(true)
                    .with_prompt("Enter the new volume")
                    .default(current_sound)
                    .show_default(false)
                    .interact_text()
                    .unwrap();

                if input != current_sound {
                    *cloned_sound_level.lock().unwrap() = input;
                }
            }
            _ => {
                let current_pack_name = &*cloned_current_pack.lock().unwrap().name;
                let pack_name = ask_for_pack(&packs_folder).unwrap();

                if pack_name != current_pack_name {
                    let pack = packs::load_pack(&packs_folder, &pack_name).unwrap();
                    *cloned_sound_level.lock().unwrap() = pack.keys_default_volume;
                    *cloned_current_pack.lock().unwrap() = pack;
                }
            }
        }
        Term::stdout().clear_screen().unwrap();
        println!(
            "Pack selected: {}",
            cloned_current_pack.lock().unwrap().name
        );
        println!("Volume set to {}%", cloned_sound_level.lock().unwrap());
    });

    for msg in rx.iter() {
        let pack_lock = current_pack.lock().unwrap();
        let buf = pack_lock.keys.get(&msg).unwrap_or_else(|| {
            pack_lock
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
    let template = include_str!("config_template.json5");
    fs::write(pack_path.join("config.json5"), template)?;

    Ok(())
}

pub fn translate_config(path: &str) -> Result<()> {
    mechvibes::translate_config(path)?;

    println!("Config translated at location: {}", path);

    Ok(())
}
