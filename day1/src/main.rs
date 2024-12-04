use std::fs;

fn main() {
	let file_path = "input.txt";
	println!("In file {file_path}");

	let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");

	let mut left_nums: Vec<u32> = Vec::new();
	let mut right_nums: Vec<u32> = Vec::new();
	
	for line in contents.lines() {
		let row_nums: Vec<&str> = line.split("   ").collect();
		if row_nums.len() == 2 {
			let left_num: u32 = row_nums[0].parse().expect("Error parsing left number.");
			left_nums.push(left_num);
			let right_num: u32 = row_nums[1].parse().expect("Error parsing right number.");
			right_nums.push(right_num);
		}
	}

	left_nums.sort();
	right_nums.sort();

	let mut total_distance: u32 = 0;
	for (i, left_num) in left_nums.iter().enumerate() {
		let right_num = right_nums[i];
		let distance = left_num.abs_diff(right_num);
		total_distance += distance;
		println!("{left_num}\t{right_num} -> {distance}");
	}
	println!("Total distance: {total_distance}");
}
