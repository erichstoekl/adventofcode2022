use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_rucksacks_from_file(filename: &str) -> Vec<(String, String)> {
	let mut tuples = Vec::new();
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	for line in reader.lines() {
		let line = line.unwrap();
		let middle = line.chars().count() / 2;
		let (first_half, second_half) = line.split_at(middle);
		let first_half_string = first_half.to_string();
		let second_half_string = second_half.to_string();
		tuples.push((first_half_string, second_half_string));
	}
	tuples
}

pub fn read_groups_from_file(filename: &str) -> Vec<Vec<String>> {
	let mut return_group: Vec<Vec<String>> = Vec::new();
	let mut group: Vec<String> = Vec::new();
	group.reserve(3);
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	for line in reader.lines() {
		let line = line.unwrap();
		group.push(line);
		if group.len() == 3 {
			return_group.push(group);
			group = Vec::new();
			group.reserve(3);
		}
	}
	return_group
}