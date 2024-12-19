use nom::Parser;
use std::collections::HashMap;
use std::ops::Shl;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread;
use std::thread::yield_now;
use std::time::Duration;
use pathfinding::num_traits::PrimInt;

const DAY: &str = "day17";

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    /// A = A / 2^[cmb](ProcessState::read_combo)
    adv(u8)=0,
    /// B = B xor OP
    bxl(u8)=1,
    /// B = [cmb](ProcessState::read_combo) % 8
    bst(u8)=2,
    /// Jump to OP if A != 0
    jnz(u8)=3,
    /// B = B xor C
    bxc=4,
    /// out <-- combo % 8
    out(u8)=5,
    /// B = A / 2´[cmd](ProcessState::read_combo)
    bdv(u8)=6,
    /// C = A / 2´[cmd](ProcessState::read_combo)
    cdv(u8)=7,
}
impl TryFrom<(u8, u8)> for Instruction {
    type Error = String;

    fn try_from((opcode, operand): (u8, u8)) -> Result<Self, Self::Error> {
        if operand > 8 { return Err(String::from("Memory corrupt: Operand larger than 8")); }
        match opcode {
            0 => Ok(Instruction::adv(operand)),
            1 => Ok(Instruction::bxl(operand)),
            2 => Ok(Instruction::bst(operand)),
            3 => Ok(Instruction::jnz(operand)),
            4 => Ok(Instruction::bxc),
            5 => Ok(Instruction::out(operand)),
            6 => Ok(Instruction::bdv(operand)),
            7 => Ok(Instruction::cdv(operand)),
            _ => Err(String::from("Memory corrupt: Operand larger than 8"))
        }
    }
}

type Program = Vec<u8>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ProcessState<'p> {
    program: &'p Program,
    program_counter: usize,
    reg_a: isize,
    reg_b: isize,
    reg_c: isize,
}

impl<'p> ProcessState<'p> {
    fn start(program: &'p Program) -> Self {
        ProcessState {
            program,
            program_counter: 0,
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
        }
    }
    fn read_combo(&self, op: u8) -> isize {
       match op {
           0|1|2|3 => op as isize,
           4 => self.reg_a,
           5 => self.reg_b,
           6 => self.reg_c,
           7 => panic!("Unsupported combo op 7"),
           _ => panic!("HW error: operand out of range"),
       }
    }
    fn restore_snapshot(program: &'p Program, register_a: isize, register_b: isize, register_c: isize) -> Self {
        ProcessState::<'p> {
            program,
            program_counter: 0,
            reg_a: register_a,
            reg_b: register_b,
            reg_c: register_c,
        }
    }
    fn is_done(&self) -> bool {
        self.program_counter + 1 >= self.program.len()
    }
    fn run_tick(&mut self) -> Result<Option<u8>, String> {
        if self.is_done() {
            return Err(String::from("Program has already exited"));
        };
        let opcode = self.program[self.program_counter];
        let operand = self.program[self.program_counter + 1];
        let operation = Instruction::try_from((opcode, operand))?;
        match operation {
            // op-using instructions
            Instruction::adv(op) => { self.reg_a = self.reg_a / 2isize.pow(self.read_combo(op) as u32) }
            Instruction::bdv(op) => { self.reg_b = self.reg_a / 2isize.pow(self.read_combo(op) as u32) }
            Instruction::cdv(op) => { self.reg_c = self.reg_a / 2isize.pow(self.read_combo(op) as u32) }
            Instruction::bxl(op) => { self.reg_b ^= op as isize; }
            Instruction::bst(op) => { self.reg_b = self.read_combo(op) % 8; }
            // opless instructions
            Instruction::bxc => { self.reg_b ^= self.reg_c; }
            // control flow instructions
            Instruction::jnz(op) => {
                if self.reg_a != 0 {
                    self.program_counter = op as usize;
                    if op >= self.program.len() as u8 {
                        return Err(String::from("Jump instruction exceeds program bounds"))
                    }
                    return Ok(None)
                }
            }
            // io instructions
            Instruction::out(op) => {
                let c = self.read_combo(op) % 8;
                self.program_counter += 2;
                return Ok(Some(c as u8));
            }
        }
        self.program_counter += 2;
        return Ok(None);
    }

    fn run_to_completion(&mut self, stdout: &Sender<u8>) -> Result<(), String> {
        // keep running until finished
        while !self.is_done() {
            if let Ok(result) = self.run_tick() {
                if let Some(c) = result {
                    println!("{}: {:?}", c, self);
                    stdout.send(c).map_err(|e| e.to_string())?;
                }
            }
        }
        return Ok(())
    }
}

