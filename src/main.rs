use anyhow::Context;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use rodio::{self, OutputStream};
use std::sync::mpsc;

mod keylogger;
mod packs;
mod player;

fn main() -> anyhow::Result<()> {
    let available_packs = packs::list_available("./assets")?;
    let pack_idx = Select::with_theme(&ColorfulTheme::default())
        .items(&available_packs)
        .default(0)
        .interact_on(&Term::stderr())
        .unwrap_or(0);
    let pack = available_packs[pack_idx].clone();

    let config = packs::load_pack("./assets", &pack).context("Selected pack couldn't be loaded")?;
    let (tx, rx) = mpsc::channel();
    let (_stream, stream_handle) =
        OutputStream::try_default().context("Couln't find an audio output channel")?;

    keylogger::listen(tx)?;

    for msg in rx.iter() {
        dbg!(&msg);
        let buf = config
            .get(&msg)
            .unwrap_or_else(|| config.get("unknown").unwrap());
        player::play_sound(stream_handle.clone(), buf.clone())?;
    }

    Ok(())
}
