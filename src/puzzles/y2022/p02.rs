use crate::get_data::get_data;
use std::collections::HashMap;

pub fn puzzle() -> (i64, i64) {
	let input = get_data(2022, 2);

	let mut results_1 = HashMap::new();
	// Win
	results_1.insert("A Y", 6 + 2);
	results_1.insert("B Z", 6 + 3);
	results_1.insert("C X", 6 + 1);
	results_1.insert("A X", 3 + 1);
	results_1.insert("B Y", 3 + 2);
	results_1.insert("C Z", 3 + 3);
	results_1.insert("A Z", 0 + 3);
	results_1.insert("B X", 0 + 1);
	results_1.insert("C Y", 0 + 2);

	let mut results_2 = HashMap::new();
	// Win
	results_2.insert("A X", 0 + 3);
	results_2.insert("B X", 0 + 1);
	results_2.insert("C X", 0 + 2);
	results_2.insert("A Y", 3 + 1);
	results_2.insert("B Y", 3 + 2);
	results_2.insert("C Y", 3 + 3);
	results_2.insert("A Z", 6 + 2);
	results_2.insert("B Z", 6 + 3);
	results_2.insert("C Z", 6 + 1);

	let mut total1 = 0;
	let mut total2 = 0;
	for round in input.split("\n") {
		total1 += results_1[round];
		total2 += results_2[round];
	}
	(total1, total2)
}
