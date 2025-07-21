use criterion::{Criterion, criterion_group, criterion_main};

fn bench_append_vs_extend(c: &mut Criterion) {
    let encoded = vec![0u8; 1024];

    c.bench_function("append", |b| {
        b.iter(|| {
            let mut out = Vec::new();
            let mut enc = encoded.clone();
            out.append(&mut enc);
        })
    });

    c.bench_function("extend_from_slice", |b| {
        b.iter(|| {
            let mut out = Vec::new();
            out.extend_from_slice(&encoded);
        })
    });
}

criterion_group!(benches, bench_append_vs_extend);
criterion_main!(benches);
