use crossterm::event::KeyCode;

use crate::{
    generator::generate_sudoku,
    objects::{Board, Difficulty, Game, GameState, InputReturnType, Player, TerminalGuard},
    terminal::{
        clear_screen, failure_message, handle_key_input, read_key, render_board,
        render_difficulty_selector, restore_terminal, setup_terminal, success_message,
    },
};

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

pub fn run_game() -> Result<(), Box<dyn std::error::Error>> {
    setup_terminal()?;
    let _guard = TerminalGuard;

    let difficulty = difficulty_selector()?;

    let sudoku = generate_sudoku(difficulty);
    let player = Player::new();
    let mut game = Game::new(sudoku.difficulty, sudoku.puzzle, sudoku.solution, player);

    loop {
        clear_screen()?;
        render_board(&mut game)?;

        // If game is solved or failed, show message and wait for quit
        if game.state == GameState::Solved {
            success_message()?;
            if let InputReturnType::Exit = handle_key_input(&mut game, read_key()?)? {
                break;
            }
            continue;
        } else if game.state == GameState::Failed {
            failure_message()?;
            match handle_key_input(&mut game, read_key()?)? {
                InputReturnType::Exit => break,
                _ => {
                    game.state = GameState::Playing;
                    continue;
                }
            }
        }

        // Check if game is over (max mistakes reached)
        if is_game_over(&game) {
            game.state = GameState::Failed;
            continue;
        }

        match handle_key_input(&mut game, read_key()?)? {
            InputReturnType::Exit => break (),
            InputReturnType::CheckPuzzle => {
                if is_solved(&mut game.board, &mut game.solution) {
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

pub fn difficulty_selector() -> Result<Difficulty, Box<dyn std::error::Error>> {
    let mut selected = 0;
    loop {
        clear_screen()?;
        render_difficulty_selector(selected)?;
        match read_key()? {
            KeyCode::Char('w') => selected = (selected - 1).max(0),
            KeyCode::Char('s') => selected = (selected + 1).min(Difficulty::all().len() - 1),
            KeyCode::Enter => break,
            _ => continue,
        }
    }
    Ok(Difficulty::all()[selected])
}

pub fn selected_number(game: &Game) -> Option<u8> {
    game.board.cells[game.player.row][game.player.col].value()
}

pub fn is_game_over(game: &Game) -> bool {
    if let Some(max_mistakes) = game.rules.max_mistakes {
        if game.mistakes >= max_mistakes {
            return true;
        }
    }
    false
}
