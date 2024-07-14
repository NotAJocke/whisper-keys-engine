use anyhow::{Context, Result};
use std::{fs::File, io::BufReader};

use rodio::{source::Buffered, Decoder, OutputStreamHandle, Sink};

/// Will return an error if it cannot create an audio sink
pub fn play_sound(
    stream_handle: &OutputStreamHandle,
    buf: &Buffered<Decoder<BufReader<File>>>,
    volume: f32,
) -> Result<()> {
    let sink = Sink::try_new(stream_handle).context("Coulnd't create an audio sink.")?;

    sink.set_volume(volume * 0.01);
    sink.append(buf.clone());
    sink.detach();

    Ok(())
}
