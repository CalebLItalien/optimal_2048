const POSSIBLE_MOVES: [&str; 4] = ["up", "left", "down", "right"];
const GRID_SIZE: usize = 4;
use crate::game_utils::GameBoard;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::{Ordering, Reverse};

#[derive(Eq)]
struct QueueItem {
    cost: u64,
    moves: u64,
    game_board: GameBoard,
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

pub fn a_star_search(start: GameBoard, goal: u64){
    println!("Starting board:");
    start.print_pretty(); 
    
    let mut open_set: BinaryHeap<Reverse<QueueItem>> = BinaryHeap::new();
    let mut closed_set: HashSet<GameBoard> = HashSet::new();

    open_set.push(Reverse(QueueItem { cost: 0, moves: 0, game_board: start.clone() }));    
    let mut came_from: HashMap<GameBoard, (GameBoard, String)> = HashMap::new();

    while let Some(Reverse(queue_item)) = open_set.pop() {
        let current = queue_item.game_board;
        let moves_made = queue_item.moves; 
        if current.is_goal(goal) { 
            reconstruct_path(came_from, current);
            return;
        }

        if !closed_set.insert(current.clone()) {
            continue;
        }

        for direction in POSSIBLE_MOVES {
            let mut neighbor = current.clone();
            if neighbor.make_move(direction) {
                let cost = heuristic(&neighbor) + moves_made + 1;
                if !closed_set.contains(&neighbor) {
                    open_set.push(Reverse(QueueItem {
                        cost: cost, 
                        moves: moves_made + 1,
                        game_board: neighbor.clone()
                    }));
                    came_from.insert(neighbor, (current.clone(), direction.to_string()));
                }
            }
        }
    }    
}
pub fn heuristic(board: &GameBoard) -> u64 {
    let max_tile = board.grid.iter().flatten().max().cloned().unwrap_or(0);
    let empty_cells = board.grid.iter().flatten().filter(|&&x| x == 0).count() as u64;
    let mut monotonicity = 0;
    let mut smoothness = 0;
    let mut merges = 0;

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            // Smoothness
            if j < GRID_SIZE - 1 && board.grid[i][j] != 0 && board.grid[i][j + 1] != 0 {
                smoothness += (board.grid[i][j] as i64 - board.grid[i][j + 1] as i64).abs() as u64;
            }
            if i < GRID_SIZE - 1 && board.grid[i][j] != 0 && board.grid[i + 1][j] != 0 {
                smoothness += (board.grid[i][j] as i64 - board.grid[i + 1][j] as i64).abs() as u64;
            }

            // Monotonicity 
            if j < GRID_SIZE - 1 && board.grid[i][j] != 0 {
                monotonicity += board.grid[i][j].checked_sub(board.grid[i][j + 1]).unwrap_or(0);
            }
            if i < GRID_SIZE - 1 && board.grid[i][j] != 0 {
                monotonicity += board.grid[i][j].checked_sub(board.grid[i + 1][j]).unwrap_or(0);
            }

            // Count potential merges
            if i < GRID_SIZE - 1 && board.grid[i][j] == board.grid[i + 1][j] && board.grid[i][j] != 0 {
                merges += 1;
            }
            if j < GRID_SIZE - 1 && board.grid[i][j] == board.grid[i][j + 1] && board.grid[i][j] != 0 {
                merges += 1;
            }
        }
    }
    let max_tile_in_corner = if board.grid[0][0] == max_tile
        || board.grid[0][GRID_SIZE - 1] == max_tile
        || board.grid[GRID_SIZE - 1][0] == max_tile
        || board.grid[GRID_SIZE - 1][GRID_SIZE - 1] == max_tile
    {
        1
    } else {
        0
    };
    25 * max_tile_in_corner as u64 + empty_cells + 2 * merges + 2 * monotonicity + 2 * smoothness
}

pub fn reconstruct_path(came_from: HashMap<GameBoard, (GameBoard, String)>, current: GameBoard) {
    let mut path: Vec<(GameBoard, String)> = Vec::new();
    let mut current_state = current.clone();
    
    while let Some((parent, move_dir)) = came_from.get(&current_state) {
        path.push((current_state.clone(), move_dir.clone()));
        current_state = parent.clone();
    }
    
    path.reverse();

    for (board, direction) in &path {
        println!("Move: {}", direction);
        board.print_pretty();
    }
    println!("Number of moves: {}", path.len());
}

