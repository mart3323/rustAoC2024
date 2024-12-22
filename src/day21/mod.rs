mod gpu;

use std::sync::mpsc;
use std::thread;
use crate::day21::gpu::part2_run_on_gpu;
use crate::utils::read_input_file;

const DAY: &str = "day21";


#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Hash)]
struct MonkeySecret(u32);
impl MonkeySecret {
    const PRUNE_MOD: u32 = 16777216;
    fn mix_mul_prune(&self, with: u32) -> Self {
        let mut nv = self.0 as u64;
        nv ^= self.0 as u64 * with as u64;
        nv %= Self::PRUNE_MOD as u64;
        MonkeySecret(nv as u32)
    }
    fn mix_div_prune(&self, with: u32) -> Self {
        let mut nv = self.0 as u64;
        nv ^= self.0 as u64 / with as u64;
        nv %= Self::PRUNE_MOD as u64;
        MonkeySecret(nv as u32)
    }

    fn next_secret(&self) -> Self {
        self.mix_mul_prune(64)
            .mix_div_prune(32)
            .mix_mul_prune(2048)
    }
    fn price(&self) -> u32 {
        self.0 % 10
    }
}

#[test]
fn test_monkey_secret() {
    let mut a = MonkeySecret(123);
    for expect in [15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254] {
        a = a.next_secret();
        assert_eq!(MonkeySecret(expect), a);
    }
    let expectations = vec!((1, 8685429), (10,4700978), (100,15273692), (2024, 8667524));
    for (start, expect) in expectations {
        let mut nr = MonkeySecret(start);
        for _ in 0..2000 {
            nr = nr.next_secret();
        }
        assert_eq!(MonkeySecret(expect), nr);
    }
}

pub fn part1() -> u64 {
    let file_contents = read_input_file(DAY, "full.txt");
    let mut sum: u64 = 0;
    for line in file_contents.lines() {
        let initial_number = str::parse(line.trim_end()).expect("All numbers to parse successfully");
        let mut nr = MonkeySecret(initial_number);
        for _ in 0..2000 {
            nr = nr.next_secret();
        }
        sum += nr.0 as u64;
    }
    return sum
}

#[test]
fn test_part2_gpu() {
    let profits = part2_run_on_gpu(vec!(1, 2, 3, 2024));
    let max = profits.into_iter().max();
    println!("{:?}", max);
    assert_eq!(Some(23), max);
}
fn part2_gpu() -> u32 {
    let file_contents = read_input_file(DAY, "full.txt");
    let monkeys: Vec<u32> = file_contents.lines().into_iter().map(str::trim_end).map(str::parse).collect::<Result<Vec<u32>, _>>().expect("To parse correctly");
    // println!("{:?}", monkeys);
    let profits = part2_run_on_gpu(monkeys);
    // println!("{:?}", profits);
    if let Some(profit) = profits.iter().max() {
        return *profit;
    } else {
        panic!();
    }
}

#[test]
fn test_part2_memory() {
    let max = part2_memory(vec!(1, 2, 3, 2024));
    assert_eq!(23, max);
}
fn part2_memory(monkeys: Vec<u32>) -> u32 {
    let mut total_reward_for_pattern = vec![0; 19*19*19*19];
    for monkey in monkeys {
        let mut reward_for_pattern = vec![0; 19*19*19*19];
        let mut secret = MonkeySecret(monkey);
        let mut diffs = [0,0,0,0];
        for i in 0..2000 {
            let prev = secret;
            secret = secret.next_secret();
            diffs = [diffs[1], diffs[2], diffs[3], secret.price() as i8 - prev.price() as i8];
            if 3 < i {
                let [d,c,b,a] = [diffs[0]+9, diffs[1]+9, diffs[2]+9, diffs[3]+9];
                let index: usize = ((a as u32) + (b as u32)*19 + (c as u32)*19*19 + (d as u32)*19*19*19) as usize;
                if reward_for_pattern[index] == 0 {
                    reward_for_pattern[index] = secret.price()+1
                }
            }
        }
        for (i,v) in reward_for_pattern.iter().enumerate() {
            total_reward_for_pattern[i] += if *v == 0 { 0 } else { v-1 };
        }
    }
    return total_reward_for_pattern.into_iter().max().unwrap()
}
pub fn part2() -> u32 {
    let file_contents = read_input_file(DAY, "full.txt");
    let monkeys: Vec<u32> = file_contents.lines().into_iter().map(str::trim_end).map(str::parse)
        .collect::<Result<Vec<u32>, _>>().expect("To parse correctly");

    let total_price = part2_memory(monkeys);
    return total_price;
}