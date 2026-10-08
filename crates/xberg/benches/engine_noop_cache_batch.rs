use std::hint::black_box;
use std::sync::Arc;
use std::time::Duration;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use xberg::{
    ExtractInput, ExtractionConfig,
    engine::{Engine, seams::NoopCache},
};

const BATCH_SIZE: usize = 8;
const BYTES_PER_INPUT: usize = 4 * 1024 * 1024;

fn inputs() -> Vec<ExtractInput> {
    (0..BATCH_SIZE)
        .map(|index| {
            ExtractInput::from_bytes(
                vec![b'x'; BYTES_PER_INPUT],
                "text/plain",
                Some(format!("batch-{index}.txt")),
            )
        })
        .collect()
}

fn config(use_cache: bool) -> ExtractionConfig {
    ExtractionConfig {
        use_cache,
        enable_quality_processing: false,
        disable_ocr: true,
        extraction_timeout_secs: None,
        ..Default::default()
    }
}

fn bench_engine_noop_cache_batch(criterion: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("benchmark runtime");
    let template = inputs();
    let mut group = criterion.benchmark_group("engine_noop_cache_batch");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(3));
    group.measurement_time(Duration::from_secs(8));

    for (name, engine, extraction_config) in [
        ("new_default", Engine::new_default(), config(true)),
        (
            "injected_noop_cache",
            Engine::builder().with_cache_backend(Arc::new(NoopCache)).build(),
            config(true),
        ),
        (
            "cache_disabled_control",
            Engine::builder().with_cache_backend(Arc::new(NoopCache)).build(),
            config(false),
        ),
    ] {
        group.bench_function(name, |bencher| {
            bencher.iter_batched(
                || template.clone(),
                |batch| {
                    let output = runtime
                        .block_on(engine.extract_batch(batch, &extraction_config))
                        .expect("batch extraction");
                    black_box(output);
                },
                BatchSize::LargeInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_engine_noop_cache_batch);
criterion_main!(benches);
