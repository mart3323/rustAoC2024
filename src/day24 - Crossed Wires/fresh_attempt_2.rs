use std::collections::HashMap;
use std::rc::Rc;

// Note: Problem. Original implementation plan of looking at only one bit at a time could fail
//       if there are gates that use other inputs but whose values always get cancelled out
//       for example
//       a00--------------------
//                              \
//       b00---XOR>--XOR>--XOR>--AND>--z00
//            /    /      /
//         a12----/-XOR>-
//               / /
//           b17--
//       z00 = (a00 & (((b00 XOR a12) XOR b17) XOR (a12 XOR b17))
//       z00 = (a00 & (b00 XOR a12 XOR b17 XOR a12 XOR b17)
//       z00 = (a00 & (b00 XOR a12 XOR a12)
//       z00 = (a00 & (b00))
//       z00 = (a00 & b00)
//       I do no think there is any easy way to find and eliminate such dead wires


#[derive(Debug)]
struct Network {
    // (index) => Option<bool>
    // Wire I has signal _
    signals: Vec<Option<bool>>,
    // (index) => ConnectedGate
    // Gate I is _
    gates: Vec<ConnectedGate>,
    // (index) => vec<jindex>
    // wire I is connected into gate j
    wires: Vec<Vec<usize>>,
    // Trade memory for speed by using a hashmap
    // (name) => index
    names: HashMap<Rc<str>, usize>
}
impl Default for Network {
    fn default() -> Network {
        Network {
            signals: vec!(),
            gates: vec!(),
            wires: vec!(),
            names: HashMap::new()
        }
    }
}
#[derive(thiserror::Error, Debug)]
enum NetworkError {
    #[error("Name {0} is not present in this network")]
    IllegalName(String),
    #[error("Wire {0} has a short circuit (multiple gates tried to write to it)")]
    ShortCircuit(String),
}
impl Network {
    fn register_wire(&mut self, name: &str) -> usize {
        match self.wire_name_to_id(name) {
            Some(index) => *index,
            None => {
                let index = self.wires.len();
                self.wires.push(vec!());
                self.signals.push(None);
                self.names.insert(name.into(), index);
                index
            }
        }
    }
    fn wire_name_to_id(&self, name: &str) -> Option<&usize> {
        self.names.get(name)
    }
    /// 1. Write the provided value to the provided wire
    /// 2. Run any logic gates which take this wire as input
    /// 3. Recursively write the outputs of those gates
    /// 
    /// Recursion is depth first. i.e. after writing to this wire, the next write is from the first connected gate (assuming it has both inputs), then the first gate connected to *that* one
    /// ```text
    /// a --> b --> c
    ///  \---> d --> e
    ///         \--> f
    /// ```
    /// Order of writes: a, b, c, d, e, f
    /// 
    /// A [NetworkError::ShortCircuit] is thrown if a wire that already has a value is written to. This includes [name], but also any recursive writes by connected gates
    pub fn write(&mut self, name: &str, value: bool) -> Result<(), NetworkError> {
        let index = self.wire_name_to_id(name).ok_or_else(|| NetworkError::IllegalName(name.to_owned()))?;
        self.write_internal(*index, value)
    }
    fn write_internal(&mut self, index: usize, value: bool) -> Result<(), NetworkError> {
        if self.signals[index] != None {
            let name = self.names.iter().find(|(k, v)| **v == index).unwrap().0.to_string();
            return Err(NetworkError::ShortCircuit(name));
        }
        // Write to target wire
        self.signals[index] = Some(value);
        
        // Process gates connected to this wire, collecting any newly output values
        let mut affected = HashMap::new();
        self.wires[index].iter().for_each(|&gate| {
            let gate= &self.gates[gate];
            if let (Some(a), Some(b)) = (self.signals[gate.a], self.signals[gate.b]) {
                affected.insert(gate.out, gate.process(a,b));
            };
        });
        // Recursively write these new values out to the network as well
        for (index, value) in affected {
            self.write_internal(index, value)?;
        }
        Ok(())
    }
    /// Pulls the requested wire to ground, deleting the signal on that wire as well as any wires that depend on it
    /// 
    /// For example, with the following network where gates a,b,c,d,e,f all have emitted outputs (suppose there are other gates hidden from the diagram)
    /// pulling 'd' to ground will delete the signals d, e, and f
    /// ```text
    /// a --> b --> c
    ///  \---> d --> e
    ///         \--> f
    /// ```
    /// 
    /// If the wire doesn't have any signal, then this is a no-op
    pub fn ground(&mut self, name: &str) -> Result<(), NetworkError> {
        let index = *self.wire_name_to_id(name).ok_or_else(|| NetworkError::IllegalName(name.to_owned()))?;
        return self.ground_internal(index);
    }
    pub fn ground_internal(&mut self, index: usize) -> Result<(), NetworkError> {
        if self.signals[index] == None {
            return Ok(()); // Already grounded, so no update necessary
        }
        // Clear this wire
        self.signals[index] = None;

        // Collect wires that depend on the just-cleared wire
        let next_indexes: Vec<usize> = self.wires[index].iter().map(|&i| self.gates[i].out)
            .collect();
        for next_index in next_indexes {
            self.ground_internal(next_index)?;
        }
        return Ok(());
    }
    pub fn attach_gate(&mut self, gate: Gate, input_a: &str, input_b: &str, output: &str) {
        let a = match self.wire_name_to_id(input_a) {
            Some(&index) => index,
            None => self.register_wire(input_a)
        };
        let b = match self.wire_name_to_id(input_b) {
            Some(&index) => index,
            None => self.register_wire(input_b)
        };
        let out = match self.wire_name_to_id(output) {
            Some(&index) => index,
            None => self.register_wire(output)
        };
        let gate = ConnectedGate {
            gate,
            a,
            b,
            out
        };
        self.gates.push(gate);
        let gate_index = self.gates.len() - 1;
        self.wires[a].push(gate_index);
        self.wires[b].push(gate_index);
    }
}
#[derive(Debug)]
struct ConnectedGate {
    gate: Gate,
    a: usize,
    b: usize,
    out: usize
}
impl LogicGate for ConnectedGate {
    fn process(&self, a: bool, b: bool) -> bool {
        self.gate.process(a,b)
    }
}
#[derive(Debug)]
enum Gate {
    Xor,
    And,
    Or
}
impl LogicGate for Gate {
    fn process(&self, a: bool, b: bool) -> bool {
        match self {
            Gate::Xor => {a ^ b}
            Gate::And => {a & b}
            Gate::Or => {a | b}
        }
    }
}

