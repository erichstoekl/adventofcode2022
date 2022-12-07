use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_work_pairs_from_file(filename: &str) -> Vec<(u32, u32, u32, u32)> {
	let mut tuples = Vec::new();
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	for line in reader.lines() {
		let line: String = line.unwrap();
		let parts: Vec<&str> = line.split(",").collect();
		let first_part: String = parts[0].parse().unwrap();
		let second_part: String = parts[1].parse().unwrap();
		let ranges_elf_1: Vec<&str> = first_part.split("-").collect();
		let ranges_elf_2: Vec<&str> = second_part.split("-").collect();
		let tuple_1: u32 = ranges_elf_1[0].parse().unwrap();
		let tuple_2: u32 = ranges_elf_1[1].parse().unwrap();
		let tuple_3: u32 = ranges_elf_2[0].parse().unwrap();
		let tuple_4: u32 = ranges_elf_2[1].parse().unwrap();
		tuples.push((tuple_1, tuple_2, tuple_3, tuple_4));
	}
	tuples
}