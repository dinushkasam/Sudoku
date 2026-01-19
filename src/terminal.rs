use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{Write, stdout};

use crate::objects::{Cell, Game, InputReturnType};

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

    for r in 0..9 {
        if r % 3 == 0 && r != 0 {
            execute!(out, cursor::MoveTo(0, line))?;
            execute!(out, Clear(ClearType::UntilNewLine))?;
            writeln!(out, "------+-------+------")?;
            line += 1;
        }

        execute!(out, cursor::MoveTo(0, line))?;
        execute!(out, Clear(ClearType::UntilNewLine))?;

        for c in 0..9 {
            if c % 3 == 0 && c != 0 {
                write!(out, "| ")?;
            }

            let is_cursor = game.player.row == r && game.player.col == c;

            if is_cursor {
                execute!(out, SetForegroundColor(Color::Yellow))?;
            }

            match game.board.cells[r][c] {
                Cell::Given(n) => {
                    if !is_cursor {
                        execute!(out, SetForegroundColor(Color::Blue))?;
                    }
                    write!(out, "{} ", n)?;
                }
                Cell::Filled(n) => {
                    if !is_cursor {
                        execute!(out, SetForegroundColor(Color::White))?;
                    }
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
        line += 1;
    }
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

            if matches!(game.board.cells[r][c], Cell::Empty | Cell::Filled(_)) {
                game.board.cells[r][c] = Cell::Filled(num as u8);
                Ok(InputReturnType::Continue)
            } else {
                Err("Cannot edit given cells".into())
            }
        }
        KeyCode::Char('0') => {
            let r = game.player.row;
            let c = game.player.col;
            game.board.cells[r][c] = Cell::Empty;
            Ok(InputReturnType::Continue)
        }
        KeyCode::Enter | KeyCode::Char('\n') => Ok(InputReturnType::CheckPuzzle),
        _ => Ok(InputReturnType::Continue),
    }
}

pub fn success_message() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = stdout();
    execute!(out, cursor::MoveTo(0, 12))?;
    writeln!(out, "You solved the puzzle! Press 'q' to quit.")?;
    out.flush()?;
    Ok(())
}

pub fn failure_message() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = stdout();
    execute!(out, cursor::MoveTo(0, 12))?;
    writeln!(
        out,
        "Incorrect solution. Press 'q' to quit or continue playing."
    )?;
    out.flush()?;
    Ok(())
}
