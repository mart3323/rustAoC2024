use nom::Parser;
use std::sync::mpsc::{channel, Sender};
use std::thread;

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

#[derive(Debug, Clone)]
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
    fn run_tick(&mut self, stdout: &Sender<u8>) -> Result<bool, String> {
        if self.program_counter + 1 >= self.program.len() {
            return Ok(true)
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
                    return Ok(false)
                }
            }
            // io instructions
            Instruction::out(op) => {
                let c = self.read_combo(op) % 8;
                stdout.send((c) as u8)
                    .map_err(|e| e.to_string())?;
            }
        }
        self.program_counter += 2;
        return Ok(false);
    }

    fn run_to_completion(&mut self, stdout: &Sender<u8>) -> Result<(), String> {
        while !self.run_tick(stdout)? {
            // keep running until finished
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

fn find_reg_a_value_to_produce_a_quine(program: &Program, reg_b: isize, reg_c: isize) -> Result<usize, String> {
    for n in 0..32 {
        let start = 2usize.pow(n);
        let end = 2usize.pow(n+1);
        println!("Testing programs {start}-{end}");
        'search:
        for reg_a in start..end {
            let (sender, receiver) = channel();
            let mut process = ProcessState::restore_snapshot(&program, reg_a as isize, reg_b, reg_c);
            let mut index = 0;
            while let Ok(finished) = process.run_tick(&sender) {
                if finished {
                    if index == program.len() {
                        return Ok(reg_a as usize);
                    } else {
                        break 'search; // Did not output full program
                    }
                }
                if let Ok(v) = receiver.try_recv() {
                    if let Some(&v2) = program.get(index) {
                        if v != v2 {
                            continue 'search;
                        } else {
                            index += 1
                        }
                    } else {
                        continue 'search;
                    }
                }
            }
        }
    }
    Err(String::from("No solutions found"))
}


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
    let reg_a = find_reg_a_value_to_produce_a_quine(&program, 0, 0);
    
    assert_eq!(reg_a, Ok(117440))
}

pub fn part2() -> usize {
    let program = Program::from(vec!(2,4,1,5,7,5,1,6,4,3,5,5,0,3,3,0));
    // Value is at least 134217728
    let reg_a = find_reg_a_value_to_produce_a_quine(&program, 0, 0);
    if let Ok(reg_a) = reg_a {
        return reg_a;
    } else {
        panic!("No solutions found")
    }
}