use std::result;
use std::{env, fs::File, collections::HashMap, borrow::Borrow};
use std::str::FromStr;
use rctree::{Node};
mod file_reader;
#[macro_use] extern crate scan_fmt;
extern crate rctree;

fn recur_build_vector(root: &Node<(String, bool, u64)>, depth: usize, v: &mut Vec<u64>) -> (u64, Vec<u64>) {
    let mut total: u64  = 0;
    let data = root.borrow();
    let s = String::from_str(&std::iter::repeat(" ").take(depth).collect::<String>()).unwrap();

    if !root.has_children() {
        return (data.2);
    }
    let children = root.children();
    for child in children {
        let (t, v) = recur_build_vector(&child, depth + 2, v);
        total += t;
    }
    
    if total <= 100000 {
        v.push(total);
        (total, v.to_vec())
    } else {
        (0, v.to_vec())
    }
}

fn solve_problem_one(filename: &str) {
    let root = file_reader::get_file_tree(filename);
    let mut v = Vec::new();
    recur_build_vector(&root, 0, &mut v);

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