#[test]
fn test_example_programs() {
    let program1 = Program::from(vec!(2, 6));
    let program2 = Program::from(vec!(5, 0, 5, 1, 5, 4));
    let program3 = Program::from(vec!(0, 1, 5, 4, 3, 0));
    let program4 = Program::from(vec!(1, 7));
    let program5 = Program::from(vec!(4, 0));

    let mut process1 = ProcessState::restore_snapshot(&program1, 0, 0, 9);
    let mut process2 = ProcessState::restore_snapshot(&program2, 10, 0, 0);
    let mut process3 = ProcessState::restore_snapshot(&program3, 2024, 0, 0);
    let mut process4 = ProcessState::restore_snapshot(&program4, 0, 29, 0);
    let mut process5 = ProcessState::restore_snapshot(&program5, 0, 2024, 43690);

    let channel1 = channel();
    let channel2 = channel();
    let channel3 = channel();
    let channel4 = channel();
    let channel5 = channel();
    // All should run successfully
    process1.run_to_completion(&channel1.0).expect("Program 1 should finish");
    process2.run_to_completion(&channel2.0).expect("Program 2 should finish");
    process3.run_to_completion(&channel3.0).expect("Program 3 should finish");
    process4.run_to_completion(&channel4.0).expect("Program 4 should finish");
    process5.run_to_completion(&channel5.0).expect("Program 5 should finish");

    // Program states & outputs should be as described
    assert_eq!(process1.reg_b, 1);
    assert_eq!(channel2.1.try_iter().collect::<Vec<u8>>(), vec!(0,1,2));
    assert_eq!(channel3.1.try_iter().collect::<Vec<u8>>(), vec!(4,2,5,6,7,7,7,7,3,1,0));
    assert_eq!(process3.reg_a, 0);
    assert_eq!(process4.reg_b, 26);
    assert_eq!(process5.reg_b, 44354);
}

// fn find_reg_a_value_to_produce_a_quine(program: &Program, reg_b: isize, reg_c: isize) -> Result<usize, String> {
//     fn program(A: u32) -> u32 {
//         let B = (A & 0b111) ^ 0b101 ;
//         let C = A.rotate_right(B) & 0b111;
//         let B = B ^ 0b110;
//         let B = B ^ C & 0b111;
//         return B;
//     }
//     let a = 0usize;
//     
//     let cache_size = Arc::new(AtomicUsize::new(0));
//     let current_reg_a = Arc::new(AtomicUsize::new(0));
// 
//     thread::scope(|s| {
//         let mut produces: HashMap<ProcessState, Vec<u8>> = HashMap::new();
//         
//         fn get_reverse_remainder<'a>(produces: &mut HashMap<ProcessState<'a>, Vec<u8>>, processState: &mut ProcessState<'a>, depth: usize) -> Vec<u8>{
// 
//             if depth > 100 {
//                 panic!("Yeah it's actually pretty deep")
//             }
//             if processState.is_done() {
//                 return vec!();
//             }
//             match produces.get(processState) {
//                 Some(rem) => rem.clone(),
//                 None => match processState.run_tick() {
//                     Ok(Some(c)) => {
//                         let mut remainder = get_reverse_remainder(produces, processState, depth+1);
//                         remainder.push(c);
//                         produces.insert(processState.clone(), remainder.clone());
//                         remainder
//                     },
//                     Ok(None) => {
//                         let remainder = get_reverse_remainder(produces, processState, depth+1);
//                         produces.insert(processState.clone(), remainder.clone());
//                         remainder
//                     },
//                     Err(_) => panic!("Program encountered an error"),
//                 }
//             }
//         }
// 
//         let csclone = Arc::clone(&cache_size);
//         let raclone = Arc::clone(&current_reg_a);
//         let (stopsend, stoprecv) = channel();
//         s.spawn(move || {
//             let mut reg_a = 0usize;
//             loop {
//                 yield_now();
//                 let mut process = ProcessState::restore_snapshot(&program, reg_a as isize, reg_b, reg_c);
// 
//                 let remainder = get_reverse_remainder(&mut produces, &mut process, 0);
//                 let will_produce = remainder.into_iter().rev().collect::<Vec<u8>>();
//                 
//                 if will_produce.len() == program.len() && will_produce.iter().enumerate().all(|(i,v)| program.get(i).is_some_and(|v2| v2 == v)) {
//                     stopsend.send(()).expect("Stop signal to send successfully");
//                     return reg_a;
//                 }
//                 reg_a += 1;
// 
//                 csclone.store(reg_a, Relaxed);
//                 raclone.store(produces.len(), Relaxed);
//             }
//         });
// 
//         let csclone = Arc::clone(&cache_size);
//         let raclone = Arc::clone(&current_reg_a);
//         s.spawn(move || {
//             loop {
//                 thread::sleep(Duration::from_secs(1));
//                 if let Ok(_) = stoprecv.try_recv() {
//                     break;
//                 } else {
//                     let cs = csclone.load(Relaxed);
//                     let ra = raclone.load(Relaxed);
//                     println!("Checked {ra} values, cache size is {cs} entries")
//                 }
//             }
//         });
//     });
// 
//     Ok(current_reg_a.load(Relaxed))
// }


