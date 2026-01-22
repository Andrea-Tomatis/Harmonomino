//! Tests for tetromino rotation data.
//!
//! Each test verifies that the rotation produces the expected shape
//! by checking exact cell coordinates.

use harmonomino::game::{Rotation, Tetromino};
use std::collections::HashSet;

/// Helper to convert cells to a set for order-independent comparison.
fn cell_set(cells: [(i8, i8); 4]) -> HashSet<(i8, i8)> {
    cells.into_iter().collect()
}

/// Helper to visualize a piece for debugging.
#[allow(dead_code)]
fn visualize(cells: [(i8, i8); 4]) -> String {
    let min_col = cells.iter().map(|(c, _)| *c).min().unwrap_or(0);
    let max_col = cells.iter().map(|(c, _)| *c).max().unwrap_or(0);
    let min_row = cells.iter().map(|(_, r)| *r).min().unwrap_or(0);
    let max_row = cells.iter().map(|(_, r)| *r).max().unwrap_or(0);

    let mut result = String::new();
    for row in (min_row..=max_row).rev() {
        for col in min_col..=max_col {
            if cells.contains(&(col, row)) {
                result.push('█');
            } else {
                result.push('.');
            }
        }
        result.push('\n');
    }
    result
}

// =============================================================================
// I PIECE TESTS
// =============================================================================

#[test]
fn i_piece_rotation_0_horizontal() {
    // ████ (row 1 in 4x4 box)
    let cells = Tetromino::I.cells(Rotation(0));
    assert_eq!(cell_set(cells), cell_set([(0, 1), (1, 1), (2, 1), (3, 1)]));
}

#[test]
fn i_piece_rotation_1_vertical() {
    // Vertical in column 2
    let cells = Tetromino::I.cells(Rotation(1));
    assert_eq!(cell_set(cells), cell_set([(2, 0), (2, 1), (2, 2), (2, 3)]));
}

#[test]
fn i_piece_rotation_2_horizontal() {
    // ████ (row 2 in 4x4 box)
    let cells = Tetromino::I.cells(Rotation(2));
    assert_eq!(cell_set(cells), cell_set([(0, 2), (1, 2), (2, 2), (3, 2)]));
}

#[test]
fn i_piece_rotation_3_vertical() {
    // Vertical in column 1
    let cells = Tetromino::I.cells(Rotation(3));
    assert_eq!(cell_set(cells), cell_set([(1, 0), (1, 1), (1, 2), (1, 3)]));
}

// =============================================================================
// O PIECE TESTS
// =============================================================================

#[test]
fn o_piece_all_rotations_identical() {
    // ██
    // ██
    let expected = cell_set([(0, 0), (1, 0), (0, 1), (1, 1)]);
    for rot in 0..4 {
        let cells = Tetromino::O.cells(Rotation(rot));
        assert_eq!(cell_set(cells), expected, "O piece rotation {rot} differs");
    }
}

// =============================================================================
// T PIECE TESTS
// =============================================================================

#[test]
fn t_piece_rotation_0_pointing_up() {
    // .█.
    // ███
    let cells = Tetromino::T.cells(Rotation(0));
    assert_eq!(cell_set(cells), cell_set([(0, 1), (1, 1), (2, 1), (1, 2)]));
}

#[test]
fn t_piece_rotation_1_pointing_right() {
    // █.
    // ██
    // █.
    let cells = Tetromino::T.cells(Rotation(1));
    assert_eq!(cell_set(cells), cell_set([(0, 0), (0, 1), (0, 2), (1, 1)]));
}

#[test]
fn t_piece_rotation_2_pointing_down() {
    // ███
    // .█.
    let cells = Tetromino::T.cells(Rotation(2));
    assert_eq!(cell_set(cells), cell_set([(0, 1), (1, 1), (2, 1), (1, 0)]));
}

#[test]
fn t_piece_rotation_3_pointing_left() {
    // .█
    // ██
    // .█
    let cells = Tetromino::T.cells(Rotation(3));
    assert_eq!(cell_set(cells), cell_set([(1, 0), (1, 1), (1, 2), (0, 1)]));
}

// =============================================================================
// S PIECE TESTS
// =============================================================================

#[test]
fn s_piece_rotation_0_horizontal() {
    // .██
    // ██.
    let cells = Tetromino::S.cells(Rotation(0));
    assert_eq!(cell_set(cells), cell_set([(0, 1), (1, 1), (1, 2), (2, 2)]));
}

#[test]
fn s_piece_rotation_1_vertical() {
    // █.
    // ██
    // .█
    let cells = Tetromino::S.cells(Rotation(1));
    assert_eq!(cell_set(cells), cell_set([(0, 1), (0, 2), (1, 0), (1, 1)]));
}

#[test]
fn s_piece_rotations_2_and_0_match() {
    let r0 = Tetromino::S.cells(Rotation(0));
    let r2 = Tetromino::S.cells(Rotation(2));
    assert_eq!(cell_set(r0), cell_set(r2));
}

#[test]
fn s_piece_rotations_3_and_1_match() {
    let r1 = Tetromino::S.cells(Rotation(1));
    let r3 = Tetromino::S.cells(Rotation(3));
    assert_eq!(cell_set(r1), cell_set(r3));
}

