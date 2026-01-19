use crate::objects::{Cell, Game, Move};

pub fn apply_move(game: &mut Game, row: usize, col: usize, new: Cell) {
    let previous = game.board.cells[row][col];

    if previous == new {
        return;
    }

    let mut mistake_delta = 0;

    if let Cell::Filled(n) = new {
        if let Some(solution_value) = game.solution.cells[row][col].value() {
            if solution_value != n {
                mistake_delta = 1;
            }
        }
    }

    game.board.cells[row][col] = new;
    game.mistakes = (game.mistakes as isize + mistake_delta) as usize;
    game.history.push(Move {
        row,
        col,
        previous,
        new,
        mistake_delta,
    });
}

pub fn undo_move(game: &mut Game) -> Result<(), Box<dyn std::error::Error>> {
    if game.history.is_empty() {
        return Err("No moves to undo".into());
    }

    if game.undos >= game.rules.max_undos.unwrap_or(0) {
        return Err("Max undos reached".into());
    }

    let last_move = game.history.pop().ok_or("No moves to undo")?;
    game.board.cells[last_move.row][last_move.col] = last_move.previous;
    game.mistakes = (game.mistakes as isize - last_move.mistake_delta).max(0) as usize;
    game.undos += 1;
    Ok(())
}
