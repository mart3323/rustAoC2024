use std::time::Duration;
use divan;

#[divan::bench_group(max_time = Duration::from_secs(5))]
mod my_benchmark {

    #[divan::bench_group()]
    mod day00 {
        #[divan::bench] fn part1() { day00::part1(); }
        #[divan::bench] fn part2() { day00::part2(); }
    }
    #[divan::bench_group()]
    mod day01 {
        #[divan::bench] fn part1() { day01::part1(); }
        #[divan::bench] fn part2() { day01::part2(); }
    }
    #[divan::bench_group()]
    mod day02 {
        #[divan::bench] fn part1() { day02::part1(); }
        #[divan::bench] fn part2() { day02::part2(); }
    }
    #[divan::bench_group()]
    mod day03 {
        #[divan::bench] fn part1() { day03::part1(); }
        #[divan::bench] fn part2() { day03::part2(); }
    }
    #[divan::bench_group()]
    mod day04 {
        #[divan::bench] fn part1() { day04::part1(); }
        #[divan::bench] fn part2() { day04::part2(); }
    }
    #[divan::bench_group()]
    mod day05 {
        #[divan::bench] fn part1() { day05::part1(); }
        #[divan::bench] fn part2() { day05::part2(); }
    }
    #[divan::bench_group()]
    mod day06 {
        #[divan::bench] fn part1() { day06::part1(); }
        #[divan::bench] fn part2() { day06::part2(); }
    }
    #[divan::bench_group()]
    mod day07 {
        #[divan::bench] fn part1() { day07::part1(); }
        #[divan::bench] fn part2() { day07::part2(); }
    }
    #[divan::bench_group()]
    mod day08 {
        #[divan::bench] fn part1() { day08::part1(); }
        #[divan::bench] fn part2() { day08::part2(); }
    }
    #[divan::bench_group()]
    mod day09 {
        #[divan::bench] fn part1() { day09::part1(); }
        #[divan::bench] fn part2() { day09::part2(); }
    }
    #[divan::bench_group()]
    mod day10 {
        #[divan::bench] fn part1() { day10::part1(); }
        #[divan::bench] fn part2() { day10::part2(); }
    }
    #[divan::bench_group()]
    mod day11 {
        #[divan::bench] fn part1() { day11::part1(); }
        #[divan::bench] fn part2() { day11::part2(); }
    }
    #[divan::bench_group()]
    mod day12 {
        #[divan::bench] fn part1() { day12::part1(); }
        #[divan::bench] fn part2() { day12::part2(); }
    }
    #[divan::bench_group()]
    mod day13 {
        #[divan::bench] fn part1() { day13::part1(); }
        #[divan::bench] fn part2() { day13::part2(); }
    }
    #[divan::bench_group()]
    mod day14 {
        #[divan::bench] fn part1() { day14::part1(); }
        #[divan::bench] fn part2() { day14::part2(); }
    }
    #[divan::bench_group()]
    mod day15 {
        #[divan::bench] fn part1() { day15::part1(); }
        #[divan::bench] fn part2() { day15::part2(); }
    }
    #[divan::bench_group()]
    mod day16 {
        #[divan::bench] fn part1() { day16::part1(); }
        #[divan::bench] fn part2() { day16::part2(); }
    }
    #[divan::bench_group()]
    mod day17 {
        #[divan::bench] fn part1() { day17::part1(); }
        #[divan::bench] fn part2() { day17::part2(); }
    }
    #[divan::bench_group()]
    mod day18 {
        #[divan::bench] fn part1() { day18::part1(); }
        #[divan::bench] fn part2() { day18::part2(); }
    }
    #[divan::bench_group()]
    mod day19 {
        #[divan::bench] fn part1() { day19::part1(); }
        #[divan::bench] fn part2() { day19::part2(); }
    }
    #[divan::bench_group()]
    mod day20 {
        #[divan::bench] fn part1() { day20::part1(); }
        #[divan::bench] fn part2() { day20::part2(); }
    }
    #[divan::bench_group()]
    mod day21 {
        #[divan::bench] fn part1() { day21::part1(); }
        #[divan::bench] fn part2() { day21::part2(); }
    }
    #[divan::bench_group()]
    mod day22 {
        #[divan::bench] fn part1() { day22::part1(); }
        #[divan::bench] fn part2() { day22::part2(); }
    }
    #[divan::bench_group()]
    mod day23 {
        #[divan::bench] fn part1() { day23::part1(); }
        #[divan::bench] fn part2() { day23::part2(); }
    }
    #[divan::bench_group()]
    mod day24 {
        #[divan::bench] fn part1() { day24::part1(); }
        #[ignore = "Solved by hand"]
        #[divan::bench] fn part2() { day24::part2(); }
    }
    #[divan::bench_group()]
    mod day25 {
        #[divan::bench] fn part1() { day25::part1(); }
    }
    
    
}

fn main() {
    // Run registered benchmarks.
    divan::main();
}