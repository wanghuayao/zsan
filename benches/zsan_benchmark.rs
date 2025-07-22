use criterion::{Criterion, criterion_group, criterion_main};
use zsan::{compress, decompress};

fn bench_compress(c: &mut Criterion) {
    let test_cases = [
        ("short_number", "12345"),
        ("negative_decimal", "-67.89"),
        ("mixed_content", "abc 123 def 45.67   ghi"),
        ("long_text", "This is a longer text with multiple numbers 123.45 and spaces   to test compression performance")
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
        ("short_number", "12345"),
        ("negative_decimal", "-67.89"),
        ("mixed_content", "abc 123 def 45.67   ghi"),
        ("long_text", "This is a longer text with multiple numbers 123.45 and spaces   to test compression performance")
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