use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent_of_code_rust_2024::day2::{part1, part2};

fn check_result(output: impl ToString, expected: &str, part: u8) {
    if output.to_string().trim() != expected.trim() {
        panic!("Output does not match expected for part {}", part);
    }
}

fn bench_part1(c: &mut Criterion) {
    let input = include_str!("../input/2024/day2.txt");
    let expected = include_str!("../input/2024/day2.output.1.txt");
    check_result(part1(input), expected, 1);
    c.bench_function("day2_part1", |b| b.iter(|| part1(black_box(input))));

}

fn bench_part2(c: &mut Criterion) {
    let input = include_str!("../input/2024/day2.txt");
    let expected = include_str!("../input/2024/day2.output.2.txt");

    check_result(part2(input), expected, 2);
    c.bench_function("day2_part2", |b| b.iter(|| part2(black_box(input))));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
