// Row, Column Box checks
// Conflict detection
use crate::objects::Board;

pub fn is_valid(board: &Board, row: usize, col: usize, num: u8) -> bool {
    let mut nums = [0u8; 9];

    // Row check
    for r in 0..9 {
        for c in 0..9 {
            if let Some(value) = board.cells[r][c].value() {
                let index = (value - 1) as usize;
                if nums[index] > 1 {
                    return false;
                }
                nums[index] += 1;
            }
            // If there is no value, continue
        }
        // After a single row, clear the nums
        nums = [0u8; 9];
    }

    // Column check
    for c in 0..9 {
        for r in 0..9 {
            if let Some(value) = board.cells[r][c].value() {
                let index = (value - 1) as usize;
                if nums[index] > 1 {
                    return false;
                }
                nums[index] += 1;
            }
        }
        nums = [0u8; 9];
    }

    // 3x3 box check
    for horizontal_box in 0..3 {
        for vertical_box in 0..3 {
            for r in 0..3 {
                for c in 0..3 {
                    let h_multiplier = 3 * horizontal_box;
                    let v_multiplier = 3 * vertical_box;
                    if let Some(value) = board.cells[h_multiplier + r][v_multiplier + c].value() {
                        let index = (value - 1) as usize;
                        if nums[index] > 1 {
                            return false;
                        }
                        nums[index] += 1;
                    }
                }
            }
        }
    }

    true
}
