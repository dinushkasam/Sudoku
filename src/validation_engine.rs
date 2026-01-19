// Row, Column Box checks
// Conflict detection
use crate::objects::Board;

pub fn is_valid(board: &Board, row: usize, col: usize, num: u8) -> bool {
    // Row check
    for c in 0..9 {
        if let Some(value) = board.cells[row][c].value() {
            if value == num {
                return false;
            }
        }
        // If there is no value, continue
    }

    // Column check
    for r in 0..9 {
        if let Some(value) = board.cells[r][col].value() {
            if value == num {
                return false;
            }
        }
    }

    // 3x3 box check
    let box_row = (row / 3) * 3;
    let box_col = (col / 3) * 3;

    for r in box_row..box_row + 3 {
        for c in box_col..box_col + 3 {
            if let Some(value) = board.cells[r][c].value() {
                if value == num {
                    return false;
                }
            }
        }
    }

    true
}
