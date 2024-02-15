use criterion::{black_box, criterion_group, criterion_main, Criterion};
use home::home_dir;
use rodio::OutputStream;
use std::{
    path::Path,
    sync::{atomic::AtomicU8, Arc, Mutex, RwLock},
    thread::{self},
};
use whisper_keys_engine::{packs, player, APP_NAME};

fn bench_integral(c: &mut Criterion) {
    let home_dir = home_dir().unwrap();
    let packs_folder = Path::new(&home_dir).join(APP_NAME);
    let pack_name = "nk-cream";
    let pack = packs::load_pack(&packs_folder, pack_name).unwrap();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let default_volume = pack.keys_default_volume;

    let arc_pack = Arc::new(Mutex::new(pack.clone()));
    let arc_volume = Arc::new(Mutex::new(default_volume));
    let arc_pack2 = Arc::new(RwLock::new(pack));
    let arc_volume2 = Arc::new(AtomicU8::new(100));

    let mut group = c.benchmark_group("Main");

    group.bench_function("bench_integral mutexes", |b| {
        b.iter(|| {
            let pack_lock = arc_pack.lock().unwrap();
            let buf = pack_lock.keys.get(black_box("keyq")).unwrap();
            let volume = *arc_volume.lock().unwrap();

            let stream_handle_clone = stream_handle.clone();
            let buf_clone = buf.clone();
            thread::spawn(move || {
                player::play_sound(stream_handle_clone, buf_clone, volume).unwrap();
            });
        });
    });
    group.bench_function("bench_integral atomic + rwLock", |b| {
        b.iter(|| {
            let pack_lock = arc_pack2.read().unwrap();
            let buf = pack_lock.keys.get(black_box("keyq")).unwrap();
            let volume = arc_volume2.load(std::sync::atomic::Ordering::Relaxed) as f32;

            let stream_handle_clone = stream_handle.clone();
            let buf_clone = buf.clone();
            thread::spawn(move || {
                player::play_sound(stream_handle_clone, buf_clone, volume).unwrap();
            });
        });
    });

    group.finish();
}

criterion_group!(benches, bench_integral);
criterion_main!(benches);
