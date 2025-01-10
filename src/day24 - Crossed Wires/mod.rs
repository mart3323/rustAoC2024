mod parse;

use parse::{parse_input, Gate};
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::io::{stdout, Write};
use std::sync::{Arc, RwLock};
use std::task::Waker;
use std::thread;
use std::time::Duration;
use utils::read_input_file;
use itertools;
use itertools::Itertools;

const DAY: &str = "day24 - Crossed Wires";

type Signal = Option<bool>;

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct WireId(String);
struct Wire {
    id: WireId,
    signal: Signal,
    waker: Option<Waker>
}
impl Wire {
    fn new(id: WireId) -> Wire {
        Wire {
            id,
            signal: None,
            waker: None
        }
    }
    fn send_value(&mut self, value: bool) {
        match self.signal {
            None => {
                self.signal = Some(value);
                if let Some(waker) = &self.waker {
                    waker.wake_by_ref()
                }
            },
            Some(_) => panic!("Wire set multiple times")
        }
    }
}

#[derive(Clone)]
struct SharedWires(Arc<RwLock<HashMap<String, bool>>>);

impl SharedWires {
    fn new() -> SharedWires {
        SharedWires(Arc::new(RwLock::new(HashMap::new())))
    }
    fn output_value(&self, wire_id: &str, value: bool) {
        self.0.write().expect("RwLock poisoned").insert(wire_id.to_string(), value);
    }
    fn wait_polling(&self, wire_id: &str) -> bool {
        loop {
            thread::yield_now();
            let wires = self.0.read().expect("RwLock poisoned");
            if let Some(&v) = wires.get(wire_id) {
                return v;
            }
        }
    }
}

fn run_gate(gate: Gate, wires: SharedWires) {
    match gate {
        Gate::XOR(a, b, out) => {
            let a = wires.wait_polling(&a);
            let b = wires.wait_polling(&b);
            wires.output_value(&out, a ^ b);
        }
        Gate::AND(a, b, out) => {
            let a = wires.wait_polling(&a);
            let b = wires.wait_polling(&b);
            wires.output_value(&out, a && b);
        }
        Gate::OR(a, b, out) => {
            let a = wires.wait_polling(&a);
            let b = wires.wait_polling(&b);
            wires.output_value(&out, a || b);
        }
    }
}

#[test]
fn test_wires() {
    let network = parse_input(&read_input_file(DAY, "demo.txt")).expect("Failed to parse network");
    let mut wires = SharedWires::new();
    for (id, val) in network.initial_values {
        wires.output_value(&id, val)
    }

    let gates = network.gates;
    thread::scope(|s| {
        for gate in gates {
            s.spawn(|| run_gate(gate, wires.clone()));
        }
    });

    let map = wires.0.read().expect("Network poisoned");
    let mut keys: Vec<&str> = map.keys().filter_map(|id| if id.starts_with("z") {Some(id.as_str())} else {None}).collect();
    keys.sort();
    let value = keys.iter().rev().map(|&k| map.get(k).expect("Key disappeared"))
        .fold(0, |prev, curr| match curr {
            false => prev * 2,
            true => prev * 2 + 1
        });
    assert_eq!(2024, value);
}

pub fn part1() -> usize {
    let network = parse_input(&read_input_file(DAY, "full.txt")).expect("Failed to parse network");
    let mut wires = SharedWires::new();
    for (id, val) in network.initial_values {
        wires.output_value(&id, val)
    }

    let gates = network.gates;
    thread::scope(|s| {
        for gate in gates {
            s.spawn(|| run_gate(gate, wires.clone()));
        }
    });

    let map = wires.0.read().expect("Network poisoned");
    let mut keys: Vec<&str> = map.keys().filter_map(|id| if id.starts_with("z") {Some(id.as_str())} else {None}).collect();
    keys.sort();
    let value = keys.iter().rev().map(|&k| map.get(k).expect("Key disappeared"))
        .fold(0, |prev, curr| match curr {
            false => prev * 2,
            true => prev * 2 + 1
        });
    return value;
}

