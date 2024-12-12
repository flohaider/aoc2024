use std::fs;
use std::collections::HashMap;

fn main() {
	let file_path = "input.txt";
	println!("In file {file_path}");

	let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");

	let mut left_nums: Vec<u32> = Vec::new();
	let mut right_nums: HashMap<u32, u32> = HashMap::new();

    //scores.insert(String::from("Blue"), 10);
	
	for line in contents.lines() {
		let row_nums: Vec<&str> = line.split("   ").collect();
		if row_nums.len() == 2 {
			let left_num: u32 = row_nums[0].parse().expect("Error parsing left number.");
			left_nums.push(left_num);
			let right_num: u32 = row_nums[1].parse().expect("Error parsing right number.");
			let count = right_nums.entry(right_num).or_insert(0);
        	*count += 1;
		}
	}

	let mut similarity_score: u32 = 0;
	for left_num in left_nums.iter() {
		let count_right_num = right_nums.get(left_num).copied().unwrap_or(0);
		similarity_score += left_num * count_right_num;
	}
	println!("Similarity score: {similarity_score}");
}
