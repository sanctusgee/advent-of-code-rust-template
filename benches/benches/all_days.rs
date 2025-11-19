use aoc_lib::SolutionRegistry;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_all_solutions(c: &mut Criterion) {
    // Benchmark all available years and days
    for year in SolutionRegistry::available_years() {
        let days = SolutionRegistry::available_days(year);

        for day in days {
            if let Some(solver) = SolutionRegistry::get_solver(year, day) {
                c.bench_function(&format!("{}/day{:02}", year, day), |b| {
                    b.iter(|| {
                        // Run solver and ignore errors in benchmark
                        let _ = black_box(solver());
                    });
                });
            }
        }
    }
}

criterion_group!(benches, bench_all_solutions);
criterion_main!(benches);
