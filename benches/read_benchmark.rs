use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::Throughput;
use criterion::{black_box, criterion_group, criterion_main};
use sqlite_varint::read_varint;

pub fn criterion_benchmark(c: &mut Criterion) {
    let bytes_vec: Vec<Vec<u8>> = vec![
        vec![0x0f],
        vec![0xff, 0x0f],
        vec![0xff, 0xff, 0x0f],
        vec![0xff, 0xff, 0xff, 0x0f],
        vec![0xff, 0xff, 0xff, 0xff, 0x0f],
        vec![0xff, 0xff, 0xff, 0xff, 0xff, 0x0f],
        vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0f],
        vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0f],
        vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0f],
        // Next ones are exceeding the max length of a varint
        vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0f, 0x0f],
        vec![
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0f, 0x0f, 0x0f,
        ],
    ];
    let mut group = c.benchmark_group("read_varint");
    for input in bytes_vec {
        group.throughput(Throughput::Bytes(input.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("Input length: {:?}", input.len())),
            &input,
            |b, input| {
                b.iter(|| read_varint(input));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
