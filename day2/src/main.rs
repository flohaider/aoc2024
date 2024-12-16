use std::fs;

fn main() {
	let file_path = "input.txt";
	println!("In file {file_path}");

	let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");

	let mut number_of_safe_reports = 0;
	for line in contents.lines() {
		// first parse report
		let row_nums: Vec<&str> = line.split(" ").collect();
		let mut report: Vec<u32> = Vec::new();
		for num in row_nums.iter() {
			let level: u32 = num.parse().expect("Error parsing row number.");
			report.push(level);
		}
		// then determine if they are increasing or decreasing
		let mut report_safe = true;
		let mut report_increasing = false;
		let mut report_decreasing = false;
		for i in 0 .. report.len() - 1 {
			let level = report.get(i).expect("Level not found");
			let level_ahead = report.get(i + 1).expect("Level ahead not found");
			let level_delta = i64::from(*level) - i64::from(*level_ahead);
			// we need strict incr or decr, so abort at 0
			if level_delta == 0 {
				report_safe = false;
				break;
			}
			// determine incr or decr
			if i == 0 {
				// determine it at the first level
				report_increasing = level_delta < 0;
				report_decreasing = level_delta > 0;
			} else {
				// and check it for all subsequent ones
				if (level_delta < 0 && report_decreasing) || (level_delta > 0 && report_increasing) {
					// direction changed, so abort
					report_safe = false;
					break;
				}
			}
			// finally make sure abs delta is between 1 and 3
			let level_delta_abs = level_delta.abs();
			if level_delta_abs < 1 || level_delta_abs > 3 {
				report_safe = false;
				break;
			}
		}
		if report_safe {
			number_of_safe_reports += 1;
		}
	}
	println!("Number of safe reports: {number_of_safe_reports}");
}
