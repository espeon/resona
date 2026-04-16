use criterion::{criterion_group, criterion_main, Criterion};
use nanorand::ChaCha;
use resona::{sona, sona_with_rng, Config};

fn bench_sona(c: &mut Criterion) {
    let config = Config::default();

    c.bench_function("generate_single_word", |b| {
        b.iter(|| {
            let _ = sona(&config);
        })
    });

    let config_custom = Config::new().min(8).max(12).build();
    c.bench_function("generate_single_word_custom", |b| {
        b.iter(|| {
            let _ = sona(&config_custom);
        })
    });

    c.bench_function("generate_100_words", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let _ = sona(&config);
            }
        })
    });

    c.bench_function("generate_1k_words", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let _ = sona(&config);
            }
        })
    });

    c.bench_function("generate_10k_words", |b| {
        b.iter(|| {
            for _ in 0..10000 {
                let _ = sona(&config);
            }
        })
    });

    let mut rng = ChaCha::new();
    c.bench_function("generate_with_external_rng", |b| {
        b.iter(|| {
            let _ = sona_with_rng(&mut rng, &config);
        })
    });
}

criterion_group!(benches, bench_sona);
criterion_main!(benches);
