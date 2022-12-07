use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_file_lines(filename: &str) -> Vec<String> {
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);

	// 1. get full file into vec<string>
	let mut lines = Vec::new();
	for line in reader.lines() {
		let line = line.unwrap();
		lines.push(line);
	}
	lines
}

pub fn read_stacks_from_vec(lines: &Vec<String>) -> Vec<Vec<char>> {
	let stack_count = (lines[0].len() + 1) / 4;

	// 3. Convert stack text data to Vec<Vec<char>> with capacity stack_count
	let mut stacks: Vec<Vec<char>> = Vec::with_capacity(stack_count);
	for _i in 0..stack_count {
		stacks.push(Vec::new());
	}
	let mut i = 0;
	while !lines[i].is_empty() {
		let line_chars: Vec<char> = lines[i].chars().collect();
		for j in 0..stack_count {
			let k = (j*4) + 1;
			if line_chars[k] != ' ' {
				stacks[j].insert(0, line_chars[k]);
			}
		}
		i += 1;
	}
	stacks
}

pub fn read_instructions_from_vec(lines: Vec<String>) -> Vec<(usize, usize, usize)> {
	let mut i = 0;
	let line_count = lines.len();
	let mut instructions = Vec::new();
	while !lines[i].is_empty() {
		i += 1;
	}
	i += 1;
	while i < line_count {
		let (a, b, c) = scan_fmt!(&lines[i], "move {} from {} to {}", usize, usize, usize).unwrap();
		instructions.push((a, b-1, c-1));
		i += 1;
	}
	instructions
}