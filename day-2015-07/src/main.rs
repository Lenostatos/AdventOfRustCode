use std::collections::HashMap;
use util::input::lines_of;

fn main() {
    let input_lines = lines_of("input.txt");

    let mut curcuit_tree = HashMap::<WireId, Wire>::new();

    for line in input_lines {
        let in_out = line.split_once(" -> ").unwrap();
        let out_id: WireId = in_out.1.to_string();

        if let Some(_) = in_out.0.find("AND") {
            let x_and_y = in_out.0.split_once(" AND ").unwrap();

            curcuit_tree.insert(
                out_id.clone(),
                Wire {
                    input: WireInput::Gate(Gate::And(
                        parse_gate_input(x_and_y.0),
                        parse_gate_input(x_and_y.1),
                    )),
                },
            );
        } else if let Some(_) = in_out.0.find("OR") {
            let x_or_y = in_out.0.split_once(" OR ").unwrap();

            curcuit_tree.insert(
                out_id.clone(),
                Wire {
                    input: WireInput::Gate(Gate::Or(
                        parse_gate_input(x_or_y.0),
                        parse_gate_input(x_or_y.1),
                    )),
                },
            );
        } else if let Some(_) = in_out.0.find("NOT") {
            let wire_id = in_out.0.strip_prefix("NOT ").unwrap();

            curcuit_tree.insert(
                out_id.clone(),
                Wire {
                    input: WireInput::Gate(Gate::Complement(parse_gate_input(wire_id))),
                },
            );
        } else if let Some(_) = in_out.0.find("LSHIFT") {
            let x_lshift_n = in_out.0.split_once(" LSHIFT ").unwrap();

            curcuit_tree.insert(
                out_id.clone(),
                Wire {
                    input: WireInput::Gate(Gate::LeftShift(
                        parse_gate_input(x_lshift_n.0),
                        x_lshift_n.1.parse::<u16>().unwrap(),
                    )),
                },
            );
        } else if let Some(_) = in_out.0.find("RSHIFT") {
            let x_rshift_n = in_out.0.split_once(" RSHIFT ").unwrap();

            curcuit_tree.insert(
                out_id.clone(),
                Wire {
                    input: WireInput::Gate(Gate::RightShift(
                        parse_gate_input(x_rshift_n.0),
                        x_rshift_n.1.parse::<u16>().unwrap(),
                    )),
                },
            );
        } else {
            let value_or_wire_id = in_out.0.parse::<u16>();

            match value_or_wire_id {
                Ok(value) => {
                    curcuit_tree.insert(
                        out_id.clone(),
                        Wire {
                            input: WireInput::Value(value),
                        },
                    );
                }
                Err(_) => {
                    curcuit_tree.insert(
                        out_id.clone(),
                        Wire {
                            input: WireInput::Wire(in_out.0.to_string()),
                        },
                    );
                }
            }
        }
    }

    let wire_a_value = get_value_for_wire(&"a".to_string(), &curcuit_tree).unwrap();

    println!("{wire_a_value}");

    curcuit_tree.insert(
        "b".to_string(),
        Wire {
            input: WireInput::Value(wire_a_value),
        },
    );

    let wire_b_value = get_value_for_wire(&"a".to_string(), &curcuit_tree).unwrap();

    println!("{wire_b_value}");
}

use String as WireId;

struct Wire {
    input: WireInput,
}

enum WireInput {
    Value(u16),
    Wire(WireId),
    Gate(Gate),
}

enum Gate {
    And(GateInput, GateInput),
    Or(GateInput, GateInput),
    LeftShift(GateInput, u16),
    RightShift(GateInput, u16),
    Complement(GateInput),
}

enum GateInput {
    Wire(WireId),
    Value(u16),
}

fn parse_gate_input(input: &str) -> GateInput {
    match input.parse::<u16>() {
        Ok(value) => GateInput::Value(value),
        Err(_) => GateInput::Wire(input.to_string()),
    }
}

enum Direction {
    Backward,
    Forward,
}

