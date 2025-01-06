mod parse;

use crate::parse::{parse_network, Gate};
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::task::Waker;
use std::thread;
use nom::bits::bits;
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
    let network = parse_network(&read_input_file(DAY, "demo.txt")).expect("Failed to parse network").1;
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
    let network = parse_network(&read_input_file(DAY, "full.txt")).expect("Failed to parse network").1;
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
    let network = parse_network(&read_input_file(DAY, "demo2.txt")).expect("Failed to parse network").1;


    let bits_of_output = (0..).find(|i| {
        !network.gates.iter().any(|gate| gate.output() == format!("z{:0>2}", i))
    }).expect("Found the first non-existing gate");

    struct SearchState {
        prev_layer_overflow_bits: Vec<String>,
        swaps: Vec<(String, String)>
    }
    fn next_states(gates: Vec<Gate>, state: SearchState, bit: usize) -> Vec<SearchState> {
        let out = vec!();
        let name_a = format!("a{:0>2}", bit);
        let name_b = format!("b{:0>2}", bit);
        let name_z = format!("z{:0>2}", bit);
        
        for swaps in state.swaps.len()..=8 {
            let swaps = 8 - swaps;
            let swap_options: Vec<Vec<[&Gate;2]>> = gates.iter()
                .array_combinations::<2>()
                .combinations(swaps)
                .collect();
            
            for swaps in swap_options {
                [true,false].iter().zip([true, false]).zip([true, false]).all(|((a,b),overflow)| {
                    if overflow && state.prev_layer_overflow_bits.is_empty() {
                        // Do not check combos with overflow bit true if the previous layer has no overflow
                        //   We do not consider states with no overflow as valid, so this edge case is solely for the initial state
                        return true;
                    }
                    let mut wires = HashMap::new();
                    wires.insert("false", false);
                    wires.insert(&name_a, a);
                    wires.insert(&name_b, b);
                    state.prev_layer_overflow_bits.iter().for_each(|name| {
                        wires.insert(name, overflow);
                    });

                    'propagate_signal: loop {
                        let mut changed = false;
                        for gate in &gates {
                            let [a,b] = gate.inputs();
                            let o = gate.output();
                            if !wires.contains_key(o) {
                                if let (Some(a), Some(b)) = (wires.get(a), wires.get(b)) {
                                    let mut o = o;
                                    for [&swap_a, &swap_b] in &swaps {
                                        match (swap_a, swap_b) {
                                            (from, to) if from.output() == o => {
                                                o = to.output()
                                            }
                                            (to, from) if from.output() == o => {
                                                o = to.output()
                                            },
                                            _ => {}
                                        }
                                    }
                                    let o = o;
                                    wires.insert(o, gate.process(*a, *b));
                                    changed = true;
                                }
                            }
                        }
                        if !changed {
                            break;
                        }
                    }

                    let expect_carry = [*a,b,overflow].iter().filter(|bit| **bit).count();
                    if wires.get(name_z.as_str()).is_some_and(|v| *v == a ^ b ^ overflow) {
                        let overflow = wires.into_iter().filter_map(|(name,value)| {
                            todo!()
                        });
                    }
                    todo!()
                    
                    // TODO: Only take the next state if
                    //       The output bit is correct
                    //       There is at least one bit acting like an overflow bit OR we are the last bit
                    //       All of the chosen swaps have an output (gates that don't participate in this layer should not predictively be swapped, they would have swapped already if lower, or will get swapped in a later iteration if higher
                });
            }
        }
        
        return out;
    }
    let mut search_state = vec!(SearchState{prev_layer_overflow_bits: vec!(String::from("false")), swaps: vec!()});
    
    for bit in 0..bits_of_output {
        println!("Processing {}", bit);
        let next_search_state: Vec<SearchState> = vec!();
        for prev_state in search_state {
            for a in [true, false] {
                for b in [true, false] {
                    for overflow in [true, false] {
                        let mut wires: HashMap<String, bool> = HashMap::new();
                        // Inject overflow and a/b bits
                        wires.insert(prev_state.prev_layer_overflow_bit.clone(), overflow);
                        wires.insert(format!("a{:0>2}", bit), a);
                        wires.insert(format!("b{:0>2}", bit), b);
                        // Simulate gates
                        'run_gates_until_nothing_changes: loop {
                            let mut changed = false;
                            for gate in &network.gates {
                                let [a, b] = gate.inputs();
                                if let (Some(a), Some(b)) = (wires.get(a), wires.get(b)) {
                                    wires.insert(gate.output().to_owned(), gate.process(*a, *b));
                                    changed = true;
                                }
                            }
                            if !changed {
                                break;
                            }
                        }
                        // Test for correct output
                        let (sum, carry) = match a as u8 + b as u8 + overflow as u8 {
                            0 => (false, false),
                            1 => (true, false),
                            2 => (false, true),
                            _ => panic!("This should never happen"),
                        };
                        if wires.get(format!("z{:0>2}", bit)).is_some_and(|v| *v == sum) {
                            let overflow_wires: Vec<String> = wires.iter().filter(|k, v| *v == carry).collect();
                        }
                    }
                }
            }
        }
        search_state = next_search_state
        // TODO: Need a new simulation method which can handle partial inputs
        //       it should either
        //         a: Know the difference between "Value not yet known" and "Value is unknown"
        //            (in that case i feed explicit "unknown" into all other inputs bits)
        //            .
        //         b: Instead of threads, propagate the changes directly, so we know we are done
        //            when we process the last gate and there is no other gate connected to its output,
        //            or the last gate doesn't get processed because it only has 1 input
        //
        //  Maybe try building it with Rcs
    }
    println!("{:?}", bits_of_output);
}
pub fn part2() -> usize {
    // Note: Reminder, the network should ADD the binary numbers
    //       (I keep incorrectly remembering that it's doing a bitwise AND or something)
    // 
    // Note: When calculating the N'th bit, any *higher* bits in the input have no impact
    //       So almost certainly i want to iterate upwards, testing just the smallest bit first and going up from there
    //
    // TODO: This will not work | Iterate the input numbers, checking only the last bit at first and going up from there
    //       This will not work | When an error is found, start considering pairs of gates
    //       This will not work | where gate 1 is involved in the current wrong bit
    //       This will not work | and gate 2 is any gate not locked in yet.
    //       This will not work | Try swapping the pair. If it fixes the current bit, add that to the list of possible solutions
    //       This will not work |     NB: The list of "locked-in" gates would depend on the swapping choices made earlier, so it has to be cloned or something
    //       This will not work |     Search state:
    //       This will not work |             int:                      Current bit under consideration
    //       This will not work |             Vec<string>:              Gates which are used by earlier bits and thus locked in
    //       This will not work |             Vec(8)<(string, string)>: Gates which have been swapped
    //   Note: Problem: For higher bits, way too many combinations of lower bits have to be considered
    // TODO: Solution?: Maybe it's okay to only do the three options for the lower bit? (0, 1, and 2(overflow))
    //            Note: I do not think this is true. The layer under consideration could *appear* to work correctly only for some inputs if it was checking only for certain overflow conditions
    //                  For example instead of the real overflow condition, it might only check for the two previous input bits
    //                  By adding OR gates, it could be extended to any number of conditions, so ruling this out would require testing all numbers
    //
    // TODO: Can i, when validating each layer, explicitly pick out and mark which gates are known to behave as "the overflow" bit
    //       And when validating the next layer, ensure that feeding in only the previous layer's "overflow" bit will give the correct result?
    //           Almost sure this should work. By not feeding in previous input bits, any other wiring that is only sometimes okay would not pass
    //           Note: Problem: Instead of hooking into a previous layer's overflow bit, there could be a duplicate circuit calculating the overflow
    //                Solution: When testing a layer, check all gates. Mark all wires that behave as the previous layer's overflow
    //                     This will handle duplicates. By feeding the next layer only from the input bits and detected 'overflows', we ensure the new layer will behave correctly for ALL inputs
    //                     since using any unknown input would prevent the output from forming, and all known inputs are already tested
    //
    // TODO: Implementation plan
    //   For each bit index 0 to ?:
    //     Feed signals x[i], y[i], and overflow[i-1] into the network,
    //     The combination is valid if the output is correct and there is at least one wire which holds the overflow signal
    //     .
    //     for j in 0..(remaining_swap_counts) {
    //       Choose j pairs of wires, which have different signals, and test the network with those pairs swapped.
    //         Add any pairs which make the network valid to the current search
    //     Note: Print the current size of the search space at each step to verify it's not blowing up

    todo!()
}