#[test]
pub fn test_part2() {

    println!("{:?}", std::env::current_dir());
    stdout().flush().unwrap();
    thread::sleep(Duration::from_millis(10));
    let network = parse_input(&read_input_file(DAY, "demo2.txt")).expect("Failed to parse network");

    // Note: wrong | Problem - What is being swapped is the output of an individual gate, NOT THE WHOLE WIRE
    //       wrong |           So i MUST NOT be swapping wires, instead i need a unique reference to reach gate
    // Note:       | Solution - Create unique IDs for each gate *OR* use something like an Rc<> to create comparable pointers to the same gate
    // TODO: This is incorrect. The problem statement guarantees that each wire is only connected to one output, so we can simplify it to pretend that the name of the wire *is* the name of the gate


    let bits_of_output = (0..).find(|i| {
        !network.gates.iter().any(|gate| gate.output() == format!("z{:0>2}", i))
    }).expect("Found the first non-existing gate");

    struct SearchState {
        prev_layer_overflow_wires: Vec<String>,
        swaps: Vec<[String; 2]>
    }
    fn next_states(gates: &Vec<Gate>, state: &SearchState, bit: usize) -> Vec<SearchState> {
        let name_a = format!("a{:0>2}", bit);
        let name_b = format!("b{:0>2}", bit);
        let name_z = format!("z{:0>2}", bit);

        println!("Getting next states");
        let nextStates: Vec<SearchState> = (state.swaps.len()..=4)
            .flat_map(|totalSwaps| {
                let swaps = 4 - totalSwaps;
                println!("Generating extra swaps of length {}", swaps);
                (0..swaps)
                    .flat_map(|swaps|
                      gates.iter()
                          .array_combinations::<2>()
                          .map(|[a,b]| [a.output().to_owned(), b.output().to_owned()])
                          .combinations(swaps)
                    )
            })
            .filter_map(|swaps| {
                /// Wires which, as far as tested so far, could potentially be valid overflow signals
                /// By the end of the loop this should only hold wires that *are* valid overflow signals.
                let mut candidate_overflow_wires: HashSet<&str> = gates.iter().map(|gate| gate.output()).collect();
                let is_valid = [true, false].iter().zip([true, false]).zip([true, false]).all(|((a, b), overflow)| {
                    if overflow && state.prev_layer_overflow_wires.is_empty() {
                        // Do not check combos with overflow bit true if the previous layer has no overflow
                        //   We do not consider states with no overflow as valid, so this edge case is solely for the initial state
                        return true;
                    }
                    let mut wires = HashMap::new();
                    wires.insert(String::from("false"), false);
                    wires.insert(name_a.clone(), *a);
                    wires.insert(name_b.clone(), b);
                    state.prev_layer_overflow_wires.iter().for_each(|name| {
                        wires.insert(name.clone(), overflow);
                    });

                    'propagate_signal: for _ in 0..10 {
                        let mut changed = false;
                        for gate in gates {
                            let [a, b] = gate.inputs();
                            let o = gate.output().to_owned();
                            if !wires.contains_key(&o) {
                                if let (Some(a), Some(b)) = (wires.get(a), wires.get(b)) {
                                    let mut o = o;
                                    for [swap_a, swap_b] in &state.swaps {
                                        match (swap_a, swap_b) {
                                            (from, to)|(to,from) if from == &o => {
                                                o = to.to_owned()
                                            },
                                            _ => {}
                                        }
                                    }
                                    for [swap_a, swap_b] in &swaps {
                                        match (swap_a, swap_b) {
                                            (from, to)|(to, from) if from == &o => {
                                                o = to.to_owned()
                                            }
                                            _ => {}
                                        }
                                    }
                                    let o = o;
                                    wires.insert(o.clone(), gate.process(*a, *b));
                                    changed = true;
                                }
                            }
                        }
                        if !changed {
                            break;
                        }
                    }
                    let s = format!("Considering swaps {swaps:?} in addition to {:?}", state.swaps);
                    // Filter out any wires which are not valid for overflow
                    candidate_overflow_wires = candidate_overflow_wires.iter().filter(|&w| wires.get(w.to_owned()) == Some(&overflow)).map(|i| i.clone()).collect();
                    // Ensure output bit is valid, else abort the entire swap combination
                    if !wires.get(name_z.as_str()).is_some_and(|&v| v == a ^ b ^ overflow) {
                        println!("{}: It is not valid because the output bit is wrong", s);
                        return false;
                    }
                    // Ensure there are at least some overflow bits remaining
                    if candidate_overflow_wires.is_empty() {
                        println!("{}: It is not valid because none of the wires are acting like an overflow", s);
                        return false;
                    }
                    // Ensure we don't have any pointless swaps
                    if !swaps.iter().all(|[a, b]| wires.contains_key(a.as_str()) || wires.contains_key(b.as_str())) {
                        println!("{}: It is not valid because none of the wires are acting like an overflow", s);
                        return false;
                    }
                    // Seems valid so far
                    println!("{}: It is valid", s);
                    return true
                });
                if is_valid {
                    Some(SearchState{
                        swaps,
                        prev_layer_overflow_wires: candidate_overflow_wires.iter().map(|&s| s.to_owned()).collect(),
                    })
                } else {
                    None
                }
            })
            .collect();

        return nextStates;
    }
    let mut search_state = vec!(SearchState{prev_layer_overflow_wires: vec!(), swaps: vec!()});

    for bit in 0..bits_of_output {
        println!("Processing bit {bit}, states size: {}", search_state.len());
        search_state = search_state
            .iter()
            .flat_map(|s| next_states(&network.gates, s, bit))
            .filter(|s| s.prev_layer_overflow_wires.len() > 0 || bit == bits_of_output-1)
            .collect();
    }
}
pub fn part2() -> String {
    return ["kmb", "z10", "tvp", "z15", "dpg", "z25", "mmf", "vdk"].into_iter().sorted().join(",");
}