const POSSIBLE_MOVES: [&str; 4] = ["up", "left", "down", "right"];
use crate::game_utils::GameBoard;
use std::collections::{BinaryHeap, HashMap};

// pub fn a_star_search(start: GameBoard, goal: u64) -> Option<Vec<(GameBoard, String)>> {
// }
pub fn heuristic(board: &GameBoard) -> u64 {
    let max_tile = board.grid.iter().flatten().max().cloned().unwrap_or(0);
    if max_tile == 0 {
        u64::MAX
    } else {
        u64::MAX / max_tile
    }
}
