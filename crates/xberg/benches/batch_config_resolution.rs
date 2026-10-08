//! Isolates the per-input configuration clone cost removed from batch preflight.
//!
//! This is not an end-to-end extraction benchmark; use `concurrency_scaling` to measure
//! complete batch throughput. ~keep

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use xberg::ExtractionConfig;

fn clone_base_config(base: &ExtractionConfig, input_count: usize) {
    for _ in 0..input_count {
        let resolved = black_box(base.clone());
        black_box(resolved);
    }
}

fn borrow_base_config(base: &ExtractionConfig, input_count: usize) {
    for _ in 0..input_count {
        let resolved = black_box(base);
        black_box(resolved);
    }
}

fn bench_batch_config_resolution(criterion: &mut Criterion) {
    let base = ExtractionConfig::default();
    let mut group = criterion.benchmark_group("batch_config_resolution/no_overrides");

    for input_count in [1_usize, 16, 64] {
        group.throughput(Throughput::Elements(input_count as u64));
        group.bench_with_input(
            BenchmarkId::new("clone", input_count),
            &input_count,
            |bencher, &count| {
                bencher.iter(|| clone_base_config(black_box(&base), count));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("borrow", input_count),
            &input_count,
            |bencher, &count| {
                bencher.iter(|| borrow_base_config(black_box(&base), count));
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_batch_config_resolution);
criterion_main!(benches);
