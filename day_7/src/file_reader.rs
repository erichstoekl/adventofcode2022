use std::fs::File;
use std::io::{BufRead, BufReader};

use scan_fmt::parse::scan;
use rctree::Node;

pub fn get_file_tree(filename: &str) -> Node<(String, bool, u64)> {
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	let root: Node<(String, bool, u64)> = Node::new(("/".to_string(), true, 0));
	let mut iterator = root.clone();
	
	for line in reader.lines() {
		let line = line.unwrap();
		if let Ok(t_d) = scan_fmt!(&line, "$ cd {}", String) {
			if t_d == ".." {
				iterator = iterator.parent().unwrap();
			} else {
				for child in iterator.children() {
					if child.borrow().0 == t_d {
						iterator = child
					}
				}
			}
		} else if let Ok((size, file_name)) = scan_fmt!(&line, "{d} {}", u64, String) {
			iterator.append(Node::new((file_name, false, size)));
		} else if let Ok(dir_name) = scan_fmt!(&line, "dir {}", String) {
			iterator.append(Node::new((dir_name, true, 0)));
		};
	}
	root
}