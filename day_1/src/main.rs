use std::fs::File;
use std::io::{BufReader,BufRead};

fn problem_one(inp: Vec<String>) {
    println!("Running problem 1...");
    let mut collector: i64 = 0;
    let mut maxval: i64 = 0;
    for val in inp.iter() {
        if val.eq(&"") {
            //println!("Empty line!");
            if collector > maxval {
                maxval = collector;
            }
            collector = 0;
        } else {
            collector += val.parse::<i64>().unwrap();
            //println!("{}", collector);
        }
    }
    println!("Max val is {}", maxval)
}

fn problem_two(inp: Vec<String>) {
    println!("Running problem 1...");
    let mut collector: i64 = 0;
    let mut ringbuf: Vec<i64> = Vec::with_capacity(3);
    for val in inp.iter() {
        if val.eq(&"") {
            println!("Empty line");
            if ringbuf.len() < 3 {
                ringbuf.push(collector);
            } else {
                let min_val = ringbuf.iter().min().unwrap();
                println!("Min val is {}", min_val);
                if &collector > min_val {
                    // Get position of min_val:
                    let min_position = ringbuf.iter().position(|&x| x == *min_val).unwrap();
                    println!("coll is {}, min pos is {}", &collector, &min_position);
                    ringbuf[min_position] = collector;
                }
            }
            collector = 0;
        } else {
            let parsed_val = val.parse::<i64>().unwrap();
            println!("Parsed val: {}", &parsed_val);
            collector += parsed_val;
        }
    }
    let mut result: i64 = 0;
    for r_val in ringbuf.iter() {
        result += r_val;
    }
    println!("Problem 2 result: {}", result);
}

fn main() {
    let inp = read_input();
    problem_two(inp);
}

fn read_input() -> Vec<String> {
    let file = File::open("./src/input.txt").unwrap();
    let mut array: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        array.push(line.unwrap());
    }
    return array;
}