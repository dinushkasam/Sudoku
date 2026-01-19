use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{Write, stdout};

use crate::{
    game_loop::selected_number,
    helpers::{apply_move, undo_move},
    objects::{Cell, Difficulty, Game, InputReturnType},
    validation_engine::is_cell_invalid,
};

pub fn setup_terminal() -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;
    Ok(())
}

pub fn restore_terminal() -> Result<(), Box<dyn std::error::Error>> {
    execute!(
        stdout(),
        terminal::LeaveAlternateScreen,
        cursor::Show,
        ResetColor
    )?;
    terminal::disable_raw_mode()?;
    Ok(())
}

pub fn read_key() -> Result<KeyCode, Box<dyn std::error::Error>> {
    loop {
        if let Event::Key(event) = event::read()? {
            return Ok(event.code);
        }
    }
}

pub fn clear_screen() -> Result<(), Box<dyn std::error::Error>> {
    execute!(stdout(), Clear(ClearType::All))?;
    Ok(())
}

pub fn render_board(game: &mut Game) -> Result<(), Box<dyn std::error::Error>> {
    let mut out = stdout();
    let mut line = 0u16;

    // Print difficulty at line 0
    execute!(out, cursor::MoveTo(0, line))?;
    execute!(out, Clear(ClearType::UntilNewLine))?;
    writeln!(
        out,
        "Difficulty: {:?} | Mistakes: {}/{} | Undos: {}/{}",
        game.difficulty,
        game.mistakes,
        game.rules.max_mistakes.unwrap_or(0),
        game.undos,
        game.rules.max_undos.unwrap_or(0)
    )?;
    line += 1;

    // Print top border at line 1
    execute!(out, cursor::MoveTo(0, line))?;
    execute!(out, Clear(ClearType::UntilNewLine))?;
    writeln!(out, "+-------+-------+-------+")?;
    line += 1;

    // Print board rows starting from line 2
    for r in 0..9 {
        if r % 3 == 0 && r != 0 {
            execute!(out, cursor::MoveTo(0, line))?;
            execute!(out, Clear(ClearType::UntilNewLine))?;
            writeln!(out, "|-------+-------+-------|")?;
            line += 1;
        }

        execute!(out, cursor::MoveTo(0, line))?;
        execute!(out, Clear(ClearType::UntilNewLine))?;
        write!(out, "| ")?;

        for c in 0..9 {
            if c % 3 == 0 && c != 0 {
                write!(out, "| ")?;
            }

            let color = cell_color(game, r, c, selected_number(game))?;
            execute!(out, SetForegroundColor(color))?;

            let is_cursor = game.player.row == r && game.player.col == c;

            match game.board.cells[r][c] {
                Cell::Given(n) | Cell::Filled(n) => {
                    write!(out, "{} ", n)?;
                }
                Cell::Empty => {
                    if is_cursor {
                        write!(out, "@ ")?;
                    } else {
                        write!(out, ". ")?;
                    }
                }
            }
            execute!(out, ResetColor)?;
        }
        writeln!(out, "|")?;
        line += 1;
    }

    // Print bottom border
    execute!(out, cursor::MoveTo(0, line))?;
    execute!(out, Clear(ClearType::UntilNewLine))?;
    writeln!(out, "+-------+-------+-------+")?;
    Ok(())
}

