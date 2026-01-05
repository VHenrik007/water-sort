use criterion::{Criterion, criterion_group, criterion_main};
use water_sort::{
    generate::system_generator::generate_random_system_with_seed, solver::{SolutionValueMode, heuristic_dijkstra_search}
};

fn criterion_benchmark(c: &mut Criterion) {
    let seed = 42;
    let heuristic_mode = SolutionValueMode::ColorCount;

    let mut group = c.benchmark_group("water_sort_heuristic");
    let generated_systems = (2..8).map(|system_size| generate_random_system_with_seed(system_size, seed).unwrap());

    for system in generated_systems {
        group.bench_function(format!("water_sort {}", system.get_state().len()), |b| b.iter(|| heuristic_dijkstra_search(&system, &heuristic_mode)));
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
