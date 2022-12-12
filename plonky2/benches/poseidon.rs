mod allocator;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::hashing::SPONGE_WIDTH;
use plonky2::hash::poseidon::Poseidon;
use tynm::type_name;

pub(crate) fn bench_poseidon<F: Poseidon>(c: &mut Criterion) {
    c.bench_function(
        &format!("poseidon<{}, {SPONGE_WIDTH}>", type_name::<F>()),
        |b| {
            b.iter_batched(
                || F::rand_array::<SPONGE_WIDTH>(),
                |state| F::poseidon(state),
                BatchSize::SmallInput,
            )
        },
    );
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_poseidon::<GoldilocksField>(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
