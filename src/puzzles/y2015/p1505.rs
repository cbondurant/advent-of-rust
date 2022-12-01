use crate::get_data::get_data;

pub fn puzzle() -> (i64, i64) {
	let input = get_data(2015, 5);
	let mut count = 0;
	let mut count2 = 0;
	for line in input.split('\n') {
		if line == "" {
			break;
		} // Deals with final newline causing problems
		let check1 = line
			.split(&['a', 'e', 'i', 'o', 'u'][..])
			.collect::<Vec<&str>>()
			.len() > 3;

		let mut check2 = false;
		let mut prev = ' ';
		for c in line.chars() {
			if c == prev {
				check2 = true;
				break;
			}
			prev = c
		}

		let check3 =
			line.contains("ab") | line.contains("cd") | line.contains("pq") | line.contains("xy");

		let mut check4 = false;
		for i in 0..line.len() - 2 {
			if line[i..i + 1] == line[i + 2..i + 3] {
				check4 = true;
				break;
			}
		}

		let mut check5 = false;
		for i in 0..line.len() - 3 {
			for j in i + 2..line.len() - 1 {
				if &line[i..i + 2] == &line[j..j + 2] {
					check5 = true;
					break;
				}
			}
		}

		if check1 & check2 & !check3 {
			count += 1;
		}

		if check4 & check5 {
			count2 += 1;
		}
	}
	(count, count2)
}
