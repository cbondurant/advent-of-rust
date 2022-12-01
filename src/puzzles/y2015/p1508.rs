use crate::get_data::get_data;

pub fn puzzle() -> (i64, i64) {
	let input = get_data(2015, 8);
	//let input = "\"\\x27\"\n\"aaa\\\"aaa\"\n\"\"\n\"abc\"";
	let mut total_chars = 0;
	let mut mem_chars = 0;
	let mut escape_chars = 0;
	for line in input.split("\n") {
		if line == "" {
			break;
		}
		total_chars += line.len();
		escape_chars += line.len() + 2; // open and close "
		let mut chars = line.chars();
		while let Some(c) = chars.next() {
			if c == '"' {
				escape_chars += 1; // '"' to '\"'
				continue;
			}
			mem_chars += 1;

			if c == '\\' {
				escape_chars += 1; // '\' to '\\'
				let c = chars.next().unwrap();
				if c == 'x' {
					chars.nth(1); // consume 2 values
				}

				if c == '\\' {
					escape_chars += 1;
				}

				if c == '"' {
					escape_chars += 1;
				}
				continue;
			}
		}
	}
	(
		(total_chars - mem_chars) as i64,
		(escape_chars - total_chars) as i64,
	)
}
