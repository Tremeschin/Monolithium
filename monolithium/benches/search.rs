use criterion::{Criterion, criterion_group, criterion_main};
use monolithium::{SeedFactory, commands::SearchCommand};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("search linear", |b| {
        b.iter(|| {
            black_box(
                SearchCommand {
                    seeds: SeedFactory::Linear {
                        start: 0,
                        total: 1000,
                    },
                    chunks: 1,
                    threaded: false,
                    center_x: 0,
                    center_z: 0,
                    radius: 100,
                    step: 200,
                    limit: 999999,
                    area: 0,
                    hill: false,
                    depth: false,
                    silent: true,
                }
                .run(),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
