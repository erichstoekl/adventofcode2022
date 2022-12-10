use std::env;
mod lib;

fn solve_problem_one(filename: &str) {
    let forest: lib::Forest = lib::Forest::new(filename);
    let vis_count = forest.count_visible_trees();
    println!("Total: {}", vis_count);
}

fn solve_problem_two(filename: &str) {
    let forest: lib::Forest = lib::Forest::new(filename);
    let optimal_scenic = forest.get_optimal_scenic_score();
    println!("Optimal scenic: {}", optimal_scenic);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: day_8 <filename> <problem_num>");
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
