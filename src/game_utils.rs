const GRID_SIZE: usize = 4;
const POSSIBLE_MOVES: [&str; 4] = ["up", "left", "down", "right"];
use rand::{Rng, thread_rng};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameBoard {
    pub grid: [[u64; GRID_SIZE]; GRID_SIZE],
}

impl GameBoard {
    pub fn new() -> GameBoard {
        let mut new_gameboard = GameBoard {
            grid: [[0; GRID_SIZE]; GRID_SIZE],
        };
        new_gameboard.spawn_new_tile();
        new_gameboard.spawn_new_tile();
        new_gameboard
    }

    pub fn print_pretty(&self) {
        for row in self.grid.iter() {
            println!("[{}]", row.iter()
                .map(|&num| if num == 0 { String::from("_") } else { num.to_string() })
                .collect::<Vec<String>>()
                .join(" "));
        }
        println!(); 
    }
    // pub fn reconstruct_path(path: Vec<(GameBoard, String)>, start: GameBoard, goal: u64){
    //     println!("Starting board: ");
    //     println!("Goal: {}", goal);
    //     start.print_pretty();

    //     for (board, direction) in path {
    //         println!("Move: {}", direction);
    //         board.print_pretty();
    //     }
    // }
    
    pub fn get_possible_new_states(&self) -> Vec<GameBoard> {
        /*
        Returns the possible new states of the board
         */
        let mut new_states = Vec::new();
        for &direction in &POSSIBLE_MOVES {
            let mut board_copy = self.copy();
            let success = board_copy.make_move(direction);
            if success{
                new_states.push(board_copy);
            }
        }
        new_states
    }

    pub fn is_goal(&self, goal: u64) -> bool {
        /*
        Returns whether or not the board is in a winning state
         */
        self.grid.iter().flatten().any(|&tile| tile == goal)
    }

    pub fn make_move(&mut self, direction: &str) -> bool {
        /*
        Makes a move on the board, in the specified direction
         */
       match direction {
        "left" => self.move_left(),
        "right" => self.move_right(),
        "down" => self.move_down(),
        "up" => self.move_up(),
        _ => false,
       }
    }
    fn move_left(&mut self) -> bool {
        /*
        Moves tiles to the left and merges if possible
         */
        let mut moved = false;
        for row in &mut self.grid {
            let mut target = 0;
            for i in 0..GRID_SIZE {
                if row[i] != 0 {
                    if target != i {
                        row[target] = row[i];
                        row[i] = 0;
                        moved = true;
                    }
                    target += 1;
                }
            }
            for _ in 0..3 { 
                for i in 0..GRID_SIZE - 1 {
                    if row[i] != 0 && row[i] == row[i + 1] {
                        row[i] *= 2;
                        row[i + 1] = 0;
                        moved = true;
                        break; 
                    }
                }
                target = 0;
                for i in 0..GRID_SIZE {
                    if row[i] != 0 {
                        if target != i {
                            row[target] = row[i];
                            row[i] = 0;
                            moved = true;
                        }
                        target += 1;
                    }
                }
            }
        }
        if moved { self.spawn_new_tile(); }
        moved
    }
    fn move_right(&mut self) -> bool {
        /*
        Moves tiles to the right and merges if possible
         */
        let mut moved = false;
        for row in &mut self.grid {
            let mut target = GRID_SIZE - 1;
            for i in (0..GRID_SIZE).rev() {
                if row[i] != 0 {
                    if target != i {
                        row[target] = row[i];
                        row[i] = 0;
                        moved = true;
                    }
                    target = target.saturating_sub(1);
                }
            }
            for _ in 0..3 {
                for i in (1..GRID_SIZE).rev() {
                    if row[i] != 0 && row[i] == row[i - 1] {
                        row[i] *= 2;
                        row[i - 1] = 0;
                        moved = true;
                        break;
                    }
                }
                target = GRID_SIZE - 1;
                for i in (0..GRID_SIZE).rev() {
                    if row[i] != 0 {
                        if target != i {
                            row[target] = row[i];
                            row[i] = 0;
                            moved = true;
                        }
                        target = target.saturating_sub(1);
                    }
                }
            }
        }
        if moved { self.spawn_new_tile(); }
        moved
    }

    fn move_up(&mut self) -> bool {
        /*
        Moves tiles up and merges if possible
         */
        let mut moved = false;
        for j in 0..GRID_SIZE {
            let mut target = 0;
            for i in 0..GRID_SIZE {
                if self.grid[i][j] != 0 {
                    if target != i {
                        self.grid[target][j] = self.grid[i][j];
                        self.grid[i][j] = 0;
                        moved = true;
                    }
                    target += 1;
                }
            }
            for _ in 0..3 {
                for i in 0..GRID_SIZE - 1 {
                    if self.grid[i][j] != 0 && self.grid[i][j] == self.grid[i + 1][j] {
                        self.grid[i][j] *= 2;
                        self.grid[i + 1][j] = 0;
                        moved = true;
                        break;
                    }
                }
                target = 0;
                for i in 0..GRID_SIZE {
                    if self.grid[i][j] != 0 {
                        if target != i {
                            self.grid[target][j] = self.grid[i][j];
                            self.grid[i][j] = 0;
                            moved = true;
                        }
                        target += 1;
                    }
                }
            }
        }
        if moved { self.spawn_new_tile(); }
        moved
    }

    fn move_down(&mut self) -> bool {
        /*
        Moves tiles down and merges if possible
         */
        let mut moved = false;
        for j in 0..GRID_SIZE {
            let mut target = GRID_SIZE - 1;
            for i in (0..GRID_SIZE).rev() {
                if self.grid[i][j] != 0 {
                    if target != i {
                        self.grid[target][j] = self.grid[i][j];
                        self.grid[i][j] = 0;
                        moved = true;
                    }
                    target = target.saturating_sub(1);
                }
            }
            for _ in 0..3 {
                for i in (1..GRID_SIZE).rev() {
                    if self.grid[i][j] != 0 && self.grid[i][j] == self.grid[i - 1][j] {
                        self.grid[i][j] *= 2;
                        self.grid[i - 1][j] = 0;
                        moved = true;
                        break;
                    }
                }
                target = GRID_SIZE - 1;
                for i in (0..GRID_SIZE).rev() {
                    if self.grid[i][j] != 0 {
                        if target != i {
                            self.grid[target][j] = self.grid[i][j];
                            self.grid[i][j] = 0;
                            moved = true;
                        }
                        target = target.saturating_sub(1);
                    }
                }
            }
        }
        if moved { self.spawn_new_tile(); }
        moved
    }

    pub fn copy(&self) -> GameBoard {
        /*
        Returns a clone of the board
         */
        GameBoard {
            grid: self.grid.clone(),
        }
    }

    pub fn spawn_new_tile(&mut self) {
        /*
        Spawns two new tiles on the board
         */
        let mut positions = Vec::new();
        for (i, row) in self.grid.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                if value == 0 {
                    positions.push((i, j));
                }
            }
        }
        let length_positions = positions.len();
        if length_positions > 0{
            let mut rng = thread_rng();
            let to_change = rng.gen_range(0..length_positions);
            let prob = rng.gen_range(1..=10);

            let (row, col) = positions[to_change];
            if prob > 1{
                self.grid[row][col] = 2;
            } else {
                self.grid[row][col] = 4;
            }
        }
    }
}
