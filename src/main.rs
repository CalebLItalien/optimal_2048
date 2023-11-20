mod game_utils;
mod a_star_search;
use game_utils::GameBoard;
use a_star_search::a_star_search;
use std::env;
use std::time::SystemTime;
use std::fs::File;
use std::io::Write;
use crate::game_utils::{BOARD_COUNTER_LIST, TIME_TAKEN, MOVES_MADE};

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
                TIME_TAKEN.lock().unwrap().push(duration);
            }
            Err(_) => {
                println!("An error occurred while measuring the time.");
            }
        }
        BOARD_COUNTER_LIST.lock().unwrap().push(GameBoard::print_board_count());
        GameBoard::reset_board_counter();
        println!("=================================");
    }
    print_statistics();
    if let Err(e) = write_statistics_to_file() {
        eprintln!("Failed to write statistics to file: {}", e);
    }}

fn print_statistics() {
    let board_counter_list = BOARD_COUNTER_LIST.lock().unwrap();
    let time_taken = TIME_TAKEN.lock().unwrap();
    let moves_made = MOVES_MADE.lock().unwrap();

    println!("Board Counts:");
    for (index, count) in board_counter_list.iter().enumerate() {
        println!("{}: {}", index + 1, count);
    }

    println!("\nTime Taken:");
    for (index, duration) in time_taken.iter().enumerate() {
        let nanos = duration.as_nanos(); // Duration in nanoseconds
        println!("{}: {:.6} ms", index + 1, nanos as f64 / 1_000_000.0); // Converts nanoseconds to milliseconds
    }

    println!("\nMoves Made:");
    for (index, moves) in moves_made.iter().enumerate() {
        println!("{}: {}", index + 1, moves);
    }
}

fn write_statistics_to_file() -> std::io::Result<()> {
    let board_counter_list = BOARD_COUNTER_LIST.lock().unwrap();
    let time_taken = TIME_TAKEN.lock().unwrap();
    let moves_made = MOVES_MADE.lock().unwrap();

    let average_board_count = board_counter_list.iter().sum::<u32>() as f64 / board_counter_list.len() as f64;
    let average_time_taken_nanos = time_taken.iter().map(|d| d.as_nanos()).sum::<u128>() as f64 / time_taken.len() as f64;
    let average_time_taken_ms = average_time_taken_nanos / 1_000_000.0; // Converts the average time from nanoseconds to milliseconds
    let average_moves_made = moves_made.iter().sum::<usize>() as f64 / moves_made.len() as f64;

    let mut file = File::create("statistics.txt")?;

    writeln!(file, "Average Board Counter: {:.2}", average_board_count)?;
    writeln!(file, "Average Time Taken (ms): {:.6}", average_time_taken_ms)?; // Six decimal places for milliseconds
    writeln!(file, "Average Moves Made: {:.2}", average_moves_made)?;

    Ok(())
}
