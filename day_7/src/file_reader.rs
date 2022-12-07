use std::cell::Cell;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

use scan_fmt::parse::scan;

use crate::FileNode;

pub fn get_file_lines(filename: &str) {
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	let mut dir_tree: HashMap<String, Cell<FileNode>> = HashMap::new();
	dir_tree.insert("/".to_string(), Cell::new({ FileNode {
		is_dir: true,
		size: 0,
		children: HashMap::new()
	} }));

	let mut cur_pos: Cell<FileNode> = *dir_tree.get("/").unwrap();
	for line in reader.lines() {
		let line = line.unwrap();
		if let Ok(t_d) = scan_fmt!(&line, "$ cd {}", String) {
			if cur_pos.get().children.contains_key(&t_d) {
				cur_pos = cur_pos.children.get(&t_d).unwrap().get_mut();
			}
		} else if let Ok((size, file_name)) = scan_fmt!(&line, "{d} {}", u64, String) {
			println!("Got size and file_name");
			let insert_file: Cell<FileNode> = Cell::new({ FileNode {
				is_dir: false,
				size: size,
				children: HashMap::new()
			} });
			cur_pos.children.insert(file_name, insert_file);
		} else if let Ok(dir_name) = scan_fmt!(&line, "dir {}", String) {
			println!("Got dir name");
		};
	}
	println!("Done!");
}