use std::{
    path::Path,
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc, Mutex, RwLock,
    },
    thread, time,
};

use criterion::{criterion_group, criterion_main, Criterion};
use whisper_keys_engine::{packs, APP_NAME};

fn bench_integral(c: &mut Criterion) {
    let mut group = c.benchmark_group("Main");
    let amount = 20;

    let home_dir = home::home_dir().unwrap();
    let packs_folder = Path::new(&home_dir).join(APP_NAME);
    let pack_name = "nk-cream";
    let pack = packs::load_pack(&packs_folder, pack_name).unwrap();

    let volume_f = pack.keys_default_volume;
    let volume_u: u8 = pack.keys_default_volume as u8;
    let keys = pack.keys;

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    group.bench_function(format!("{} sounds played with mutexes", amount), |b| {
        b.iter(|| {
            let keys_arc = Arc::new(Mutex::new(keys.clone()));
            let volume_arc = Arc::new(Mutex::new(volume_f));

            for _ in 0..amount {
                let keys_lock = keys_arc.lock().unwrap();
                let volume = *volume_arc.lock().unwrap();

                let key = keys_lock.get("keyq").unwrap();

                let sink = rodio::Sink::try_new(&stream_handle).unwrap();
                sink.set_volume(volume * 0.01);
                sink.append(key.clone());
                sink.detach();

                thread::sleep(time::Duration::from_millis(50));
            }
        });
    });
    group.bench_function(
        format!("{} sounds played with rwlock + atomics", amount),
        |b| {
            b.iter(|| {
                let keys_rw = RwLock::new(keys.clone());
                let volume = AtomicU8::new(volume_u);

                for _ in 0..amount {
                    let keys_reader = keys_rw.read().unwrap();

                    let key = keys_reader.get("keyq").unwrap();

                    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
                    let volume_cast = volume.load(Ordering::Relaxed) as f32;
                    sink.set_volume(volume_cast * 0.01);
                    sink.append(key.clone());
                    sink.detach();

                    thread::sleep(time::Duration::from_millis(50));
                }
            });
        },
    );

    group.finish();
}

criterion_group!(benches, bench_integral);
criterion_main!(benches);
