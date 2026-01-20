use crate::eval_fns::EvalFn;
use crate::game::Board;

pub struct PileHeight;

impl EvalFn for PileHeight {
    fn eval(&self, board: &Board) -> u8 {
        for (i, row) in board.cells.iter().rev().enumerate() {
            if row.iter().any(|&cell| cell) {
                return 19 - i as u8;
            }
        }
        20
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Board;

    #[test]
    fn test_pile_height_empty_board() {
        let board = Board {
            cells: [[false; 10]; 20],
        };
        let ef = PileHeight;
        assert_eq!(ef.eval(&board), 20);
    }

    #[test]
    fn test_pile_height_partial_board() {
        let mut cells = [[false; 10]; 20];
        cells[18][0] = true; // Row 18 has a block
        let board = Board { cells };
        let ef = PileHeight;
        assert_eq!(ef.eval(&board), 3);
    }
}
