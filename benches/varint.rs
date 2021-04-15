use criterion::{criterion_group, criterion_main, Criterion};

use bincode::Options;
use rand::distributions::Distribution;

fn slice_varint_u8(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Uniform::from(0..u8::MAX);
    let input: Vec<u8> = std::iter::from_fn(|| Some(dist.sample(&mut rng)))
        .take(10_000)
        .collect();
    let options = bincode::options();
    let bytes = options.serialize(&input).unwrap();

    c.bench_function("slice_varint_u8", |b| {
        b.iter(|| {
            let _: Vec<u8> = options.deserialize(&bytes).unwrap();
        })
    });
}

fn slice_varint_u16(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Uniform::from(0..u16::MAX);
    let input: Vec<u16> = std::iter::from_fn(|| Some(dist.sample(&mut rng)))
        .take(10_000)
        .collect();
    let options = bincode::options();
    let bytes = options.serialize(&input).unwrap();

    c.bench_function("slice_varint_u16", |b| {
        b.iter(|| {
            let _: Vec<u16> = options.deserialize(&bytes).unwrap();
        })
    });
}

fn slice_varint_u32(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Uniform::from(0..u32::MAX);
    let input: Vec<u32> = std::iter::from_fn(|| Some(dist.sample(&mut rng)))
        .take(10_000)
        .collect();
    let options = bincode::options();
    let bytes = options.serialize(&input).unwrap();

    c.bench_function("slice_varint_u32", |b| {
        b.iter(|| {
            let _: Vec<u32> = options.deserialize(&bytes).unwrap();
        })
    });
}

fn slice_varint_u64(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Uniform::from(0..u64::MAX);
    let input: Vec<u64> = std::iter::from_fn(|| Some(dist.sample(&mut rng)))
        .take(10_000)
        .collect();
    let options = bincode::options();
    let bytes = options.serialize(&input).unwrap();

    c.bench_function("slice_varint_u64", |b| {
        b.iter(|| {
            let _: Vec<u64> = options.deserialize(&bytes).unwrap();
        })
    });
}

fn bufreader_varint_u8(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Uniform::from(0..u8::MAX);
    let input: Vec<u8> = std::iter::from_fn(|| Some(dist.sample(&mut rng)))
        .take(10_000)
        .collect();
    let options = bincode::options();
    let bytes = options.serialize(&input).unwrap();

    c.bench_function("bufreader_varint_u8", |b| {
        b.iter(|| {
            let _: Vec<u8> = options
                .deserialize_from(&mut std::io::BufReader::new(&bytes[..]))
                .unwrap();
        })
    });
}

fn bufreader_varint_u16(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Uniform::from(0..u16::MAX);
    let input: Vec<u16> = std::iter::from_fn(|| Some(dist.sample(&mut rng)))
        .take(10_000)
        .collect();
    let options = bincode::options();
    let bytes = options.serialize(&input).unwrap();

    c.bench_function("bufreader_varint_u16", |b| {
        b.iter(|| {
            let _: Vec<u16> = options
                .deserialize_from(&mut std::io::BufReader::new(&bytes[..]))
                .unwrap();
        })
    });
}

fn bufreader_varint_u32(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Uniform::from(0..u32::MAX);
    let input: Vec<u32> = std::iter::from_fn(|| Some(dist.sample(&mut rng)))
        .take(10_000)
        .collect();
    let options = bincode::options();
    let bytes = options.serialize(&input).unwrap();

    c.bench_function("bufreader_varint_u32", |b| {
        b.iter(|| {
            let _: Vec<u32> = options
                .deserialize_from(&mut std::io::BufReader::new(&bytes[..]))
                .unwrap();
        })
    });
}

fn bufreader_varint_u64(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Uniform::from(0..u64::MAX);
    let input: Vec<u64> = std::iter::from_fn(|| Some(dist.sample(&mut rng)))
        .take(10_000)
        .collect();
    let options = bincode::options();
    let bytes = options.serialize(&input).unwrap();

    c.bench_function("bufreader_varint_u64", |b| {
        b.iter(|| {
            let _: Vec<u64> = options
                .deserialize_from(&mut std::io::BufReader::new(&bytes[..]))
                .unwrap();
        })
    });
}

criterion_group!(
    benches,
    slice_varint_u8,
    slice_varint_u16,
    slice_varint_u32,
    slice_varint_u64,
    bufreader_varint_u8,
    bufreader_varint_u16,
    bufreader_varint_u32,
    bufreader_varint_u64,
);
criterion_main!(benches);
