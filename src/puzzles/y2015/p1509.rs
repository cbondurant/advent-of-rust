use crate::get_data::get_data;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

struct Salesman {
	routes: HashMap<i32, HashMap<i32, i64>>,
	locations: HashMap<String, i32>,
}

impl Salesman {
	fn new() -> Salesman {
		Salesman {
			routes: HashMap::new(),
			locations: HashMap::new(),
		}
	}

	fn add_route(&mut self, start: &str, end: &str, distance: i64) {
		let loc_len = self.locations.len();
		let start = *self
			.locations
			.entry(start.to_string())
			.or_insert(loc_len as i32);
		let loc_len = self.locations.len();
		let end = *self
			.locations
			.entry(end.to_string())
			.or_insert(loc_len as i32);
		self.routes
			.entry(start)
			.or_insert(HashMap::new())
			.insert(end, distance);
		self.routes
			.entry(end)
			.or_insert(HashMap::new())
			.insert(start, distance);
	}

	fn find_shortest_remaining(&self, cur_path: &mut Vec<i32>) -> i64 {
		let mut shortest_dist = i64::MAX;
		let mut shortest_dest = -1;
		let used_locations: HashSet<i32> = HashSet::from_iter(cur_path.iter().cloned());
		for destination in self.routes.keys().filter(|x| !used_locations.contains(x)) {
			cur_path.push(*destination);
			let dist = self.find_shortest_remaining(cur_path);
			if shortest_dist > dist {
				shortest_dist = dist;
				shortest_dest = *destination;
			}
			cur_path.pop();
		}

		if shortest_dist == i64::MAX {
			return 0;
		}
		if cur_path.len() != 0 {
			shortest_dist
				+ self
					.routes
					.get(&shortest_dest)
					.unwrap()
					.get(&cur_path[cur_path.len() - 1])
					.unwrap()
		} else {
			shortest_dist
		}
	}
}

pub fn puzzle() -> (i64, i64) {
	let input = get_data(2015, 9);
	let mut salesman = Salesman::new();
	for line in input.split("\n") {
		let tokens: Vec<&str> = line.split(' ').collect();
		if tokens.len() != 5 {
			continue;
		}
		let start = tokens[0];
		let end = tokens[2];
		let distance = tokens[4].parse().unwrap();
		salesman.add_route(start, end, distance);
	}

	(salesman.find_shortest_remaining(&mut Vec::new()), 0)
}
