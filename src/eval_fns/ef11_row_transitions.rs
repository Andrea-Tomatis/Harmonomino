use crate::eval_fns::EvalFn;
use crate::game::Board;

/// The sum of all horizontal transitions between occupied and unoccupied cells.
/// Walls count as occupied, so an empty cell at the edge counts as a transition.
pub struct RowTransitions;

impl EvalFn for RowTransitions {
    fn eval(&self, board: &Board) -> u8 {
        let mut transitions = 0;

        for row in 0..Board::HEIGHT {
            // Left wall to first cell
            if !board[row][0] {
                transitions += 1;
            }

            // Transitions within the row
            for col in 0..Board::WIDTH - 1 {
                if board[row][col] != board[row][col + 1] {
                    transitions += 1;
                }
            }

            // Last cell to right wall
            if !board[row][Board::WIDTH - 1] {
                transitions += 1;
            }
        }

        transitions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Board;

    const EF: &dyn EvalFn = &RowTransitions;

    #[test]
    fn test_empty_board() {
        let board = Board::new();
        // Each row: left wall->empty (1) + empty->right wall (1) = 2 per row
        // 20 rows * 2 = 40
        assert_eq!(EF.eval(&board), 40);
    }

    #[test]
    fn test_full_row() {
        let mut board = Board::new();
        // Fill one entire row
        for col in 0..Board::WIDTH {
            board[0][col] = true;
        }
        // Row 0: no transitions (wall-filled-...-filled-wall)
        // Other 19 rows: 2 each = 38
        assert_eq!(EF.eval(&board), 38);
    }

    #[test]
    fn test_alternating_row() {
        let mut board = Board::new();
        // Alternating pattern in row 0: filled, empty, filled, empty...
        for col in 0..Board::WIDTH {
            board[0][col] = col % 2 == 0;
        }
        // Row 0: wall->filled(0) + 9 internal transitions + empty->wall(1) = 10
        // Other 19 rows: 2 each = 38
        // Total = 10 + 38 = 48
        assert_eq!(EF.eval(&board), 48);
    }
}
