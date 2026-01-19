use std::io::{self, Write, stdout};

use crossterm::{
    cursor, execute,
    terminal::{Clear, ClearType},
};

use crate::{
    objects::{Board, Cell, Game, GameState, InputReturnType, Player, TerminalGuard},
    terminal::{
        clear_screen, failure_message, handle_key_input, read_key, render_board, restore_terminal,
        setup_terminal, success_message,
    },
};

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
        }
        "0" => {
            game.board.cells[r][c] = Cell::Empty;
            Ok(())
        }
        "enter" | "" => Ok(()),
        "w" | "s" | "a" | "d" => {
            // Movement keys are handled by handle_movement, no cell input needed
            Ok(())
        }
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

pub fn run_game(game: &mut Game) -> Result<(), Box<dyn std::error::Error>> {
    setup_terminal()?;
    let _guard = TerminalGuard;

    loop {
        clear_screen()?;
        render_board(game)?;

        // If game is solved or failed, show message and wait for quit
        if game.state == GameState::Solved {
            success_message()?;
            if let InputReturnType::Exit = handle_key_input(game, read_key()?)? {
                break;
            }
            continue;
        } else if game.state == GameState::Failed {
            failure_message()?;
            match handle_key_input(game, read_key()?)? {
                InputReturnType::Exit => break,
                _ => {
                    game.state = GameState::Playing;
                    continue;
                }
            }
        }

        match handle_key_input(game, read_key()?)? {
            InputReturnType::Exit => break (),
            InputReturnType::CheckPuzzle => {
                if is_solved(&game.board, &game.solution) {
                    game.state = GameState::Solved;
                } else {
                    game.state = GameState::Failed;
                }
                // Re-render to show the message immediately
                continue;
            }
            InputReturnType::Continue => continue,
        };
    }

    restore_terminal()?;
    Ok(())
}
