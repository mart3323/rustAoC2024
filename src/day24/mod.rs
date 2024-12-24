mod parse;

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, RwLock};
use std::task::{Context, Poll, Waker};
use std::thread;
use crate::day24::parse::{parse_network, Gate};
use crate::utils::read_input_file;

const DAY: &str = "day24";

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
pub fn part2() -> usize {
    todo!()
}