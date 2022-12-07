use std::env;
mod file_reader;

fn solve_problem_one(filename: &str) {
    let work_pairs = file_reader::read_work_pairs_from_file(filename);
    let mut total: u32 = 0;

    for pair in work_pairs.iter() {
        // Is elf 2 completely contained in elf 1?
        if pair.0 <= pair.2 && pair.1 >= pair.3 {
            total += 1;
        }
        // Is elf 1 completely contained in elf 2?
        else if pair.0 >= pair.2 && pair.1 <= pair.3 {
            total += 1;
        }
    }
    println!("Problem 1 solution: {}", total)
}

fn solve_problem_two(filename: &str) {
    let work_pairs = file_reader::read_work_pairs_from_file(filename);
    let mut total: u32 = 0;

    for pair in work_pairs.iter() {
        // Is elf 2 partially contained in elf 1?
        if pair.0 <= pair.3 && pair.1 >= pair.2 {
            total += 1;
        }
    }
    println!("Problem 2 solution: {}", total)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: day_4 <filename> <problem_num>");
        return;
    }
    let filename = &args[1];
    let problem: u32 = args[2].parse().unwrap();
    if problem == 1 {
        solve_problem_one(filename);
    } else if problem == 2 {
        solve_problem_two(filename);
    }
}
