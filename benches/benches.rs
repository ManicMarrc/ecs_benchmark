use criterion::{
  criterion_group,
  criterion_main,
  BenchmarkId,
  Criterion,
  Throughput,
};

const N: [usize; 3] = [1_000, 100_000, 1_000_000];

macro_rules! new_bench {
  ($name:ident) => {
    paste::paste! {
      pub fn [<bench_$name>](c: &mut Criterion) {
        let mut group = c.benchmark_group(stringify!($name));
        for i in N {
          group.throughput(Throughput::Elements(i as u64));
          group.bench_with_input(BenchmarkId::new("bevy_ecs", i), &i, |b, i| {
            let mut bevy_ecs = ecs_benchmark::bevy_ecs::Ecs::new();
            b.iter(|| bevy_ecs.$name(*i));
          });
          group.bench_with_input(BenchmarkId::new("edict", i), &i, |b, i| {
            let mut edict = ecs_benchmark::edict::Ecs::new();
            b.iter(|| edict.$name(*i));
          });
          group.bench_with_input(BenchmarkId::new("hecs", i), &i, |b, i| {
            let mut hecs = ecs_benchmark::hecs::Ecs::new();
            b.iter(|| hecs.$name(*i));
          });
          group.bench_with_input(BenchmarkId::new("shipyard", i), &i, |b, i| {
            let mut shipyard = ecs_benchmark::shipyard::Ecs::new();
            b.iter(|| shipyard.$name(*i));
          });
        }
        group.finish();
      }
    }
  };
}

new_bench!(unbatched_spawn);
new_bench!(batched_spawn);
new_bench!(add_remove);
new_bench!(iter);
new_bench!(multiple_iter);
new_bench!(multiple_iter_same);
new_bench!(iter_mut);
new_bench!(multiple_iter_mut);
new_bench!(multiple_iter_mut_same);

criterion_group!(
  benches,
  bench_unbatched_spawn,
  bench_batched_spawn,
  bench_add_remove,
  bench_iter,
  bench_multiple_iter,
  bench_multiple_iter_same,
  bench_iter_mut,
  bench_multiple_iter_mut,
  bench_multiple_iter_mut_same
);
criterion_main!(benches);
