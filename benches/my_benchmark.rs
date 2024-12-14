use criterion::{criterion_group, criterion_main, Criterion};
use AoC2024::{day0, day10, day11, day12, day13, day2, day3, day4, day5, day6, day7, day8, day9};

fn day0(c: &mut Criterion) {
    c.bench_function("day 0 part1", |b| b.iter(day0::part1));
    c.bench_function("day 0 part2", |b| b.iter(day0::part2));
}
fn day1(c: &mut Criterion) {
}
fn day2(c: &mut Criterion) {
    c.bench_function("day 2 part1", |b| b.iter(day2::part1));
    c.bench_function("day 2 part2", |b| b.iter(day2::part2));
}
fn day3(c: &mut Criterion) {
    c.bench_function("day 3 part1", |b| b.iter(day3::part1));
    c.bench_function("day 3 part2", |b| b.iter(day3::part2));
}
fn day4(c: &mut Criterion) {
    c.bench_function("day 4 part1", |b| b.iter(day4::part1));
    c.bench_function("day 4 part2", |b| b.iter(day4::part2));
}
fn day5(c: &mut Criterion) {
    c.bench_function("day 5 part1", |b| b.iter(day5::part1));
    c.bench_function("day 5 part2", |b| b.iter(day5::part2));
}
fn day6(c: &mut Criterion) {
    c.bench_function("day 6 part1", |b| b.iter(day6::part1));
    c.bench_function("day 6 part2", |b| b.iter(day6::part2));
}
fn day7(c: &mut Criterion) {
    c.bench_function("day 7 part1", |b| b.iter(day7::part1));
    c.bench_function("day 7 part2", |b| b.iter(day7::part2));
}
fn day8(c: &mut Criterion) {
    c.bench_function("day 8 part1", |b| b.iter(day8::part1));
    c.bench_function("day 8 part2", |b| b.iter(day8::part2));
}
fn day9(c: &mut Criterion) {
    c.bench_function("day 9 part1", |b| b.iter(day9::part1));
    c.bench_function("day 9 part2", |b| b.iter(day9::part2));
}
fn day10(c: &mut Criterion) {
    c.bench_function("day 10 part1", |b| b.iter(day10::part1));
    c.bench_function("day 10 part2", |b| b.iter(day10::part2));
}

fn day11(c: &mut Criterion) {
    c.bench_function("day 11 part1", |b| b.iter(day11::part1));
    c.bench_function("day 11 part2", |b| b.iter(day11::part2));
}
fn day12(c: &mut Criterion) {
    c.bench_function("day 12 part1", |b| b.iter(day12::part1));
    c.bench_function("day 12 part2", |b| b.iter(day12::part2));
}
fn day13(c: &mut Criterion) {
    c.bench_function("day 13 part1", |b| b.iter(day13::part1));
    c.bench_function("day 13 part2", |b| b.iter(day13::part2));
}
fn day14(c: &mut Criterion) {
    c.bench_function("day 14 part1", |b| b.iter(day14::part1));
    c.bench_function("day 14 part2", |b| b.iter(day14::part2));
}

criterion_group!(benches, day0, day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13);
criterion_main!(benches);
