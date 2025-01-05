use criterion::{criterion_group, criterion_main, Criterion};

fn bench_day00(c: &mut Criterion) {
    c.bench_function("day 0 part1", |b| b.iter(day00::part1));
    c.bench_function("day 0 part2", |b| b.iter(day00::part2));
}
fn bench_day01(c: &mut Criterion) {
    c.bench_function("day 1 part1", |b| b.iter(day01::part1));
    c.bench_function("day 1 part2", |b| b.iter(day01::part2));
}
fn bench_day02(c: &mut Criterion) {
    c.bench_function("day 2 part1", |b| b.iter(day02::part1));
    c.bench_function("day 2 part2", |b| b.iter(day02::part2));
}
fn bench_day03(c: &mut Criterion) {
    c.bench_function("day 3 part1", |b| b.iter(day03::part1));
    c.bench_function("day 3 part2", |b| b.iter(day03::part2));
}
fn bench_day04(c: &mut Criterion) {
    c.bench_function("day 4 part1", |b| b.iter(day04::part1));
    c.bench_function("day 4 part2", |b| b.iter(day04::part2));
}
fn bench_day05(c: &mut Criterion) {
    c.bench_function("day 5 part1", |b| b.iter(day05::part1));
    c.bench_function("day 5 part2", |b| b.iter(day05::part2));
}
fn bench_day06(c: &mut Criterion) {
    c.bench_function("day 6 part1", |b| b.iter(day06::part1));
    c.bench_function("day 6 part2", |b| b.iter(day06::part2));
}
fn bench_day07(c: &mut Criterion) {
    c.bench_function("day 7 part1", |b| b.iter(day07::part1));
    c.bench_function("day 7 part2", |b| b.iter(day07::part2));
}
fn bench_day08(c: &mut Criterion) {
    c.bench_function("day 8 part1", |b| b.iter(day08::part1));
    c.bench_function("day 8 part2", |b| b.iter(day08::part2));
}
fn bench_day09(c: &mut Criterion) {
    c.bench_function("day 9 part1", |b| b.iter(day09::part1));
    c.bench_function("day 9 part2", |b| b.iter(day09::part2));
}
fn bench_day10(c: &mut Criterion) {
    c.bench_function("day 10 part1", |b| b.iter(day10::part1));
    c.bench_function("day 10 part2", |b| b.iter(day10::part2));
}

fn bench_day11(c: &mut Criterion) {
    c.bench_function("day 11 part1", |b| b.iter(day11::part1));
    c.bench_function("day 11 part2", |b| b.iter(day11::part2));
}
fn bench_day12(c: &mut Criterion) {
    c.bench_function("day 12 part1", |b| b.iter(day12::part1));
    c.bench_function("day 12 part2", |b| b.iter(day12::part2));
}
fn bench_day13(c: &mut Criterion) {
    c.bench_function("day 13 part1", |b| b.iter(day13::part1));
    c.bench_function("day 13 part2", |b| b.iter(day13::part2));
}
fn bench_day15(c: &mut Criterion) {
    c.bench_function("day 15 part1", |b| b.iter(day15::part1));
    c.bench_function("day 15 part2", |b| b.iter(day15::part2));
}
fn bench_day16(c: &mut Criterion) {
    c.bench_function("day 16 part1", |b| b.iter(day16::part1));
    c.bench_function("day 16 part2", |b| b.iter(day16::part2));
}
fn bench_day17(c: &mut Criterion) {
    c.bench_function("day 17 part1", |b| b.iter(day17::part1));
    // c.bench_function("day 17 part2", |b| b.iter(day17::part2));
}
fn bench_day18(c: &mut Criterion) {
    c.bench_function("day 18 part1", |b| b.iter(day18::part1));
    c.bench_function("day 18 part2", |b| b.iter(day18::part2));
}
fn bench_day19(c: &mut Criterion) {
    c.bench_function("day 19 part1", |b| b.iter(day19::part1));
    c.bench_function("day 19 part2", |b| b.iter(day19::part2));
}
fn bench_day20(c: &mut Criterion) {
    c.bench_function("day 20 part1", |b| b.iter(day20::part1));
    c.bench_function("day 20 part2", |b| b.iter(day20::part2));
}
fn bench_day21(c: &mut Criterion) {
    c.bench_function("day 21 part1", |b| b.iter(day21::part1));
    c.bench_function("day 21 part2", |b| b.iter(day21::part2));
}
fn bench_day22(c: &mut Criterion) {
    c.bench_function("day 22 part1", |b| b.iter(day22::part1));
    c.bench_function("day 22 part2", |b| b.iter(day22::part2));
}

fn bench_day24(c: &mut Criterion) {
    c.bench_function("day 24 part1", |b| b.iter(day24::part1));
    c.bench_function("day 24 part2", |b| b.iter(day24::part2));
}

// criterion_group!(benches, day0, day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13);
criterion_group!(benches, bench_day00);
criterion_main!(benches);
