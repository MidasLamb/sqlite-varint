use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use criterion::{black_box, criterion_group, criterion_main};
use sqlite_varint::serialize_to_varint;

pub fn criterion_benchmark(c: &mut Criterion) {
    let bytes_vec: Vec<i64> = vec![0, 1, 256, 0xffff, 0xffffff, 0xffffffff];
    let mut group = c.benchmark_group("serialize_varint");
    for input in bytes_vec {
        group.throughput(Throughput::Bytes(input as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("Input : {:x?}", input)),
            &input,
            |b, input| {
                b.iter(|| serialize_to_varint(*input));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
