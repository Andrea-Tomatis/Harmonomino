//! SRS (Super Rotation System) rotation data for all tetrominoes.
//!
//! Each piece has 4 rotation states (0=spawn, 1=CW, 2=180°, 3=CCW).
//! Coordinates are `(col, row)` offsets from piece origin, where row 0 is bottom.
//!
//! Visual reference for each piece at rotation state 0:
//!
//! ```text
//! I: ████      O: ██      T:  █       S:  ██      Z: ██       J: █        L:   █
//!                 ██         ███         ██           ██         ███         ███
//! ```

use super::Tetromino;

/// I piece rotations in a 4x4 bounding box.
/// ```text
/// Rot 0:  Rot 1:  Rot 2:  Rot 3:
/// ....    ..█.    ....    .█..
/// ████    ..█.    ....    .█..
/// ....    ..█.    ████    .█..
/// ....    ..█.    ....    .█..
/// ```
const I: [[(i8, i8); 4]; 4] = [
    [(0, 1), (1, 1), (2, 1), (3, 1)], // 0: horizontal, row 1
    [(2, 0), (2, 1), (2, 2), (2, 3)], // 1: vertical, col 2
    [(0, 2), (1, 2), (2, 2), (3, 2)], // 2: horizontal, row 2
    [(1, 0), (1, 1), (1, 2), (1, 3)], // 3: vertical, col 1
];

/// O piece rotations (identical in all states).
/// ```text
/// ██
/// ██
/// ```
const O: [[(i8, i8); 4]; 4] = [
    [(0, 0), (1, 0), (0, 1), (1, 1)],
    [(0, 0), (1, 0), (0, 1), (1, 1)],
    [(0, 0), (1, 0), (0, 1), (1, 1)],
    [(0, 0), (1, 0), (0, 1), (1, 1)],
];

/// T piece rotations in a 3x3 bounding box.
/// ```text
/// Rot 0:  Rot 1:  Rot 2:  Rot 3:
/// .█.     █..     ...     .█.
/// ███     ██.     ███     ██.
/// ...     █..     .█.     .█.
/// ```
const T: [[(i8, i8); 4]; 4] = [
    [(0, 1), (1, 1), (2, 1), (1, 2)], // 0: T pointing up
    [(0, 0), (0, 1), (0, 2), (1, 1)], // 1: T pointing right
    [(0, 1), (1, 1), (2, 1), (1, 0)], // 2: T pointing down
    [(1, 0), (1, 1), (1, 2), (0, 1)], // 3: T pointing left
];

/// S piece rotations.
/// ```text
/// Rot 0:  Rot 1:
/// .██     █.
/// ██.     ██
/// ...     .█
/// ```
const S: [[(i8, i8); 4]; 4] = [
    [(0, 1), (1, 1), (1, 2), (2, 2)], // 0: horizontal
    [(0, 1), (0, 2), (1, 0), (1, 1)], // 1: vertical
    [(0, 1), (1, 1), (1, 2), (2, 2)], // 2: same as 0
    [(0, 1), (0, 2), (1, 0), (1, 1)], // 3: same as 1
];

/// Z piece rotations.
/// ```text
/// Rot 0:  Rot 1:
/// ██.     .█
/// .██     ██
/// ...     █.
/// ```
const Z: [[(i8, i8); 4]; 4] = [
    [(0, 2), (1, 2), (1, 1), (2, 1)], // 0: horizontal
    [(0, 0), (0, 1), (1, 1), (1, 2)], // 1: vertical
    [(0, 2), (1, 2), (1, 1), (2, 1)], // 2: same as 0
    [(0, 0), (0, 1), (1, 1), (1, 2)], // 3: same as 1
];

/// J piece rotations.
/// ```text
/// Rot 0:  Rot 1:  Rot 2:  Rot 3:
/// █..     ██.     ...     .█.
/// ███     █..     ███     .█.
/// ...     █..     ..█     ██.
/// ```
const J: [[(i8, i8); 4]; 4] = [
    [(0, 1), (1, 1), (2, 1), (0, 2)], // 0: J pointing up-left
    [(0, 0), (0, 1), (0, 2), (1, 2)], // 1: J pointing up-right
    [(0, 1), (1, 1), (2, 1), (2, 0)], // 2: J pointing down-right
    [(0, 0), (1, 0), (1, 1), (1, 2)], // 3: J pointing down-left
];

