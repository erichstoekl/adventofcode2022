use std::env;
mod file_reader;
#[macro_use] extern crate scan_fmt;

fn perform_movement_v1(stacks: &mut Vec<Vec<char>>, instructions: Vec<(usize, usize, usize)>) -> &Vec<Vec<char>> {
    for ist in instructions.iter() {
        for _i in 0..ist.0 {
            let popped = stacks[ist.1].pop().unwrap();
            stacks[ist.2].push(popped);
        }
    }

    stacks
}

fn perform_movement_v2(stacks: &mut Vec<Vec<char>>, instructions: Vec<(usize, usize, usize)>) -> &Vec<Vec<char>> {
    for ist in instructions.iter() {
        let stack_height = stacks[ist.1].len();
        let remove_pos = stack_height - ist.0;
        for _i in 0..ist.0 {
            let popped = stacks[ist.1].remove(remove_pos);
            stacks[ist.2].push(popped);
        }
    }

    stacks
}

fn solve_problem_one(filename: &str) {
    let lines = file_reader::get_file_lines(filename);
    let mut stacks = file_reader::read_stacks_from_vec(&lines);
    let instructions = file_reader::read_instructions_from_vec(lines);
    let result = perform_movement_v1(&mut stacks, instructions);
    for res in result.iter() {
        print!("{}", res.last().unwrap());
    }
}

fn solve_problem_two(filename: &str) {
    let lines = file_reader::get_file_lines(filename);
    let mut stacks = file_reader::read_stacks_from_vec(&lines);
    let instructions = file_reader::read_instructions_from_vec(lines);
    let result = perform_movement_v2(&mut stacks, instructions);
    for res in result.iter() {
        print!("{}", res.last().unwrap());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: day_5 <filename> <problem_num>");
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