fn get_value_for_wire(wire_id: &WireId, curcuit_tree: &HashMap<WireId, Wire>) -> Option<u16> {
    let mut wire_values = HashMap::<&WireId, u16>::new();
    let mut wire_trail = vec![wire_id];
    let mut direction = Direction::Backward;

    loop {
        if wire_trail.is_empty() {
            break;
        }
        let current_wire_id = wire_trail.last().unwrap();
        match direction {
            Direction::Backward => {
                let wire_input = &curcuit_tree.get(*current_wire_id).unwrap().input;
                match wire_input {
                    WireInput::Value(value) => {
                        wire_values.insert(&current_wire_id, *value);
                        direction = Direction::Forward;
                        wire_trail.pop();
                    }
                    WireInput::Wire(id) => {
                        wire_trail.push(id);
                    }
                    WireInput::Gate(gate) => match gate {
                        Gate::And(gate_input_1, gate_input_2) => match gate_input_1 {
                            GateInput::Wire(wire_id) => {
                                wire_trail.push(wire_id);
                            }
                            GateInput::Value(value_1) => match gate_input_2 {
                                GateInput::Wire(wire_id) => {
                                    wire_trail.push(wire_id);
                                }
                                GateInput::Value(value_2) => {
                                    wire_values.insert(&current_wire_id, *value_1 & *value_2);
                                    direction = Direction::Forward;
                                    wire_trail.pop();
                                }
                            },
                        },
                        Gate::Or(gate_input_1, gate_input_2) => match gate_input_1 {
                            GateInput::Wire(wire_id) => {
                                wire_trail.push(wire_id);
                            }
                            GateInput::Value(value_1) => match gate_input_2 {
                                GateInput::Wire(wire_id) => {
                                    wire_trail.push(wire_id);
                                }
                                GateInput::Value(value_2) => {
                                    wire_values.insert(&current_wire_id, *value_1 | *value_2);
                                    direction = Direction::Forward;
                                    wire_trail.pop();
                                }
                            },
                        },
                        Gate::LeftShift(gate_input, gate_value) => match gate_input {
                            GateInput::Wire(wire_id) => {
                                wire_trail.push(wire_id);
                            }
                            GateInput::Value(value) => {
                                wire_values.insert(&current_wire_id, *value << *gate_value);
                                direction = Direction::Forward;
                                wire_trail.pop();
                            }
                        },
                        Gate::RightShift(gate_input, gate_value) => match gate_input {
                            GateInput::Wire(wire_id) => {
                                wire_trail.push(wire_id);
                            }
                            GateInput::Value(value) => {
                                wire_values.insert(&current_wire_id, *value >> *gate_value);
                                direction = Direction::Forward;
                                wire_trail.pop();
                            }
                        },
                        Gate::Complement(gate_input) => match gate_input {
                            GateInput::Wire(wire_id) => {
                                wire_trail.push(wire_id);
                            }
                            GateInput::Value(value) => {
                                wire_values.insert(&current_wire_id, !*value);
                                direction = Direction::Forward;
                                wire_trail.pop();
                            }
                        },
                    },
                }
            }
            Direction::Forward => {
                let current_wire_input = &curcuit_tree.get(*current_wire_id).unwrap().input;
                match current_wire_input {
                    WireInput::Value(_) => {
                        panic!("Should not be possible to reach a value while going forward.");
                    }
                    WireInput::Wire(id) => {
                        let previous_wire_value = wire_values.get(id).unwrap();
                        wire_values.insert(&current_wire_id, *previous_wire_value);
                        wire_trail.pop();
                    }
                    WireInput::Gate(gate) => match gate {
                        // Only match gate input 2 in forward direction, since gate input 1 has to have a value at that point.
                        Gate::And(gate_input_1, gate_input_2) => match gate_input_2 {
                            GateInput::Value(value_2) => {
                                let value_1 = get_gate_input_value(gate_input_1, &wire_values);

                                wire_values.insert(&current_wire_id, value_1 & *value_2);
                                wire_trail.pop();
                            }
                            GateInput::Wire(wire_id_2) => {
                                let value_2 = wire_values.get(wire_id_2);

                                if value_2.is_none() {
                                    wire_trail.push(wire_id_2);
                                    direction = Direction::Backward;
                                } else {
                                    let value_1 = get_gate_input_value(gate_input_1, &wire_values);

                                    wire_values
                                        .insert(&current_wire_id, value_1 & value_2.unwrap());
                                    wire_trail.pop();
                                }
                            }
                        },
                        Gate::Or(gate_input_1, gate_input_2) => match gate_input_2 {
                            GateInput::Value(value_2) => {
                                let value_1 = get_gate_input_value(gate_input_1, &wire_values);

                                wire_values.insert(&current_wire_id, value_1 | *value_2);
                                wire_trail.pop();
                            }
                            GateInput::Wire(wire_id_2) => {
                                let value_2 = wire_values.get(wire_id_2);

                                if value_2.is_none() {
                                    wire_trail.push(wire_id_2);
                                    direction = Direction::Backward;
                                } else {
                                    let value_1 = get_gate_input_value(gate_input_1, &wire_values);

                                    wire_values
                                        .insert(&current_wire_id, value_1 | value_2.unwrap());
                                    wire_trail.pop();
                                }
                            }
                        },
                        Gate::LeftShift(gate_input, shift_value) => {
                            let gate_input_value = match gate_input {
                                GateInput::Wire(wire_id) => wire_values.get(wire_id).unwrap(),
                                GateInput::Value(_) => panic!("This path should not be possible."),
                            };

                            wire_values.insert(&current_wire_id, *gate_input_value << *shift_value);
                            wire_trail.pop();
                        }
                        Gate::RightShift(gate_input, shift_value) => {
                            let gate_input_value = match gate_input {
                                GateInput::Wire(wire_id) => wire_values.get(wire_id).unwrap(),
                                GateInput::Value(_) => panic!("This path should not be possible."),
                            };

                            wire_values.insert(&current_wire_id, *gate_input_value >> *shift_value);
                            wire_trail.pop();
                        }
                        Gate::Complement(gate_input) => {
                            let gate_input_value = match gate_input {
                                GateInput::Wire(wire_id) => wire_values.get(wire_id).unwrap(),
                                GateInput::Value(_) => panic!("This path should not be possible."),
                            };

                            wire_values.insert(&current_wire_id, !*gate_input_value);
                            wire_trail.pop();
                        }
                    },
                }
            }
        }
    }

    wire_values.get(wire_id).cloned()
}

