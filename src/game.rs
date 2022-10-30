use crate::cell::Cell;
use crate::{player::Player};
use std::io::{stdin, stdout, Write};
use std::process;

pub struct Game{
    pub current_player: Player,
}

type Board = [Cell; 9];

impl Game{
    pub fn new() -> Self{
        Game { current_player: Player::Cross }
    }

    pub fn start(&mut self){
        let mut board = [Cell::Empty; 9];
        self.current_player = Player::Cross;
        loop{
            self.draw_board(&board);
            println!("Current player move: {}", self.current_player);
            self.read_command(&mut board);
        }
    }

    fn draw_board(&self, board: &Board){
        for (i, row) in board.chunks(3).enumerate(){
            for(j, cell) in row.iter().enumerate(){
                self.print_cell(cell);
            }
            print!("\n");
        }
    }

    fn print_cell(&self, cell: &Cell){
        match cell{
            Cell::Empty => {
                print!("[-]");
            },
            Cell::Player(Player::Circle) => {
                print!("[O]");
            },  
            Cell::Player(Player::Cross) => {
                print!("[X]");
            }
        }    
    }
    
    fn process_command(&mut self, input: &String, board: &mut Board){
        let input_str = input.trim();
        match input_str{
            "q" | "Q" | "" | " " => {
                process::exit(0);
            },
            index_str => {
                let mut index: usize = index_str.parse().expect("Could not parse.");
                index -= 1;
                match board[index]{
                    Cell::Empty => {
                        board[index] = Cell::Player(self.current_player);
                        self.current_player = self.opposite_player();
                        let win_cell = self.check_win(board, index);
                        if(win_cell == Cell::Empty){
                            let mut tie: bool = true;
                            for cell in board{
                                if *cell == Cell::Empty{
                                    tie = false;
                                    break;
                                }
                            }
                            if tie{
                                print!("\x1B[2J\n\r");
                                stdout().flush().expect("Unable to flush!!! \"stdout\"");
                                println!("Tie!");
                                self.start();
                            }
                            return;
                        }
                        print!("\x1B[2J\n\r");
                        stdout().flush().expect("Unable to flush!!! \"stdout\"");
                        println!("{} won!", board[index]);
                        self.start();
                    },
                    _ => {
                        self.read_command(board);
                    }
                }
            }
        }
    }

    fn check_win(&self, board: &Board, last_move: usize) -> Cell{
        let last_move_player = board[last_move];
        let diagonal_win = self.diagonal_win_check(board, last_move, &last_move_player);
        let horizontal_win = self.horizontal_win_check(board, last_move, &last_move_player);
        let vertical_win = self.vertical_win_check(board, last_move, &last_move_player);
        if diagonal_win || horizontal_win || vertical_win{
            return last_move_player;
        }
        Cell::Empty
    }

    fn diagonal_win_check(&self, board: &Board, last_move: usize, last_move_player: &Cell) -> bool{
        match last_move{
            4 => {
                if (&board[last_move-4] == last_move_player && &board[last_move+4] == last_move_player) 
                || (&board[last_move-2] == last_move_player && &board[last_move+2] == last_move_player) {
                    return true;
                }
            },
            0 => {
                if &board[last_move+4] == last_move_player && &board[last_move+8] == last_move_player {
                    return true;
                }
            },
            8 => {
                if &board[last_move-4] == last_move_player && &board[last_move-8] == last_move_player{
                    return true;
                }
            },
            2 => {
                if &board[last_move+2] == last_move_player && &board[last_move+6] == last_move_player{
                    return true;
                }
            },
            6 => {
                if &board[last_move-2] == last_move_player && &board[last_move-6] == last_move_player{
                    return true;
                }
            },
            _ => {
            }
        };
        return false;
    }

    fn horizontal_win_check(&self, board: &Board, last_move: usize, last_move_player: &Cell) -> bool{
        match last_move{
            0 | 3 | 6 => {
                if &board[last_move+1] == last_move_player && &board[last_move+2] == last_move_player {
                    return true;
                } 
            },
            2 | 5 | 8 => {
                if &board[last_move-1] == last_move_player && &board[last_move-2] == last_move_player {
                    return true;
                }
            },
            _ => {
                if &board[last_move+1] == last_move_player && &board[last_move-1] == last_move_player{
                    return true;
                }
            }
        };
        return false;
    }

    fn vertical_win_check(&self, board: &Board, last_move: usize, last_move_player: &Cell) -> bool{
        match last_move{
            0..=2 => {
                if &board[last_move+3] == last_move_player && &board[last_move+6] == last_move_player{
                    return true;
                }
            },
            6..=8 => {
                if &board[last_move-3] == last_move_player && &board[last_move-6] == last_move_player{
                    return true;
                }
            },
            _ => {
                if &board[last_move+3] == last_move_player && &board[last_move-3] == last_move_player{
                    return true;
                }
            }
        };
        return false;
    }

    fn opposite_player(&self) -> Player{
        let opposite = if self.current_player == Player::Circle{
            Player::Cross
        }else{
            Player::Circle
        };
        return opposite;
    }

    fn read_command(&mut self, board: &mut Board){
        print!("Enter your next move: ");
        stdout().flush().expect("Failed to flush output.");
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("Could not read line.");
        self.process_command(&input, board);
    }
}