/// L piece rotations.
/// ```text
/// Rot 0:  Rot 1:  Rot 2:  Rot 3:
/// ..█     █..     ...     ██.
/// ███     █..     ███     .█.
/// ...     ██.     █..     .█.
/// ```
const L: [[(i8, i8); 4]; 4] = [
    [(0, 1), (1, 1), (2, 1), (2, 2)], // 0: L pointing up-right
    [(0, 0), (0, 1), (0, 2), (1, 0)], // 1: L pointing down-right
    [(0, 1), (1, 1), (2, 1), (0, 0)], // 2: L pointing down-left
    [(1, 0), (1, 1), (1, 2), (0, 2)], // 3: L pointing up-left
];

impl Tetromino {
    /// Returns the cell offsets for this piece at the given rotation state.
    #[must_use]
    pub const fn rotation_cells(self, rotation: u8) -> [(i8, i8); 4] {
        let r = (rotation % 4) as usize;
        match self {
            Self::I => I[r],
            Self::O => O[r],
            Self::T => T[r],
            Self::S => S[r],
            Self::Z => Z[r],
            Self::J => J[r],
            Self::L => L[r],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Rotation;
    use super::Tetromino;
    use std::collections::HashSet;

    /// Helper to convert cells to a set for order-independent comparison.
    fn cell_set(cells: [(i8, i8); 4]) -> HashSet<(i8, i8)> {
        cells.into_iter().collect()
    }

    // =========================================================================
    // I PIECE
    // =========================================================================

    #[test]
    fn i_rotation_0_horizontal() {
        let cells = Tetromino::I.cells(Rotation(0));
        assert_eq!(cell_set(cells), cell_set([(0, 1), (1, 1), (2, 1), (3, 1)]));
    }

    #[test]
    fn i_rotation_1_vertical() {
        let cells = Tetromino::I.cells(Rotation(1));
        assert_eq!(cell_set(cells), cell_set([(2, 0), (2, 1), (2, 2), (2, 3)]));
    }

    #[test]
    fn i_rotation_2_horizontal() {
        let cells = Tetromino::I.cells(Rotation(2));
        assert_eq!(cell_set(cells), cell_set([(0, 2), (1, 2), (2, 2), (3, 2)]));
    }

    #[test]
    fn i_rotation_3_vertical() {
        let cells = Tetromino::I.cells(Rotation(3));
        assert_eq!(cell_set(cells), cell_set([(1, 0), (1, 1), (1, 2), (1, 3)]));
    }

    // =========================================================================
    // O PIECE
    // =========================================================================

    #[test]
    fn o_all_rotations_identical() {
        let expected = cell_set([(0, 0), (1, 0), (0, 1), (1, 1)]);
        for rot in 0..4 {
            let cells = Tetromino::O.cells(Rotation(rot));
            assert_eq!(cell_set(cells), expected, "O rotation {rot} differs");
        }
    }

    // =========================================================================
    // T PIECE
    // =========================================================================

    #[test]
    fn t_rotation_0_pointing_up() {
        let cells = Tetromino::T.cells(Rotation(0));
        assert_eq!(cell_set(cells), cell_set([(0, 1), (1, 1), (2, 1), (1, 2)]));
    }

    #[test]
    fn t_rotation_1_pointing_right() {
        let cells = Tetromino::T.cells(Rotation(1));
        assert_eq!(cell_set(cells), cell_set([(0, 0), (0, 1), (0, 2), (1, 1)]));
    }

    #[test]
    fn t_rotation_2_pointing_down() {
        let cells = Tetromino::T.cells(Rotation(2));
        assert_eq!(cell_set(cells), cell_set([(0, 1), (1, 1), (2, 1), (1, 0)]));
    }

    #[test]
    fn t_rotation_3_pointing_left() {
        let cells = Tetromino::T.cells(Rotation(3));
        assert_eq!(cell_set(cells), cell_set([(1, 0), (1, 1), (1, 2), (0, 1)]));
    }

    // =========================================================================
    // S PIECE
    // =========================================================================

    #[test]
    fn s_rotation_0_horizontal() {
        let cells = Tetromino::S.cells(Rotation(0));
        assert_eq!(cell_set(cells), cell_set([(0, 1), (1, 1), (1, 2), (2, 2)]));
    }

    #[test]
    fn s_rotation_1_vertical() {
        let cells = Tetromino::S.cells(Rotation(1));
        assert_eq!(cell_set(cells), cell_set([(0, 1), (0, 2), (1, 0), (1, 1)]));
    }

    #[test]
    fn s_rotations_symmetric() {
        assert_eq!(
            cell_set(Tetromino::S.cells(Rotation(0))),
            cell_set(Tetromino::S.cells(Rotation(2)))
        );
        assert_eq!(
            cell_set(Tetromino::S.cells(Rotation(1))),
            cell_set(Tetromino::S.cells(Rotation(3)))
        );
    }

    // =========================================================================
    // Z PIECE
    // =========================================================================

    #[test]
    fn z_rotation_0_horizontal() {
        let cells = Tetromino::Z.cells(Rotation(0));
        assert_eq!(cell_set(cells), cell_set([(0, 2), (1, 2), (1, 1), (2, 1)]));
    }

    #[test]
    fn z_rotation_1_vertical() {
        let cells = Tetromino::Z.cells(Rotation(1));
        assert_eq!(cell_set(cells), cell_set([(0, 0), (0, 1), (1, 1), (1, 2)]));
    }

    #[test]
    fn z_rotations_symmetric() {
        assert_eq!(
            cell_set(Tetromino::Z.cells(Rotation(0))),
            cell_set(Tetromino::Z.cells(Rotation(2)))
        );
        assert_eq!(
            cell_set(Tetromino::Z.cells(Rotation(1))),
            cell_set(Tetromino::Z.cells(Rotation(3)))
        );
    }

    // =========================================================================
    // J PIECE
    // =========================================================================

    #[test]
    fn j_rotation_0() {
        let cells = Tetromino::J.cells(Rotation(0));
        assert_eq!(cell_set(cells), cell_set([(0, 1), (1, 1), (2, 1), (0, 2)]));
    }

    #[test]
    fn j_rotation_1() {
        let cells = Tetromino::J.cells(Rotation(1));
        assert_eq!(cell_set(cells), cell_set([(0, 0), (0, 1), (0, 2), (1, 2)]));
    }

    #[test]
    fn j_rotation_2() {
        let cells = Tetromino::J.cells(Rotation(2));
        assert_eq!(cell_set(cells), cell_set([(0, 1), (1, 1), (2, 1), (2, 0)]));
    }

    #[test]
    fn j_rotation_3() {
        let cells = Tetromino::J.cells(Rotation(3));
        assert_eq!(cell_set(cells), cell_set([(0, 0), (1, 0), (1, 1), (1, 2)]));
    }

    // =========================================================================
    // L PIECE
    // =========================================================================

    #[test]
    fn l_rotation_0() {
        let cells = Tetromino::L.cells(Rotation(0));
        assert_eq!(cell_set(cells), cell_set([(0, 1), (1, 1), (2, 1), (2, 2)]));
    }

    #[test]
    fn l_rotation_1() {
        let cells = Tetromino::L.cells(Rotation(1));
        assert_eq!(cell_set(cells), cell_set([(0, 0), (0, 1), (0, 2), (1, 0)]));
    }

    #[test]
    fn l_rotation_2() {
        let cells = Tetromino::L.cells(Rotation(2));
        assert_eq!(cell_set(cells), cell_set([(0, 1), (1, 1), (2, 1), (0, 0)]));
    }

    #[test]
    fn l_rotation_3() {
        let cells = Tetromino::L.cells(Rotation(3));
        assert_eq!(cell_set(cells), cell_set([(1, 0), (1, 1), (1, 2), (0, 2)]));
    }

    // =========================================================================
    // GENERAL PROPERTIES
    // =========================================================================

    #[test]
    fn all_pieces_have_4_unique_cells() {
        for piece in Tetromino::ALL {
            for rot in 0..4 {
                let cells = piece.cells(Rotation(rot));
                let unique: HashSet<_> = cells.into_iter().collect();
                assert_eq!(
                    unique.len(),
                    4,
                    "{piece:?} rotation {rot} has {} unique cells",
                    unique.len()
                );
            }
        }
    }

    #[test]
    fn rotation_4_equals_rotation_0() {
        for piece in Tetromino::ALL {
            let r0 = piece.cells(Rotation(0));
            let r4 = piece.cells(Rotation(4));
            assert_eq!(cell_set(r0), cell_set(r4), "{piece:?} rotation 4 != 0");
        }
    }

    #[test]
    fn all_cells_are_connected() {
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
                    assert!(
                        neighbors.iter().any(|n| set.contains(n)),
                        "{piece:?} rotation {rot}: cell ({col}, {row}) is disconnected"
                    );
                }
            }
        }
    }
}
