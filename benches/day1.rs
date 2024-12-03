
// use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent_of_code_rust_2024::day1::{part1, part2};

fn bench_part1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day1.txt");
    c.bench_function("day1_part1", |b| b.iter(|| part1(black_box(input))));
}

fn bench_part2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day1.txt");
    c.bench_function("day1_part2", |b| b.iter(|| part2(black_box(input))));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
