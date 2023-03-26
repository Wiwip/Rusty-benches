use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use SpatialBenches::Spatial;


mod hashmap;
mod kdtree;
mod rtree;

criterion_main!(spatial_benches);
criterion_group!(spatial_benches, spatial_tests);

fn spatial_tests(c: &mut Criterion) {
    let range = 32.0;

    let mut g = c.benchmark_group("Bulk Insert Time Comparison");
    for i in [500, 1000, 2500, 5000, 10000].iter() {
        let mut kd_bench = kdtree::Benchmark::new();
        g.bench_with_input(BenchmarkId::new("KdTree", i), i, |b, i| {
            b.iter(|| kd_bench.build_tree(*i))
        });

        let mut rtree_bench = rtree::Benchmark::new();
        g.bench_with_input(BenchmarkId::new("RTree", i), i, |b, i| {
            b.iter(|| rtree_bench.build_tree(*i))
        });

        let mut hashmap_bench = hashmap::Benchmark {
            map: Default::default(),
            list_offsets: vec![],
            cell_size: 32.0,
        };
        g.bench_with_input(BenchmarkId::new("HashMap", i), i, |b, i| {
            b.iter(|| hashmap_bench.build_tree(*i))
        });
    }
    g.finish();

    let mut g = c.benchmark_group("Spatial Queries Comparison");
    for i in [500, 1000, 2500, 5000, 10000].iter() {
        let mut kd_bench = kdtree::Benchmark::new();
        kd_bench.build_tree(*i);
        g.bench_with_input(BenchmarkId::new("KdTree", i), i, |b, i| {
            b.iter(|| kd_bench.within(range))
        });

        let mut rtree_bench = rtree::Benchmark::new();
        rtree_bench.build_tree(*i);
        g.bench_with_input(BenchmarkId::new("RTree", i), i, |b, i| {
            b.iter(|| rtree_bench.within(range))
        });

        let mut hashmap_bench = hashmap::Benchmark {
            map: Default::default(),
            cell_size: 32.0,
            list_offsets: vec![
                [-1, 1, 0],
                [0, 1, 0],
                [1, 1, 0],
                [-1, 0, 0],
                [0, 0, 0],
                [1, 0, 0],
                [-1, -1, 0],
                [0, -1, 0],
                [1, -1, 0],
            ],
        };
        hashmap_bench.build_tree(*i);
        g.bench_with_input(BenchmarkId::new("HashMap", i), i, |b, i| {
            b.iter(|| hashmap_bench.within(range))
        });
    }
    g.finish();
}