trait LogicGate {
    fn process(&self, a: bool, b: bool) -> bool;

    fn process_optional(&self, a: Option<bool>, b: Option<bool>) -> Option<bool> {
        match (a,b) {
            (Some(a),Some(b)) => Some(self.process(a,b)),
            _ => None
        }
    }
}

#[test]
fn test_network_propagates() {
    let mut network = Network::default();
    //    a00---&->z00
    //      b00/
    // 
    //   a01---+-->z01
    //     b01/
    // 
    //   a02---%-->c02---&-->z02
    //     b02/       \ /
    //                 X
    //   a03----------/-\+-->z03
    network.attach_gate(Gate::And, "a00", "b00", "z00");
    network.attach_gate(Gate::Or, "a01", "b01", "z01");
    network.attach_gate(Gate::Xor, "a02", "b02", "c02");
    network.attach_gate(Gate::And,"a03", "c02", "z02");
    network.attach_gate(Gate::Or, "a03", "c02", "z03");
    
    println!("{:?}", network);
    network.write("a00", true).expect("Write unsuccessful");
    network.write("a01", true).expect("Write unsuccessful");
    network.write("a02", true).expect("Write unsuccessful");
    network.write("a03", true).expect("Write unsuccessful");
    network.write("b00", true).expect("Write unsuccessful");
    network.write("b01", true).expect("Write unsuccessful");
    network.write("b02", true).expect("Write unsuccessful");

    //    a00(1)---&->(1)z00
    //      b00(1)/
    // 
    //   a01(1)---+-->(1)z01
    //     b01(1)/
    // 
    //   a02(1)---%-->c02(1)---&-->(1)z02
    //     b02(1)/          \ /
    //                       X
    //   a03(1)-------------/-\+-->(1)z03
    println!("{:?}", network);
    network.ground("a00");
    println!("{:?}", network);

}
