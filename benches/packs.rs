use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use home::home_dir;
use whisper_keys_engine::{packs, APP_NAME};

fn list_available(c: &mut Criterion) {
    let home_dir = home_dir().unwrap();
    let packs_folder = Path::new(&home_dir).join(APP_NAME);

    c.bench_function("list_available", |b| {
        b.iter(|| packs::list_available(black_box(&packs_folder)).unwrap())
    });
}

fn load_pack(c: &mut Criterion) {
    let home_dir = home_dir().unwrap();
    let packs_folder = Path::new(&home_dir).join(APP_NAME);
    let pack_name = "nk-cream";

    c.bench_function("load_pack", |b| {
        b.iter(|| packs::load_pack(black_box(&packs_folder), black_box(pack_name)).unwrap())
    });
}

fn get_key(c: &mut Criterion) {
    let home_dir = home_dir().unwrap();
    let packs_folder = Path::new(&home_dir).join(APP_NAME);
    let pack_name = "nk-cream";
    let pack = packs::load_pack(&packs_folder, pack_name).unwrap();

    c.bench_function("get_key", |b| {
        b.iter(|| pack.keys.get(black_box("keyq")).unwrap())
    });
}

criterion_group!(benches, list_available, load_pack, get_key);
criterion_main!(benches);
