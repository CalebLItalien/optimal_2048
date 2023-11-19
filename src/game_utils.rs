const GRID_SIZE: usize = 4;
use rand::{Rng, thread_rng};

use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameBoard {
    pub grid: [[u64; GRID_SIZE]; GRID_SIZE],
}

trait PadToWidth {
    fn pad_to_width(&self, width: usize) -> String;
}

impl PadToWidth for String {
    fn pad_to_width(&self, width: usize) -> String {
        format!("{:<width$}", self, width = width)
    }
}

lazy_static! {
    static ref MOVE_COUNTER: Mutex<u32> = Mutex::new(0);
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

    fn increment_move_counter() {
        let mut num_moves = MOVE_COUNTER.lock().unwrap();
        *num_moves += 1;
    }
    
    pub fn reset_move_counter() {
        let mut num_moves = MOVE_COUNTER.lock().unwrap();
        *num_moves = 0;
    }
    
    pub fn print_pretty(&self) {
        let horizontal_line = "---------------------------------";
        println!("{}", horizontal_line);
        for row in self.grid.iter() {
            let mut formatted_row = String::from("|");
            for &num in row.iter() {
                let cell = if num == 0 { "".to_string() } else { num.to_string() };
                formatted_row += &format!(" {:^5} |", cell);
            }
            println!("{}", formatted_row);
            println!("{}", horizontal_line);
        }
        println!();
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
        if moved { 
            GameBoard::increment_move_counter();
            self.spawn_new_tile(); 
        }
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
        if moved { 
            GameBoard::increment_move_counter();
            self.spawn_new_tile(); 
        }
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
        if moved { 
            GameBoard::increment_move_counter();
            self.spawn_new_tile(); 
        }
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
        if moved { 
            GameBoard::increment_move_counter();
            self.spawn_new_tile();
         }
        moved
    }

    pub fn spawn_new_tile(&mut self) {
        let mut rng = thread_rng();
        let empty_count = self.grid.iter().flatten().filter(|&&x| x == 0).count();
    
        if empty_count == 0 {
            return;
        }
        let mut random_pos = rng.gen_range(1..=empty_count);
    
        'outer: for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if self.grid[i][j] == 0 {
                    random_pos -= 1;
                    if random_pos == 0 {
                        self.grid[i][j] = if rng.gen_range(1..=10) > 1 { 2 } else { 4 };
                        break 'outer;
                    }
                }
            }
        }
    }

    pub fn print_board_count() {
        let num_moves = MOVE_COUNTER.lock().unwrap();
        println!("Total number of boards made: {}", *num_moves);
    }
    
}
