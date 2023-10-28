//! --- Day 7: Some Assembly Required ---
//! This year, Santa brought little Bobby Tables a set of wires and bitwise logic gates!
//! Unfortunately, little Bobby is a little under the recommended age range, and he needs
//! help assembling the circuit.
//! 
//! Each wire has an identifier (some lowercase letters) and can carry a 16-bit signal
//! (a number from `0` to `65535`). A signal is provided to each wire by a gate, another
//! wire, or some specific value. Each wire can only get a signal from one source, but can
//! provide its signal to multiple destinations. A gate provides no signal until all of
//! its inputs have a signal.
//! 
//! The included instructions booklet describes how to connect the parts together:
//! `x AND y -> z` means to connect wires `x` and `y` to an `AND` gate, and then connect
//! its output to wire `z`.
//! 
//! For example:
//! 
//! - `123 -> x` means that the signal `123` is provided to wire `x`.
//! - `x AND y -> z` means that the bitwise `AND` of wire `x` and wire `y` is provided to
//!   wire `z`.
//! - `p LSHIFT 2 -> q` means that the value from wire `p` is left-shifted by `2` and then
//!   provided to wire `q`.
//! - `NOT e -> f` means that the bitwise complement of the value from wire `e` is
//!   provided to wire `f`.
//!
//! Other possible gates include `OR` (bitwise OR) and `RSHIFT` (right-shift). If, for
//! some reason, you'd like to emulate the circuit instead, almost all programming
//! languages (for example, C, JavaScript, or Python) provide operators for these gates.
//! 
//! For example, here is a simple circuit:
//! 
//! ```
//! 123 -> x
//! 456 -> y
//! x AND y -> d
//! x OR y -> e
//! x LSHIFT 2 -> f
//! y RSHIFT 2 -> g
//! NOT x -> h
//! NOT y -> i
//! ```
//! After it is run, these are the signals on the wires:
//! 
//! ```
//! d: 72
//! e: 507
//! f: 492
//! g: 114
//! h: 65412
//! i: 65079
//! x: 123
//! y: 456
//! ```
//! In little Bobby's kit's instructions booklet (provided as your puzzle input), what
//! signal is ultimately provided to wire `a`?
//!
//! --- Part Two ---
//! Now, take the signal you got on wire `a`, override wire `b` to that signal, and reset
//! the other wires (including wire `a`). What new signal is ultimately provided to wire
//! `a`?

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Gate {
    And,
    Or,
    Not,
    Lshift,
    Rshift
}


#[derive(Debug, PartialEq)]
enum Input {
    Op(Gate),
    Signal(u16),
    Wire(String),
}


#[derive(Debug)]
struct Instruction {
    inputs: Vec<Input>,
    wire: String,
}


impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, String> {
        let parts: Vec<String> = input.split(" -> ")
            .map(|x| x.trim().to_string())
            .collect();
        if parts.len() != 2 { return Err(format!("malformed input: {input}")) }
        let (lhs, rhs) = (&parts[0], &parts[1]);
        let gates = vec!["AND", "OR", "NOT", "LSHIFT", "RSHIFT"];
        let inputs = lhs
            .split_whitespace()
            .map(|x| {
                if gates.contains(&x) {
                    match x {
                        "AND" => Ok(Input::Op(Gate::And)),
                        "OR" => Ok(Input::Op(Gate::Or)),
                        "NOT" => Ok(Input::Op(Gate::Not)),
                        "LSHIFT" => Ok(Input::Op(Gate::Lshift)),
                        "RSHIFT" => Ok(Input::Op(Gate::Rshift)),
                        _ => Err(String::from("invalid op")),
                    }
                } else {
                    let value = x.parse::<u16>();
                    match value {
                        Ok(signal) => Ok(Input::Signal(signal)),
                        _ => Ok(Input::Wire(x.to_string())),
                    }
                }
            })
            .collect::<Result<Vec<Input>, _>>()?;
        Ok(Self { inputs, wire: rhs.to_owned() })
    }
}


fn try_parse_instructions(input: String) -> Result<Vec<Instruction>, String> {
    let lines = input.split("\n");
    let instructions = lines
        .filter_map(|l| {
            if l.is_empty() { return None }
            Some(l.try_into())
        })
        .collect::<Result<Vec<Instruction>, _>>()?;
    Ok(instructions)
}


