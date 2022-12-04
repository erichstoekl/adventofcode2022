use std::env;
mod solver;
mod file_reader;

fn solve_problem_one(filename: &str) {
    let rucksacks: Vec<(String, String)> = file_reader::read_rucksacks_from_file(filename);
    let mut total: u32 = 0;
    for rucksack in rucksacks.into_iter() {
        let common_char = solver::find_common_char_between_compartments(rucksack);
        let priority = solver::get_item_priority(common_char);
        total += priority;
    }
    println!("total priority is: {}", total);
}

fn solve_problem_two(filename: &str) {
    let groups_of_three: Vec<Vec<String>> = file_reader::read_groups_from_file(filename);
    let mut total = 0;
    for group in groups_of_three {
        let common_char = solver::find_common_char_group_of_three(group);
        let priority_val = solver::get_item_priority(common_char);
        total += priority_val;
    }
    println!("Result for problem two: {}", total);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check the number of arguments
    if args.len() != 3 {
        println!("Usage: day_3 <filename> <problem_num>");
        return;
    }

    let filename = &args[1];
    let problem: u32 = args[2].parse().unwrap();
    if problem == 1 {
        solve_problem_one(filename);
    } else {
        solve_problem_two(filename);
    }
}