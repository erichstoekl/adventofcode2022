use std::collections::HashMap;

pub fn find_common_char_group_of_three(group: Vec<String>) -> char {
	let mut char_map: HashMap<char, usize> = HashMap::new();
	for i in 0..3 {
		let mut this_group_map: HashMap<char, bool> = HashMap::new();
		for c in group[i].chars() {
			this_group_map.insert(c, true);
		}
		for (k, v) in this_group_map.iter() {
			let val_reference: &mut usize = char_map.entry(*k).or_insert(0);
			*val_reference += 1;
		}
	}
	for (c, count) in char_map.iter() {
		if *count == 3 {
			return *c;
		}
	}
	return '0';
}

pub fn find_common_char_between_compartments(rucksack: (String, String)) -> char {
	let mut char_map = HashMap::new();
	for c in rucksack.0.chars() {
		char_map.insert(c, true);
	}
	for c in rucksack.1.chars() {
		if char_map.contains_key(&c) {
			return c;
		}
	}
	return '0';
}

pub fn get_item_priority(c: char) -> u32 {
	if c >= 'A' && c <= 'Z' {
		return (c as u32) - ('A' as u32) + 27;
	} else {
		return (c as u32) - ('a' as u32)  + 1;
	}
}