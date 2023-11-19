mod game_utils;
mod a_star_search;
use game_utils::GameBoard;
use a_star_search::a_star_search;
use std::env;
use std::time::SystemTime;

#[macro_use]
extern crate lazy_static;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <iterations> <goal>", args[0]);
        std::process::exit(1);
    }

    let iterations: u32 = args[1].parse().expect("Error converting iterations to a number");
    let goal: u64 = args[2].parse().expect("Error converting goal to a number");

    println!("=================================");
    for _ in 0..iterations {
        let board = GameBoard::new();
        let start_time = SystemTime::now();
        a_star_search(board, goal);
        match SystemTime::now().duration_since(start_time) {
            Ok(duration) => {
                println!("Time taken: {:?}", duration);
            }
            Err(_) => {
                println!("An error occurred while measuring the time.");
            }
        }
        GameBoard::print_board_count();
        GameBoard::reset_board_counter();
        println!("=================================");
    }
}
