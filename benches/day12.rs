use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent_of_code_rust_2024::day12::{part1, part2, part1_iter, part2_iter};

fn check_result(output: impl ToString, expected: &str, part: u8) {
    if output.to_string().trim() != expected.trim() {
        panic!("Output does not match expected for part {}", part);
    }
}

fn bench_part1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day12.txt");
    let expected = include_str!("../input/2024/day12.output.1.txt");

    check_result(part1(input), expected, 1);
    c.bench_function("day12_part1", |b| b.iter(|| part1(black_box(input))));

    check_result(part1_iter(input), expected, 1);
    c.bench_function("day12_part1_iter", |b| b.iter(|| part1_iter(black_box(input))));
}

fn bench_part2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day12.txt");
    let expected = include_str!("../input/2024/day12.output.2.txt");

    check_result(part2(input), expected, 2);
    c.bench_function("day12_part2", |b| b.iter(|| part2(black_box(input))));

    check_result(part2_iter(input), expected, 2);
    c.bench_function("day12_part2_iter", |b| b.iter(|| part2_iter(black_box(input))));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
