use std::collections::HashMap;

pub use crate::get_data::get_data;

enum ExprType<'a> {
	Not(&'a str),
	And(&'a str, &'a str),
	Or(&'a str, &'a str),
	LShift(&'a str, u16),
	RShift(&'a str, u16),
	Const(u16),
	Link(&'a str),
}

fn parse_expr(expr: &str) -> ExprType {
	if expr.starts_with("NOT ") {
		return ExprType::Not(&expr[4..]);
	}
	if let Ok(i) = expr.parse::<u16>() {
		return ExprType::Const(i);
	}
	let symbols = expr.split(' ').collect::<Vec<&str>>();
	if symbols.len() == 1 {
		return ExprType::Link(symbols[0]);
	}
	match symbols[1] {
		"LSHIFT" => ExprType::LShift(symbols[0], symbols[2].parse().unwrap()),
		"RSHIFT" => ExprType::RShift(symbols[0], symbols[2].parse().unwrap()),
		"AND" => ExprType::And(symbols[0], symbols[2]),
		"OR" => ExprType::Or(symbols[0], symbols[2]),
		_ => ExprType::Const(0),
	}
}

fn eval_wire<'a>(
	w: &'a HashMap<&str, ExprType>,
	cur_wire: &'a str,
	c: &mut HashMap<&'a str, u16>,
) -> u16 {
	if let Ok(val) = cur_wire.parse() {
		return val;
	}

	if let Some(val) = c.get(cur_wire) {
		return *val;
	}

	let val = match w.get(cur_wire).unwrap() {
		ExprType::Const(i) => *i,
		ExprType::And(s1, s2) => eval_wire(w, s1, c) & eval_wire(w, s2, c),
		ExprType::Or(s1, s2) => eval_wire(w, s1, c) | eval_wire(w, s2, c),
		ExprType::LShift(s, i) => eval_wire(w, s, c) << i,
		ExprType::RShift(s, i) => eval_wire(w, s, c) >> i,
		ExprType::Not(s) => !eval_wire(w, s, c),
		ExprType::Link(s) => eval_wire(w, s, c),
	};
	c.insert(cur_wire, val);
	return val;
}

pub fn puzzle() -> (u16, u16) {
	let input = get_data(2015, 7);
	let mut wires: HashMap<&str, ExprType> = HashMap::new();
	let mut cache = HashMap::new();
	for line in input.split("\n") {
		if line != "" {
			if let [expr, target] = &line.split(" -> ").collect::<Vec<&str>>()[..2] {
				wires.insert(target, parse_expr(expr));
			}
		}
	}
	let part1 = eval_wire(&wires, "a", &mut cache);
	let mut cache = HashMap::new();
	wires.insert("b", ExprType::Const(part1));
	let part2 = eval_wire(&wires, "a", &mut cache);
	(part1, part2)
}