fn get_gate_input_value(gate_input: &GateInput, wire_values: &HashMap<&String, u16>) -> u16 {
    match gate_input {
        GateInput::Value(value) => *value,
        GateInput::Wire(wire_id_1) => *wire_values.get(wire_id_1).unwrap(),
    }
}

#[test]
fn reference_equality() {
    let foo = String::from("foo");
    let foo2 = String::from("foo");
    assert_eq!(&foo, &foo2);
}

#[test]
fn example() {
    let mut curcuit_tree = HashMap::new();

    curcuit_tree.insert(
        "z".to_string(),
        Wire {
            input: WireInput::Value(123),
        },
    );
    curcuit_tree.insert(
        "x".to_string(),
        Wire {
            input: WireInput::Wire("z".to_string()),
        },
    );
    curcuit_tree.insert(
        "y".to_string(),
        Wire {
            input: WireInput::Value(456),
        },
    );
    curcuit_tree.insert(
        "d".to_string(),
        Wire {
            input: WireInput::Gate(Gate::And(
                GateInput::Wire("x".to_string()),
                GateInput::Value(456),
            )),
        },
    );
    curcuit_tree.insert(
        "e".to_string(),
        Wire {
            input: WireInput::Gate(Gate::Or(
                GateInput::Wire("x".to_string()),
                GateInput::Wire("y".to_string()),
            )),
        },
    );
    curcuit_tree.insert(
        "f".to_string(),
        Wire {
            input: WireInput::Gate(Gate::LeftShift(GateInput::Wire("x".to_string()), 2)),
        },
    );
    curcuit_tree.insert(
        "g".to_string(),
        Wire {
            input: WireInput::Gate(Gate::RightShift(GateInput::Wire("y".to_string()), 2)),
        },
    );
    curcuit_tree.insert(
        "h".to_string(),
        Wire {
            input: WireInput::Gate(Gate::Complement(GateInput::Wire("x".to_string()))),
        },
    );
    curcuit_tree.insert(
        "i".to_string(),
        Wire {
            input: WireInput::Gate(Gate::Complement(GateInput::Wire("y".to_string()))),
        },
    );

    assert_eq!(
        get_value_for_wire(&"x".to_string(), &curcuit_tree).unwrap(),
        123
    );
    assert_eq!(
        get_value_for_wire(&"y".to_string(), &curcuit_tree).unwrap(),
        456
    );
    assert_eq!(
        get_value_for_wire(&"d".to_string(), &curcuit_tree).unwrap(),
        72
    );
    assert_eq!(
        get_value_for_wire(&"e".to_string(), &curcuit_tree).unwrap(),
        507
    );
    assert_eq!(
        get_value_for_wire(&"f".to_string(), &curcuit_tree).unwrap(),
        492
    );
    assert_eq!(
        get_value_for_wire(&"g".to_string(), &curcuit_tree).unwrap(),
        114
    );
    assert_eq!(
        get_value_for_wire(&"h".to_string(), &curcuit_tree).unwrap(),
        65412
    );
    assert_eq!(
        get_value_for_wire(&"i".to_string(), &curcuit_tree).unwrap(),
        65079
    );
}
