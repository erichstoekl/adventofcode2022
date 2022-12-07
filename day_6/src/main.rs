use std::{env, collections::HashMap};

fn solve_problem(input: Vec<char>) {
	let mut char_map: HashMap<char, usize> = HashMap::new();
    let mut processed = 0;
    let input_length = input.len();
    let delimiter = 14;
    for i in 0..input_length {
        let c = input[i];
        if processed < delimiter {
            let val_ref = char_map.entry(c).or_insert(0);
            *val_ref += 1;
        } else if char_map.len() > (delimiter - 1) {
            println!("Found index at {}", i);
            break;
        } else {
            // Remove an element that is at current index - delimiter
            let fell_off_edge = input[i-delimiter];
            let val_entry = char_map.entry(fell_off_edge); 
            let val_ref = val_entry.or_default();
            *val_ref -= 1;
            if *val_ref == 0 {
                char_map.remove(&fell_off_edge);
            }
            let val_ref_two = char_map.entry(c).or_insert(0);
            *val_ref_two += 1;
        }
        processed += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: day_5 <input>");
        return;
    }
    let input = &args[1];
    solve_problem(input.chars().collect());
}