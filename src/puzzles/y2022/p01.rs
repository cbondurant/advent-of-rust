use crate::get_data::get_data;
use std::collections::BTreeSet;
use std::str::FromStr;

pub fn puzzle() -> (i64, i64) {
	let input = get_data(2022, 1);

	let mut elves = BTreeSet::new();

	for elf in input.split("\n\n") {
		let mut total = 0;
		for food in elf.split("\n") {
			if let Ok(cal) = i64::from_str(food) {
				total += cal;
			}
		}
		elves.insert(total);
	}

	println!("{:?}", elves);

	(
		*elves.iter().rev().next().unwrap(),
		elves.iter().rev().take(3).sum(),
	)
}
