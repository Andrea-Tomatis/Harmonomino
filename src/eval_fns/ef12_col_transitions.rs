use crate::eval_fns::EvalFn;
use crate::game::Board;

/// The sum of all vertical transitions between occupied and unoccupied cells.
/// The floor counts as occupied, so an empty cell at the bottom counts as a transition.
pub struct ColTransitions;

impl EvalFn for ColTransitions {
    fn eval(&self, board: &Board) -> u8 {
        let mut transitions = 0;

        for col in 0..Board::WIDTH {
            // Floor to bottom cell (floor counts as occupied)
            if !board[0][col] {
                transitions += 1;
            }

            // Transitions within the column
            for row in 0..Board::HEIGHT - 1 {
                if board[row][col] != board[row + 1][col] {
                    transitions += 1;
                }
            }

            // Top cell to ceiling (ceiling counts as empty, so transition only if top cell is filled)
            // unless we don't want to count it, unclear based on paper, purposefully untested
            transitions += u8::from(board[Board::HEIGHT - 1][col]);
        }

        transitions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Board;

    const EF: &dyn EvalFn = &ColTransitions;

    #[test]
    fn test_empty_board() {
        let board = Board::new();
        // Each column: floor->empty = 1 transition
        // 10 columns * 1 = 10
        assert_eq!(EF.eval(&board), 10);
    }

    #[test]
    fn test_full_bottom_row() {
        let mut board = Board::new();
        for col in 0..Board::WIDTH {
            board[0][col] = true;
        }
        // Each column: floor->filled (0) + filled->empty (1) = 1
        // 10 columns * 1 = 10
        assert_eq!(EF.eval(&board), 10);
    }

    #[test]
    fn test_single_column_stack() {
        let mut board = Board::new();
        // Stack 5 blocks in column 0
        for row in 0..5 {
            board[row][0] = true;
        }
        // Col 0: floor->filled(0) + filled->empty at row 5 (1) = 1
        // Other 9 cols: floor->empty (1) each = 9
        // Total = 1 + 9 = 10
        assert_eq!(EF.eval(&board), 10);
    }

    #[test]
    fn test_gap_in_column() {
        let mut board = Board::new();
        // Column 0: filled at 0, empty at 1, filled at 2
        board[0][0] = true;
        board[2][0] = true;
        // Col 0: floor->filled(0) + filled->empty(1) + empty->filled(1) + filled->empty(1) = 3
        // Other 9 cols: 1 each = 9
        // Total = 3 + 9 = 12
        assert_eq!(EF.eval(&board), 12);
    }
}
