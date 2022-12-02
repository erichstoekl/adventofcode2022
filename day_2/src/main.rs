use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_score_for_tup(tup_vec: &Vec<(char, char)>, problem: i32) -> i32 {
    let mut total_score: i32 = 0;
    let matrix: [[i32; 3]; 3] = if problem == 1 {
        [[3, 0, 6], [6, 3, 0], [0, 6, 3]]
    } else {
        [[3, 1, 2], [4, 5, 6], [8, 9, 7]]
    };
    let static_A = 'A' as i32;
    let static_X = 'X' as i32;
    for tup in tup_vec.iter() {
        let opponent = (tup.0 as i32) - static_A;
        let myself = (tup.1 as i32) - static_X;
        total_score += matrix[myself as usize][opponent as usize];
        if problem == 1 {
            total_score += myself + 1;
        }
    }
    total_score
}

fn read_tuples_from_file(filename: &str) -> Vec<(char, char)> {
    let mut tuples = Vec::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let first = parts.next().unwrap().parse::<char>().unwrap();
        let second = parts.next().unwrap().parse::<char>().unwrap();
        tuples.push((first, second));
    }
    tuples
}

fn main() {
    let tup_vec = read_tuples_from_file("./src/input.txt");
    let total_score = calculate_score_for_tup(&tup_vec, 2);
    println!("Total score is: {}", total_score);
}