fn signal_from_processed_inputs(inputs: &[Input]) -> u16 {
    match inputs[0] {
        Input::Signal(x) => if inputs.len() == 1 {
            return x
        }
        Input::Op(Gate::Not) => match inputs[1] {
            Input::Signal(x) => return !x,
            _ => unreachable!("not gate must be followed by a signal")
        },
        _ => {}
    }
    // all other cases should contain pattern `signal gate signal`
    // even though a signal can be an amount by which to shift
    let a = match inputs[0] {
        Input::Signal(a) => a,
        _ => unreachable!()
    };
    let b = match inputs[2] {
        Input::Signal(b) => b,
        _ => unreachable!()
    };
    match inputs[1] {
        Input::Op(g) => match g {
            Gate::And => a & b,
            Gate::Or => a | b,
            Gate::Lshift => a << b,
            Gate::Rshift => a >> b,
            _ => unreachable!("NOT already covered")
        },
        _ => unreachable!()
    }
}


fn doit(
    wire: &str,
    instructions: &HashMap<String, Vec<Input>>,
    signals: &mut HashMap<String, u16>
)
    -> Option<u16>
{
    if signals.contains_key(wire) {
        return signals.get(wire).cloned()
    }
    //  123->x->NOT->h
    //        \
    //        and->d
    //        /
    //  456->y->NOT->i
    let mut new_inputs: Vec<Input> = vec![];
    for input in instructions.get(wire)? {
        let new = match input {
            Input::Signal(s) => Input::Signal(*s),
            Input::Wire(w) => {
                let sig = doit(&w, instructions, signals)?;
                signals.insert(w.to_owned(), sig);
                Input::Signal(sig)
            },
            Input::Op(g) => Input::Op(*g),
        };
        new_inputs.push(new);
    }
    Some(signal_from_processed_inputs(&new_inputs))
}


fn part_1(input: String) -> Result<HashMap<String, u16>, String> {
    let mut signals = HashMap::<String, u16>::new();
    let instructions: HashMap<String, Vec<Input>> = try_parse_instructions(input)?
        .into_iter()
        .map(|ins| (ins.wire, ins.inputs))
        .collect();
    for (wire, _) in &instructions {
        let sig = doit(&wire, &instructions, &mut signals).unwrap();
        signals.insert(wire.clone(), sig);
    }
    Ok(signals)
}


fn part_2(input: String) -> Result<HashMap<String, u16>, String> {
    let mut signals = part_1(input.clone())?;
    let a = signals.get("a").unwrap();
    let mut instructions: HashMap<String, Vec<Input>> = try_parse_instructions(input)?
        .into_iter()
        .map(|ins| (ins.wire, ins.inputs))
        .collect();
    *instructions.get_mut("b").unwrap() = vec![Input::Signal(*a)];
    signals.clear();
    for (wire, _) in &instructions {
        let sig = doit(&wire, &instructions, &mut signals).unwrap();
        signals.insert(wire.to_owned(), sig);
    }
    Ok(signals)
}


#[cfg(test)]
mod tests {
    use std::{path::Path, collections::HashMap};

    fn load_input(fname: &str) -> String {
        // TODO: add error handling and return Result
        let path = Path::new(file!()).parent().unwrap().join(fname);
        std::fs::read_to_string(path).unwrap()
    }

    #[test]
    fn part_1() {
        let input = load_input("input.txt");
        let output = super::part_1(input).unwrap();
        let expected = 3176;
        assert_eq!(output.get("a").unwrap(), &expected);
    }

    #[test]
    fn part_1_sm() {
        let input = load_input("input_sm.txt");
        let output = super::part_1(input).unwrap();
        let expected = HashMap::<&str, u16>::from_iter(
            vec![
                ("d", 72),
                ("e", 507),
                ("f", 492),
                ("g", 114),
                ("h", 65412),
                ("i", 65079),
                ("x", 123),
                ("y", 456),
            ]
        );
        assert_eq!(output.get("d").unwrap(), expected.get("d").unwrap());
    }

    #[test]
    fn part_2() {
        let input = load_input("input.txt");
        let output = super::part_2(input).unwrap();
        let expected = 14710;
        assert_eq!(output.get("a").unwrap(), &expected);
    }
}
