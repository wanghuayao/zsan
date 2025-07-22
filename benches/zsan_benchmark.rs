use criterion::{Criterion, criterion_group, criterion_main};
use zsan::{compress, decompress};

fn bench_compress(c: &mut Criterion) {
    let test_cases = [
        (
            "case-1",
            "6224      ABC20200902       1312       1145       7802       5411          17800          90532                                           1109.2049 ",
        ),
        (
            "case-2",
            "9951      EFG0990     132230     132280     102230     192230          4         46     5938.6 561969.6111",
        ),
    ];

    for (name, input) in test_cases.iter() {
        c.bench_function(&format!("compress_{}", name), |b| {
            b.iter(|| {
                let mut output = Vec::new();
                compress(input, &mut output);
            })
        });
    }
}

fn bench_decompress(c: &mut Criterion) {
    let test_cases = [
        (
            "case-1",
            "6224      ABC20200902       1312       1145       7802       5411          17800          90532                                           1109.2049 ",
        ),
        (
            "case-2",
            "9951      EFG0990     132230     132280     102230     192230          4         46     5938.6 561969.6111",
        ),
    ];

    for (name, input) in test_cases.iter() {
        // Pre-compress the data once before benchmarking
        let mut compressed = Vec::new();
        compress(input, &mut compressed);
        let compressed_data = compressed;

        c.bench_function(&format!("decompress_{}", name), |b| {
            b.iter(|| {
                let mut output = Vec::new();
                decompress(&compressed_data, &mut output);
            })
        });
    }
}

criterion_group!(zsan_benches, bench_compress, bench_decompress);
criterion_main!(zsan_benches);
