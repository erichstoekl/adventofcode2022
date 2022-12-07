use std::{env, fs::File, collections::HashMap};
mod file_reader;
use std::cell::Cell;
#[macro_use] extern crate scan_fmt;

#[derive (Clone, Copy)]
struct FileNode {
    is_dir: bool,
    size: u64,
    children: HashMap<String, Cell<FileNode>>,
}

fn solve_problem_one(filename: &str) {
    file_reader::get_file_lines(filename);
}

fn solve_problem_two(filename: &str) {

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: day_7 <filename> <problem_num>");
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