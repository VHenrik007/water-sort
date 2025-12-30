use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use water_sort::{
    generate::system_generator::generate_random_system_with_seed, solver::system_solver::Solver,
};

fn criterion_benchmark(c: &mut Criterion) {
    let system = generate_random_system_with_seed(4, 42).unwrap();
    let solver = Solver {};
    c.bench_function("water_10", |b| {
        b.iter(|| solver.find_solution(black_box(&system)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
