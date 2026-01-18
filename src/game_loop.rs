use std::io::{self, Write};

use crate::objects::{Board, Cell, Game, Player};

pub fn render_board(board: &Board, player: &Player) {
    println!("+-------+-------+-------+");
    for (i, row) in board.cells.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            println!("+-------+-------+-------+");
        }
        print!("| ");
        for (j, cell) in row.iter().enumerate() {
            if j > 0 && j % 3 == 0 {
                print!("| ");
            }

            if player.row == i && player.col == j {
                print!("@ ");
            } else {
                match cell {
                    Cell::Empty => print!(". "),
                    Cell::Given(n) | Cell::Filled(n) => print!("{} ", n),
                }
            }
        }
        println!("|");
    }
    println!("+-------+-------+-------+");
}

pub fn read_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase())
}

pub fn handle_movement(input: &str, player: &mut Player) {
    match input {
        "w" => player.row -= 1,
        "s" => player.row += 1,
        "a" => player.col -= 1,
        "d" => player.col += 1,
        _ => return,
    }
}

pub fn handle_cell_input(input: &str, game: &mut Game) -> Result<(), Box<dyn std::error::Error>> {
    let r = game.player.row;
    let c = game.player.col;

    match input {
        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
            let num = input.parse::<u8>()?;
            if let Cell::Empty | Cell::Filled(_) = game.board.cells[r][c] {
                game.board.cells[r][c] = Cell::Filled(num);
                Ok(())
            } else {
                return Err("Cannot edit given cells".into());
            }
        },
        "0" => {
            game.board.cells[r][c] = Cell::Empty;
            Ok(())
        },
        "enter" | "" => {
            Ok(())
        },
        "w" | "s" | "a" | "d" => {
            // Movement keys are handled by handle_movement, no cell input needed
            Ok(())
        },
        _ => return Err("Invalid input".into()),
    }
}

pub fn is_solved(board: &Board, solution: &Board) -> bool {
    for r in 0..9 {
        for c in 0..9 {
            if board.cells[r][c].value() != solution.cells[r][c].value() {
                return false;
            }
        }
    }
    true
}