#[test]
fn test_part1() {
    let program = Program::from(vec!(0,1,5,4,3,0));

    let mut process = ProcessState::restore_snapshot(&program, 729,0,0);
    let (sender, receiver) = channel();
    process.run_to_completion(&sender).unwrap();
    let values: Vec<u8> = receiver.try_iter().collect();
    assert_eq!(values, vec!(4,6,3,5,6,3,5,2,1,0));
}

#[test]
fn test_failed_submissions() {
    let program = Program::from(vec!(2,4,1,5,7,5,1,6,4,3,5,5,0,3,3,0));
    let mut process = ProcessState::restore_snapshot(&program, 47792830,0,0);
    let (sender, receiver) = channel();
    process.run_to_completion(&sender).unwrap();
    let values: Vec<u8> = receiver.try_iter().collect();

    assert_ne!(values, vec!(4, 6, 3, 5, 6, 3, 5, 2, 1, 0));
}

pub fn part1() -> String {
    let program = Program::from(vec!(2,4,1,5,7,5,1,6,4,3,5,5,0,3,3,0));
    let mut process = ProcessState::restore_snapshot(&program, 47792830,0,0);

    let (sender, receiver) = channel();
    process.run_to_completion(&sender).unwrap();
    let values: Vec<u8> = receiver.try_iter().collect();
    let values: Vec<String> = values.into_iter().map(|s| s.to_string()).collect();
    return values.join(",");
}

#[test]
fn test_part2() {
    let program = Program::from(vec!(0,3,5,4,3,0));
}
#[test]
fn test_manual_solution() {
}
pub fn part2() -> usize {
    let program = Program::from(vec!(2,4,1,5,7,5,1,6,4,3,5,5,0,3,3,0));

    // Value is at least 134217728
    let reg_a: usize = 0b010_001_100_100_111_001_111_001_001_101_111_100_110_101_111_011;
    let reg_a: usize = 0b011_111_101_110_100_111_101_001_001_111_001_111_100_100_001_010;
    let mut process = ProcessState::restore_snapshot(&program, reg_a as isize,0,0);
    let (sender, receiver) = channel();

    process.run_to_completion(&sender).expect("Program should finish");
    let x: Vec<u8> = receiver.try_iter().collect();
    assert_eq!(program, x);
    return 0
}

fn part2_solve_one_digit(previous: u8, expect: u8) -> Vec<u8>{
    let mut options = Vec::with_capacity(8);
    for b in 0b000..0b111 {
        if program(previous.shl(3) + b) == expect {
            options.push(b)
        }
    }
    return options;
}
fn program(A: u8) -> u8 {
    let B = (A & 0b111) ^ 0b101 ;
    let C = A.rotate_right(B as u32) & 0b111;
    let B = B ^ 0b110;
    let B = B ^ C & 0b111;
    return B as u8;
}
fn solve_part2_recursive(previous: u8, expect: &[u8]) -> Option<Vec<u8>> {
    println!("{},{:?}", previous, expect);
    if let Some(n) = expect.first() {
        let mut options = part2_solve_one_digit(previous, *n);
        options.sort_by(|a,b| b.cmp(a));
        println!("options: {:?}", options);
        for opt in options {
            if let Some(mut result) = solve_part2_recursive(previous*8+opt, &expect[1..]) {
                result.push(opt);
                return Some(result);
            };
        };
        return None;
    } else {
        return Some(vec!());
    }
}
#[test]
fn solve_part2() {
    let res = solve_part2_recursive(0, &[0, 3,4,5,3, 0]);
    if let Some(result) = res {
        println!("{:#?}", result);
    };
}
