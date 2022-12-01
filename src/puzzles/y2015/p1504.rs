pub fn puzzle() -> (i64, i64) {
	let input = "iwrupvqb";
	let mut i = 0;
	let fivezeroes = 0;
	loop {
		let attempt = format!("{}{}", input, i);
		// gotta force it to the right type
		let hash = md5::compute(attempt);
		if format!("{:x}", hash).starts_with("000000") {
			break;
		}
		i += 1;
	}
	(fivezeroes, i)
}
