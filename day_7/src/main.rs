use std::result;
use std::{env, fs::File, collections::HashMap, borrow::Borrow};
use std::str::FromStr;
use rctree::{Node};
mod file_reader;
#[macro_use] extern crate scan_fmt;
extern crate rctree;

static mut global: u64 = 0;

fn recur_build_vector(root: &Node<(String, bool, u64)>, depth: usize) -> u64 {
    let mut total: u64  = 0;
    let data = root.borrow();
    let s = String::from_str(&std::iter::repeat(" ").take(depth).collect::<String>()).unwrap();
    /*
    if data.1 {
        println!("{}- {} (dir)", s, data.0);
    } else {
        println!("{}- {} (file, size={})", s, data.0, data.2);
    }
*/
    if !root.has_children() {
        return data.2;
    }
    let children = root.children();
    for child in children {
        let t = recur_build_vector(&child, depth + 2);
        total += t;
    }

    
    if total <= 100000 {
        println!("Total is {}", total);
        unsafe {
            global += total;
        }
    }
    total
}

fn solve_problem_one(filename: &str) {
    let root = file_reader::get_file_tree(filename);
    recur_build_vector(&root, 0);
    unsafe {
        println!("Final result: {}", global);
    }
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