use criterion::{criterion_group, criterion_main, Criterion};
use water_sort::{
    generate::system_generator::generate_random_system_with_seed, solver::bfs_shortest_path,
};

fn criterion_benchmark(c: &mut Criterion) {
    let seed = 42;

    let mut group = c.benchmark_group("water_sort_bfs");
    let generated_systems =
        (2..6).map(|system_size| generate_random_system_with_seed(system_size, seed).unwrap());

    for system in generated_systems {
        group.bench_function(format!("water_sort {}", system.get_state().len()), |b| {
            b.iter(|| bfs_shortest_path(&system))
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
