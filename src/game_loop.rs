use crate::{
    objects::{Board, Game, GameState, InputReturnType, TerminalGuard},
    terminal::{
        clear_screen, failure_message, handle_key_input, read_key, render_board, restore_terminal,
        setup_terminal, success_message,
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
