

use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Forest {
	trees: Vec<Vec<usize>>,
	width: usize,
	height: usize,
}

impl Forest {
	pub fn new(filename: &str) -> Forest {
		let t = Forest::get_matrix(filename);
		let w = t.len();
		let h = t[0].len();
		Forest {
			trees: t,
			width: w,
			height: h,
		}
	}

	pub fn count_visible_trees(&self) -> usize {
		let mut total = 0;
		for i in 0..self.height {
			for j in 0..self.width {
				if self.is_visible(i, j) {
					total += 1;
				}
			}
		}
		total
	}
	
	pub fn get_optimal_scenic_score(&self) -> u32 {
		let mut optimal_scenic: u32 = 0;
		for i in 1..(self.height-1) {
			for j in 1..(self.width-1) {
				let scenic = self.get_scenic_score(i, j);
				if scenic > optimal_scenic {
					optimal_scenic = scenic;
				}
			}
		}
		optimal_scenic
	}
	
	fn is_visible(&self, i: usize, j: usize) -> bool {
		if self.is_border(i, j) {
			return true;
		}
		if self.visible_interior(i, j) {
			return true;
		}

		false
	}

	fn is_border(&self, i: usize, j: usize) -> bool {
		if i == 0 || j == 0 || i == self.height - 1 || j == self.width - 1 {
			return true;
		}
		false
	}
	
	fn visible_interior(&self, i: usize, j: usize) -> bool {
		let cur_tree = self.trees[i][j];
		let mut invisible_count = 0;
		// Is everything left of the current tree shorter than the current tree?
		for x in 0..j {
			if self.trees[i][x] >= cur_tree {
				invisible_count += 1;
				break;
			}
		}
		// right?
		for x in (j+1)..self.width {
			if self.trees[i][x] >= cur_tree {
				invisible_count += 1;
				break;
			}
		}
		// above?
		for x in 0..i {
			if self.trees[x][j] >= cur_tree {
				invisible_count += 1;
				break;
			}
		}
		//below?
		for x in (i+1)..self.height {
			if self.trees[x][j] >= cur_tree {
				invisible_count += 1;
				break;
			}
		}

		if invisible_count == 4 {
			return false;
		}
		true
	}

	fn get_scenic_score(&self, i: usize, j: usize) -> u32 {
		let cur_tree = self.trees[i][j];

		let mut total_scenic: u32 = 1;
		let mut local_scenic: u32 = 0;

		// Let's get the count of trees to the left until the blocker/border
		for x in (0..j).rev() {
			local_scenic += 1;
			if self.trees[i][x] >= cur_tree {
				break;
			}
		}
		total_scenic *= local_scenic;
		local_scenic = 0;
		// Let's get the count of trees to the right until the blocker/border
		for x in (j+1)..self.width {
			local_scenic += 1;
			if self.trees[i][x] >= cur_tree {
				break;
			}
		}
		total_scenic *= local_scenic;
		local_scenic = 0;
		// above?
		for x in (0..i).rev() {
			local_scenic += 1;
			if self.trees[x][j] >= cur_tree {
				break;
			}
		}
		total_scenic *= local_scenic;
		local_scenic = 0;
		//below?
		for x in (i+1)..self.height {
			local_scenic += 1;
			if self.trees[x][j] >= cur_tree {
				break;
			}
		}
		total_scenic *= local_scenic;
		total_scenic
	}

	fn get_matrix(filename: &str) -> Vec<Vec<usize>> {
		let file = File::open(filename).unwrap();
		let reader = BufReader::new(file);

		// 1. get full file into vec<string>
		let mut matrix: Vec<Vec<usize>> = Vec::new();
		let mut i = 0;
		for line in reader.lines() {
			matrix.push(Vec::new());
			let line = line.unwrap();
			for c in line.chars() {
				matrix[i].push(c.to_digit(10).unwrap() as usize);
			}
			i += 1;
		}
		matrix
	}
}