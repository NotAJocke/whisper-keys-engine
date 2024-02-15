use std::{path::Path, thread};

use criterion::{criterion_group, criterion_main, Criterion};
use home::home_dir;
use rodio::OutputStream;
use whisper_keys_engine::{packs::load_pack, player, APP_NAME};

fn play_sound(c: &mut Criterion) {
    let home_dir = home_dir().unwrap();
    let packs_folder = Path::new(&home_dir).join(APP_NAME);
    let pack_name = "nk-cream";

    let pack = load_pack(&packs_folder, pack_name).unwrap();
    let buf = pack.keys.get("keya").unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    c.bench_function("play_sound", |b| {
        b.iter(|| {
            let stream_handle_clone = stream_handle.clone();
            let buf_clone = buf.clone();
            thread::spawn(move || player::play_sound(stream_handle_clone, buf_clone, 100.0))
        });
    });
}

criterion_group!(benches, play_sound);
criterion_main!(benches);
