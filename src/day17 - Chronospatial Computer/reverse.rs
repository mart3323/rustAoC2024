use super::*;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;

/*
My program
   : do {
2 4:   B = A % 3 bits
1 5:   B ^= 101
7 5:   C = A << B
1 6:   B ^= 110
4 3:   B ^= C
5 5:   print(B)
0 3:   A << 3
3 0: } while A != 0

Simplified:
  For every 3 bit grouping in A as GROUP, starting from least significant:
    print(GROUP ^ 011 ^ (A << (GROUP ^ 101)))

How to reverse?
Start processing output values in reverse. Each output value determines the possible values of the current GROUP
(groups processed this way provide bits from most to least significant)
On the first iteration, all offset values assumed to be 0
On deeper iterations, offset values depend on the values determined previously
*/

fn possible_inputs(out: u8, reg_a: u64) -> Vec<u8> {
    // print(GROUP ^ 011 ^ (A << (GROUP ^ 101)))
    // When doing it by hand i had a fancier method, but just to make sure i don't make an error
    // let's just brute force it. It's only 8 possible values, the difference is trivial
    let mut inputs = Vec::with_capacity(8);
    for candidate in 0..8u8 {
        let new_reg_a = (reg_a << 3) + candidate as u64;
        
        let mut b = ((new_reg_a % 8)) as u8;
        b ^= 0b101;
        let c = ((new_reg_a >> b) % 8) as u8;
        b ^= 0b110;
        b ^= c;
        if b == out {
            inputs.push(candidate);
        }
    }
    
    return inputs;
}
#[test]
fn test_possible_inputs() {
    assert_eq!(vec!(0b011), possible_inputs(0, 0));
    assert_eq!(vec!(0b000, 0b001, 0b101, 0b111), possible_inputs(3, 0b11));
}

// Remaining outputs from end to start
fn decompile_sequence_recursive(mut reg_a: u64, remaining: Vec<u8>) -> Option<Vec<u8>> {
    if remaining.is_empty() {
        return Some(vec!());
    } else {
        let expect = *remaining.iter().next().unwrap();
        let remaining: Vec<u8> = remaining.into_iter().skip(1).collect();
        let candidates = possible_inputs(expect, reg_a);
        for i in candidates {
            match decompile_sequence_recursive((reg_a << 3) + i as u64, remaining.clone()) {
                None => {  },
                Some(mut rem) => {
                    rem.insert(0, i);
                    return Some(rem) // Recombine start to end
                }
            }
        }
        return None;
    }
}
fn decompile_sequence(program: &Program) -> Option<u64> {
    let expect = program.clone().into_iter().rev().collect();
    let option = decompile_sequence_recursive(0, expect);
    match option {
        None => None,
        Some(trits) => {
            let mut out = 0u64;
            for trit in trits {
                out = (out << 3) + trit as u64
            }
            Some(out)
        }
    }
}
#[test]
fn test_solve_part2() {
    let program = vec!(2, 4, 1, 5, 7, 5, 1, 6, 4, 3, 5, 5, 0, 3, 3, 0);
    let solution = solve_part2(&program);
    println!("Answer: reg_a = {}", solution)
}
pub fn solve_part2(program: &Vec<u8>) -> u64 {
    let reg_a = decompile_sequence(&program);
    let reg_a = reg_a.expect("Failed to find solution");
    assert!(reg_a < 139543356615066);

    // Validate
    let (sender, receiver) = mpsc::channel();
    ProcessState::restore_snapshot(&program, reg_a as isize, 0, 0).run_to_completion(&sender).unwrap();
    for i in program {
        assert_eq!(receiver.try_recv(), Ok::<u8, TryRecvError>(*i));
    }
    assert_eq!(receiver.try_recv(), Err(TryRecvError::Empty));

    return reg_a
}