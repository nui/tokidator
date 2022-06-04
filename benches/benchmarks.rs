#![allow(unused_variables)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num_derive::{FromPrimitive, ToPrimitive};

use tokidator::rbac::json_discriminant_array_to_vec;

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    num_derive::FromPrimitive,
    num_derive::ToPrimitive,
    strum::Display,
    strum::EnumCount,
)]
enum TestPermission {
    Permission0,
    Permission1,
    Permission2,
    Permission3,
    Permission4,
    Permission5,
    Permission6,
    Permission7,
    Permission8,
    Permission9,
    Permission10,
    Permission11,
    Permission12,
    Permission13,
    Permission14,
    Permission15,
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("json_discriminant_array_to_vec", |b| {
        b.iter(|| {
            json_discriminant_array_to_vec::<TestPermission>(black_box(
                "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]",
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
