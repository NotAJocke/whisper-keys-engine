use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU16, Ordering},
        mpsc, Arc, RwLock,
    },
    thread,
};

use crate::{
    keylogger,
    packs::{self},
    player, APP_NAME,
};
use anyhow::{Context, Result};
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};
use home::home_dir;
use rodio::OutputStream;

fn ask_for_pack(packs_folder: &PathBuf) -> Result<String> {
    let available_packs = packs::list_available(packs_folder).context("Couln't get local packs")?;

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

    let (tx, rx) = mpsc::channel();
    let sound_level = Arc::new(AtomicU16::new(default_volume));
    let current_pack = Arc::new(RwLock::new(pack));

    let (_stream, stream_handle) =
        OutputStream::try_default().context("Couln't find an audio output channel")?;

    keylogger::listen(tx)?;

    let sound_level_clone = Arc::clone(&sound_level);
    let current_pack_clone = Arc::clone(&current_pack);
    thread::spawn(move || loop {
        println!();
        let action = Select::with_theme(&ColorfulTheme::default())
            .items(&["Change volume", "Change pack"])
            .default(0)
            .interact_on(&Term::stderr())
            .unwrap_or(0);

        if action == 0 {
            let current_sound = sound_level_clone.load(Ordering::Relaxed);
            let input: u16 = Input::new()
                .allow_empty(true)
                .with_prompt("Enter the new volume")
                .default(current_sound)
                .show_default(false)
                .interact_text()
                .unwrap();

            if input != current_sound {
                sound_level_clone.store(input, Ordering::Relaxed);
            }
        } else {
            let mut pack_lock = current_pack_clone.write().unwrap();
            let current_pack_name = pack_lock.name.as_str();
            let pack_name = ask_for_pack(&packs_folder).unwrap();

            if pack_name != current_pack_name {
                let pack = packs::load_pack(&packs_folder, &pack_name).unwrap();
                sound_level_clone.store(pack.keys_default_volume, Ordering::Relaxed);
                *pack_lock = pack;
            }
        }

        Term::stdout().clear_screen().unwrap();
        println!(
            "Pack selected: {}\nVolume set to {}%",
            current_pack_clone.read().unwrap().name,
            sound_level_clone.load(Ordering::Relaxed)
        );
    });

    let sound_level_clone2 = Arc::clone(&sound_level);
    let current_pack_clone2 = Arc::clone(&current_pack);
    for msg in &rx {
        let pack_lock = current_pack_clone2.read().unwrap();
        let buf = pack_lock.keys.get(&msg).unwrap_or_else(|| {
            pack_lock
                .keys
                .get("unknown")
                .context("Couln't get proprety 'unknown' in config file.")
                .unwrap()
        });
        player::play_sound(
            &stream_handle,
            buf,
            sound_level_clone2.load(Ordering::Relaxed),
        )?;
    }

    Ok(())
}
