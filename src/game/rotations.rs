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
