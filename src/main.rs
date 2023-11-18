mod game_utils;
mod a_star_search;
use game_utils::GameBoard;
use a_star_search::a_star_search;
use std::time::SystemTime;
fn main() {
    let board = GameBoard::new();
    let start_time = SystemTime::now();
    a_star_search(board, 32);
    match SystemTime::now().duration_since(start_time) {
        Ok(duration) => {
            println!("Time taken: {:?}", duration);
        }
        Err(_) => {
            println!("An error occurred while measuring the time.");
        }
    }
}
