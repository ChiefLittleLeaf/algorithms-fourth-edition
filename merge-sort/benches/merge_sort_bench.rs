use criterion::{black_box, criterion_group, criterion_main, Criterion};
use merge_sort::{merge, merge_sort};
use rand::Rng;

fn benchmark_merge_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("merge_sort");
    group.sample_size(10); // Adjust sample size as needed

    group.bench_function("merge_sort_20_000_000", |b| {
        b.iter(|| {
            let mut arr = vec![0_i32; 20_000_000];
            rand::thread_rng().fill(&mut arr[..]);
            merge_sort(black_box(&mut arr));
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_merge_sort);
criterion_main!(benches);
