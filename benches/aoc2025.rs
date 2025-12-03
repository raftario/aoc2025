use aoc2025::CHALLENGES;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

fn benchmark(bencher: &mut Criterion) {
    for challenge in CHALLENGES {
        bencher.bench_with_input(
            BenchmarkId::new(
                format!("Day {}", challenge.day()),
                format!("Part {}", challenge.part()),
            ),
            &challenge.input(),
            |bencher, input| bencher.iter(|| challenge.run(input)),
        );
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