// =============================================================================
// Z PIECE TESTS
// =============================================================================

#[test]
fn z_piece_rotation_0_horizontal() {
    // ██.
    // .██
    let cells = Tetromino::Z.cells(Rotation(0));
    assert_eq!(cell_set(cells), cell_set([(0, 2), (1, 2), (1, 1), (2, 1)]));
}

#[test]
fn z_piece_rotation_1_vertical() {
    // .█
    // ██
    // █.
    let cells = Tetromino::Z.cells(Rotation(1));
    assert_eq!(cell_set(cells), cell_set([(0, 0), (0, 1), (1, 1), (1, 2)]));
}

#[test]
fn z_piece_rotations_2_and_0_match() {
    let r0 = Tetromino::Z.cells(Rotation(0));
    let r2 = Tetromino::Z.cells(Rotation(2));
    assert_eq!(cell_set(r0), cell_set(r2));
}

#[test]
fn z_piece_rotations_3_and_1_match() {
    let r1 = Tetromino::Z.cells(Rotation(1));
    let r3 = Tetromino::Z.cells(Rotation(3));
    assert_eq!(cell_set(r1), cell_set(r3));
}

// =============================================================================
// J PIECE TESTS
// =============================================================================

#[test]
fn j_piece_rotation_0() {
    // █..
    // ███
    let cells = Tetromino::J.cells(Rotation(0));
    assert_eq!(cell_set(cells), cell_set([(0, 1), (1, 1), (2, 1), (0, 2)]));
}

#[test]
fn j_piece_rotation_1() {
    // ██
    // █.
    // █.
    let cells = Tetromino::J.cells(Rotation(1));
    assert_eq!(cell_set(cells), cell_set([(0, 0), (0, 1), (0, 2), (1, 2)]));
}

#[test]
fn j_piece_rotation_2() {
    // ███
    // ..█
    let cells = Tetromino::J.cells(Rotation(2));
    assert_eq!(cell_set(cells), cell_set([(0, 1), (1, 1), (2, 1), (2, 0)]));
}

#[test]
fn j_piece_rotation_3() {
    // .█
    // .█
    // ██
    let cells = Tetromino::J.cells(Rotation(3));
    assert_eq!(cell_set(cells), cell_set([(0, 0), (1, 0), (1, 1), (1, 2)]));
}

// =============================================================================
// L PIECE TESTS
// =============================================================================

#[test]
fn l_piece_rotation_0() {
    // ..█
    // ███
    let cells = Tetromino::L.cells(Rotation(0));
    assert_eq!(cell_set(cells), cell_set([(0, 1), (1, 1), (2, 1), (2, 2)]));
}

#[test]
fn l_piece_rotation_1() {
    // █.
    // █.
    // ██
    let cells = Tetromino::L.cells(Rotation(1));
    assert_eq!(cell_set(cells), cell_set([(0, 0), (0, 1), (0, 2), (1, 0)]));
}

#[test]
fn l_piece_rotation_2() {
    // ███
    // █..
    let cells = Tetromino::L.cells(Rotation(2));
    assert_eq!(cell_set(cells), cell_set([(0, 1), (1, 1), (2, 1), (0, 0)]));
}

#[test]
fn l_piece_rotation_3() {
    // ██
    // .█
    // .█
    let cells = Tetromino::L.cells(Rotation(3));
    assert_eq!(cell_set(cells), cell_set([(1, 0), (1, 1), (1, 2), (0, 2)]));
}

// =============================================================================
// GENERAL PROPERTIES
// =============================================================================

#[test]
fn all_pieces_have_4_cells() {
    for piece in Tetromino::ALL {
        for rot in 0..4 {
            let cells = piece.cells(Rotation(rot));
            let unique: HashSet<_> = cells.into_iter().collect();
            assert_eq!(
                unique.len(),
                4,
                "{piece:?} rotation {rot} has {} unique cells, expected 4",
                unique.len()
            );
        }
    }
}

#[test]
fn rotation_cycle_returns_to_start() {
    for piece in Tetromino::ALL {
        let start = Tetromino::cells(piece, Rotation(0));
        let after_4 = Tetromino::cells(piece, Rotation(4));
        assert_eq!(
            cell_set(start),
            cell_set(after_4),
            "{piece:?} rotation 4 should equal rotation 0"
        );
    }
}

#[test]
fn cells_are_connected() {
    // Each piece should form a connected shape (each cell adjacent to at least one other)
    for piece in Tetromino::ALL {
        for rot in 0..4 {
            let cells = piece.cells(Rotation(rot));
            let set: HashSet<_> = cells.into_iter().collect();

            for &(col, row) in &set {
                let neighbors = [
                    (col - 1, row),
                    (col + 1, row),
                    (col, row - 1),
                    (col, row + 1),
                ];
                let has_neighbor = neighbors.iter().any(|n| set.contains(n));
                assert!(
                    has_neighbor,
                    "{piece:?} rotation {rot}: cell ({col}, {row}) has no adjacent cells"
                );
            }
        }
    }
}
