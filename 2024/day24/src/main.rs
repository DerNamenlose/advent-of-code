use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

enum Op {
    AND,
    OR,
    XOR,
}

struct Gate {
    input1_name: &'static str,
    input2_name: &'static str,
    output_name: &'static str,
    op: Op,
}

impl Gate {
    fn calculate(
        &self,
        inputs: &HashMap<&'static str, bool>,
        gates: &HashMap<&'static str, Gate>,
    ) -> bool {
        let input1 = inputs.get(self.input1_name).cloned().unwrap_or_else(|| {
            gates
                .get(self.input1_name)
                .unwrap()
                .calculate(inputs, gates)
        });
        let input2 = inputs.get(self.input2_name).cloned().unwrap_or_else(|| {
            gates
                .get(self.input2_name)
                .unwrap()
                .calculate(inputs, gates)
        });
        match self.op {
            Op::AND => input1 && input2,
            Op::OR => input1 || input2,
            Op::XOR => input1 ^ input2,
        }
    }
}

lazy_static! {
    static ref GATE_REGEX: Regex =
        Regex::new(r"([\w\d]+) (AND|OR|XOR) ([\w\d]+) -> ([\w\d]+)").unwrap();
}

impl From<&'static str> for Gate {
    fn from(value: &'static str) -> Self {
        let matches = GATE_REGEX.captures(value).unwrap();
        Gate {
            input1_name: matches.get(1).unwrap().as_str(),
            input2_name: matches.get(3).unwrap().as_str(),
            output_name: matches.get(4).unwrap().as_str(),
            op: match matches.get(2).unwrap().as_str() {
                "AND" => Op::AND,
                "OR" => Op::OR,
                "XOR" => Op::XOR,
                op => panic!("Unknown op {op}"),
            },
        }
    }
}

struct Input {
    inputs: HashMap<&'static str, bool>,
    gates: HashMap<&'static str, Gate>,
}

fn load_input() -> Input {
    let mut data = include_str!("../input.txt").lines();

    Input {
        inputs: HashMap::from_iter(
            data.by_ref()
                .take_while(|line| !line.is_empty())
                .map(|line| {
                    let mut parts = line.split(": ");
                    (
                        parts.next().unwrap(),
                        if parts.next().unwrap() == "1" {
                            true
                        } else {
                            false
                        },
                    )
                }),
        ),
        gates: HashMap::from_iter(data.map(|line| {
            let gate: Gate = line.into();
            (gate.output_name, gate)
        })),
    }
}

fn to_decimal<Iter: Iterator<Item = bool>>(bits: Iter) -> u64 {
    bits.enumerate().fold(0_u64, |existing, (idx, value)| {
        if value {
            existing + (1_u64 << idx)
        } else {
            existing
        }
    })
}

fn task1() {
    let input = load_input();

    let mut outputs: Vec<_> = input
        .gates
        .iter()
        .filter(|(&output_name, _)| output_name.starts_with("z"))
        .map(|(_, gate)| gate)
        .collect();

    outputs.sort_by_key(|gate| gate.output_name);

    let result = outputs
        .iter()
        .map(|gate| gate.calculate(&input.inputs, &input.gates));

    let number = to_decimal(result);

    println!("Task: result: {number}");
}

fn get_affecting_outputs(
    output: &str,
    gates: &HashMap<&'static str, Gate>,
) -> HashSet<&'static str> {
    let mut affecting_outputs = HashSet::new();
    let gate = gates.get(output).unwrap(); // we know it exists
    if !gate.input1_name.starts_with("x") && !gate.input1_name.starts_with("y") {
        affecting_outputs.insert(gate.input1_name);
        // affecting_outputs.(&mut );
        get_affecting_outputs(gate.input1_name, gates)
            .into_iter()
            .for_each(|gate| {
                affecting_outputs.insert(gate);
            });
    }
    if !gate.input2_name.starts_with("x") && !gate.input2_name.starts_with("y") {
        affecting_outputs.insert(gate.input2_name);
        get_affecting_outputs(gate.input2_name, gates)
            .into_iter()
            .for_each(|gate| {
                affecting_outputs.insert(gate);
            });
    }
    affecting_outputs
}

fn task2() {
    let input = load_input();

    let all_output_names: Vec<_> = input
        .gates
        .iter()
        .map(|(_, gate)| gate.output_name)
        .collect();

    let mut x_inputs: Vec<_> = input
        .inputs
        .iter()
        .filter(|(&inp, _)| inp.starts_with("x"))
        .collect();
    x_inputs.sort_by_key(|(&inp, _)| inp);
    let x_value = to_decimal(x_inputs.iter().map(|(_, &value)| value));
    let mut y_inputs: Vec<_> = input
        .inputs
        .iter()
        .filter(|(&inp, _)| inp.starts_with("y"))
        .collect();
    y_inputs.sort_by_key(|(&inp, _)| inp);
    let y_value = to_decimal(x_inputs.iter().map(|(_, &value)| value));

    let target_value = x_value + y_value;

    let mut outputs: Vec<_> = input
        .gates
        .iter()
        .filter(|(&output_name, _)| output_name.starts_with("z"))
        .map(|(_, gate)| gate)
        .collect();
    outputs.sort_by_key(|gate| gate.output_name);

    let result = outputs
        .iter()
        .map(|gate| gate.calculate(&input.inputs, &input.gates));

    let number = to_decimal(result);

    let wrong_outputs: Vec<_> = (0..outputs.len())
        .filter(|idx| number & (1 << idx) != target_value & (1 << idx))
        .map(|idx| format!("z{idx:#02}"))
        .collect();
    let correct_outputs: Vec<_> = (0..outputs.len())
        .filter(|idx| number & (1 << idx) == target_value & (1 << idx))
        .map(|idx| format!("z{idx:#02}"))
        .collect();

    let potential_bad_gates: Vec<_> = wrong_outputs
        .iter()
        .map(|output| get_affecting_outputs(output, &input.gates))
        .collect();
    let good_gates: Vec<_> = correct_outputs
        .iter()
        .map(|output| get_affecting_outputs(output, &input.gates))
        .collect();
    // let's find all gates, that influence wrong output bits, but _NOT_ right output bits
    let only_bad_gates: HashSet<_> = potential_bad_gates
        .iter()
        .flat_map(|block| {
            block
                .iter()
                .filter(|&gate| good_gates.iter().any(|block| block.contains(gate)))
        })
        .collect();

    let gate_values: HashMap<&str, bool> =
        HashMap::from_iter(input.gates.iter().map(|(&output_name, gate)| {
            (output_name, gate.calculate(&input.inputs, &input.gates))
        }));

    // separate all bad gates into true and false because it makes no sense to switch two gates with an identical value
    let (false_gates, true_gates): (Vec<_>, Vec<_>) = gate_values
        .iter()
        .filter(|(&name, _)| only_bad_gates.contains(&name))
        .partition(|(_, &value)| value);

    // for replacement_false_block in false_gates.iter().combinations(4) {
    //     for replacement_true_block in true_gates.iter().permutations(4) {
    //      TODO execute the swap and test -> this is too slow. See README for explanation
    //     }
    // }
}

fn main() {
    task1();
    task2();
}