pub fn handle_key_input(
    game: &mut Game,
    key: KeyCode,
) -> Result<InputReturnType, Box<dyn std::error::Error>> {
    match key {
        KeyCode::Char('q') => Ok(InputReturnType::Exit),
        KeyCode::Char('w') => {
            if game.player.row == 0 {
                game.player.row = 8;
            } else {
                game.player.row -= 1;
            }
            Ok(InputReturnType::Continue)
        }
        KeyCode::Char('s') => {
            if game.player.row == 8 {
                game.player.row = 0;
            } else {
                game.player.row += 1;
            }
            Ok(InputReturnType::Continue)
        }
        KeyCode::Char('a') => {
            if game.player.col == 0 {
                game.player.col = 8;
            } else {
                game.player.col -= 1;
            }
            Ok(InputReturnType::Continue)
        }
        KeyCode::Char('d') => {
            if game.player.col == 8 {
                game.player.col = 0;
            } else {
                game.player.col += 1;
            }
            Ok(InputReturnType::Continue)
        }
        KeyCode::Char(c @ '1'..='9') => {
            let num = c.to_digit(10).ok_or("Invalid number")?;
            let r = game.player.row;
            let c = game.player.col;

            // Prevent editing givens
            if matches!(
                game.board.cells[game.player.row][game.player.col],
                Cell::Given(_)
            ) {
                return Ok(InputReturnType::Continue);
            }

            apply_move(game, r, c, Cell::Filled(num as u8));

            Ok(InputReturnType::Continue)
        }
        KeyCode::Char('0') => {
            let r = game.player.row;
            let c = game.player.col;

            if matches!(game.board.cells[r][c], Cell::Given(_)) {
                return Ok(InputReturnType::Continue);
            }

            apply_move(game, r, c, Cell::Empty);
            Ok(InputReturnType::Continue)
        }
        KeyCode::Char('z') => {
            undo_move(game)?;
            Ok(InputReturnType::Continue)
        }
        KeyCode::Enter | KeyCode::Char('\n') => Ok(InputReturnType::CheckPuzzle),
        _ => Ok(InputReturnType::Continue),
    }
}

pub fn success_message() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = stdout();
    execute!(out, cursor::MoveTo(0, 15))?;
    writeln!(out, "You solved the puzzle! Press 'q' to quit.")?;
    out.flush()?;
    Ok(())
}

pub fn failure_message(game: &Game) -> Result<(), Box<dyn std::error::Error>> {
    let mut out = stdout();
    execute!(out, cursor::MoveTo(0, 15))?;
    writeln!(out, "💀 Game Over! Press 'q' to quit or continue playing.")?;
    execute!(out, cursor::MoveTo(0, 16))?;
    writeln!(
        out,
        "Mistakes: {}/{}",
        game.mistakes,
        game.rules.max_mistakes.unwrap_or(0)
    )?;
    out.flush()?;
    Ok(())
}

pub fn render_difficulty_selector(selected: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut out = stdout();
    execute!(out, cursor::MoveTo(0, 0))?;
    writeln!(out, "Select Difficulty:\n")?;

    let mut line = 1u16;
    for (index, difficulty) in Difficulty::all().iter().enumerate() {
        execute!(out, cursor::MoveTo(0, line))?;
        execute!(out, Clear(ClearType::UntilNewLine))?;
        if selected == index {
            execute!(out, SetForegroundColor(Color::Yellow))?;
            writeln!(out, "> {}", difficulty.name())?;
        } else {
            execute!(out, ResetColor)?;
            writeln!(out, "- {}", difficulty.name())?;
        }
        line += 1;
    }
    execute!(out, ResetColor)?;
    execute!(out, cursor::MoveTo(0, line))?;
    execute!(out, Clear(ClearType::UntilNewLine))?;
    writeln!(out, "\nUse W/S and Enter to select difficulty")?;
    out.flush()?;
    Ok(())
}

fn cell_color(
    game: &mut Game,
    row: usize,
    col: usize,
    selected: Option<u8>,
) -> Result<Color, Box<dyn std::error::Error>> {
    if game.player.row == row && game.player.col == col {
        return Ok(Color::Yellow);
    }

    if game.rules.mistake_highlight {
        if is_cell_invalid(&mut game.board, row, col) {
            return Ok(Color::Red);
        }
    }

    if let Some(num) = selected {
        if game.board.cells[row][col].value() == Some(num) {
            return Ok(Color::Green);
        }
    }

    match game.board.cells[row][col] {
        Cell::Given(_) => Ok(Color::Blue),
        Cell::Filled(_) => Ok(Color::White),
        Cell::Empty => Ok(Color::DarkGrey),
    }
}
