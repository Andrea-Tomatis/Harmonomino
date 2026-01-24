use rand::Rng;

/// The 7 standard Tetris pieces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tetromino {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

/// Rotation state (0-3, representing 0°, 90°, 180°, 270° clockwise).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rotation(pub u8);

impl Rotation {
    #[must_use]
    pub const fn clockwise(self) -> Self {
        Self((self.0 + 1) % 4)
    }

    #[must_use]
    pub const fn counter_clockwise(self) -> Self {
        Self((self.0 + 3) % 4)
    }
}

/// A piece with position and rotation state.
#[derive(Debug, Clone, Copy)]
pub struct FallingPiece {
    pub tetromino: Tetromino,
    pub rotation: Rotation,
    /// Position of the piece's origin (col, row). Can be negative during wall kicks.
    pub col: i8,
    pub row: i8,
}

impl FallingPiece {
    /// Creates a new piece at the spawn position.
    #[must_use]
    pub const fn spawn(tetromino: Tetromino) -> Self {
        // Spawn in the top-center of the board (row 18-19 area)
        // Standard spawn: piece appears with bottom at row 19/20
        let (col, row) = tetromino.spawn_position();
        Self {
            tetromino,
            rotation: Rotation(0),
            col,
            row,
        }
    }

    /// Returns the absolute cell positions for this piece.
    #[must_use]
    pub fn cells(self) -> [(i8, i8); 4] {
        self.tetromino
            .cells(self.rotation)
            .map(|(dc, dr)| (self.col + dc, self.row + dr))
    }

    /// Returns a copy moved by the given offset.
    #[must_use]
    pub const fn moved(self, dcol: i8, drow: i8) -> Self {
        Self {
            col: self.col + dcol,
            row: self.row + drow,
            tetromino: self.tetromino,
            rotation: self.rotation,
        }
    }

    /// Returns a copy rotated clockwise.
    #[must_use]
    pub const fn rotated_cw(self) -> Self {
        Self {
            rotation: self.rotation.clockwise(),
            tetromino: self.tetromino,
            col: self.col,
            row: self.row,
        }
    }

    /// Returns a copy rotated counter-clockwise.
    #[must_use]
    pub const fn rotated_ccw(self) -> Self {
        Self {
            rotation: self.rotation.counter_clockwise(),
            tetromino: self.tetromino,
            col: self.col,
            row: self.row,
        }
    }
}

impl Tetromino {
    /// All tetromino variants for random selection.
    pub const ALL: [Self; 7] = [
        Self::I,
        Self::O,
        Self::T,
        Self::S,
        Self::Z,
        Self::J,
        Self::L,
    ];

    /// Returns a random tetromino.
    #[must_use]
    pub fn random() -> Self {
        let mut rng = rand::rng();
        Self::ALL[rng.random_range(0..Self::ALL.len())]
    }

    /// Returns the spawn position (col, row) for this piece.
    /// Pieces spawn at the top-center of the 10-wide board.
    /// Position is chosen so all cells fit within the 20-row board.
    #[must_use]
    pub const fn spawn_position(self) -> (i8, i8) {
        // Center horizontally, spawn row based on piece height in rotation 0
        match self {
            // I piece: 4 wide, cells at row offset 1 → spawn at row 18
            Self::I => (3, 18),
            // O piece: 2x2, cells at row offset 0-1 → spawn at row 18, offset right
            Self::O => (4, 18),
            // T, S, Z, J, L: 3 wide, cells at row offset 1-2 → spawn at row 17
            _ => (3, 17),
        }
    }

    /// Returns the relative cell positions for this piece at a given rotation.
    /// Coordinates are `(col_offset, row_offset)` from the piece origin.
    /// Uses SRS (Super Rotation System) positioning from lookup tables.
    #[must_use]
    pub const fn cells(self, rotation: Rotation) -> [(i8, i8); 4] {
        self.rotation_cells(rotation.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn falling_piece_movement() {
        let piece = FallingPiece::spawn(Tetromino::T);
        let moved = piece.moved(1, -1);
        assert_eq!(moved.col, piece.col + 1);
        assert_eq!(moved.row, piece.row - 1);
    }

    #[test]
    fn rotation_state_cycle() {
        let r = Rotation(0);
        assert_eq!(r.clockwise().clockwise().clockwise().clockwise(), r);
        assert_eq!(r.counter_clockwise(), Rotation(3));
    }
}
