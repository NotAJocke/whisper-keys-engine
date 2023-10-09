use anyhow::Result;
use std::{fs::File, io::BufReader};

use rodio::{source::Buffered, Decoder, OutputStreamHandle, Sink};

pub fn play_sound(
    stream_handle: OutputStreamHandle,
    buf: Buffered<Decoder<BufReader<File>>>,
) -> Result<()> {
    let sink = Sink::try_new(&stream_handle)?;

    sink.set_volume(5.0);
    sink.append(buf.clone());
    sink.detach();

    Ok(())
}
