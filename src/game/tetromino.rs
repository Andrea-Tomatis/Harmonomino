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
        let mut rng = rand::thread_rng();
        Self::ALL[rng.gen_range(0..Self::ALL.len())]
    }

    /// Returns the spawn position (col, row) for this piece.
    /// Pieces spawn at the top-center of the 10-wide board.
    #[must_use]
    pub const fn spawn_position(self) -> (i8, i8) {
        // Center horizontally (col 3-4 area), high up on board
        match self {
            Self::O => (4, 18), // O piece is 2 wide, spawn 1 col right
            _ => (3, 18),       // All others (including 4-wide I) start at col 3
        }
    }

    /// Returns the relative cell positions for this piece at a given rotation.
    /// Coordinates are `(col_offset, row_offset)` from the piece origin.
    /// Uses SRS (Super Rotation System) style positioning.
    #[must_use]
    pub fn cells(self, rotation: Rotation) -> [(i8, i8); 4] {
        // Base shapes (rotation 0)
        // Row 0 is bottom of piece, positive row is up
        let base: [(i8, i8); 4] = match self {
            // XXXX (I piece horizontal)
            Self::I => [(0, 0), (1, 0), (2, 0), (3, 0)],

            // XX
            // XX (O piece - same in all rotations)
            Self::O => [(0, 0), (1, 0), (0, 1), (1, 1)],

            //  X
            // XXX (T piece)
            Self::T => [(0, 0), (1, 0), (2, 0), (1, 1)],

            //  XX
            // XX  (S piece)
            Self::S => [(0, 0), (1, 0), (1, 1), (2, 1)],

            // XX
            //  XX (Z piece)
            Self::Z => [(1, 0), (2, 0), (0, 1), (1, 1)],

            // X
            // XXX (J piece)
            Self::J => [(0, 0), (1, 0), (2, 0), (0, 1)],

            //   X
            // XXX (L piece)
            Self::L => [(0, 0), (1, 0), (2, 0), (2, 1)],
        };

        // O piece doesn't rotate
        if self == Self::O {
            return base;
        }

        // Apply rotation
        // For rotation, we rotate around a center point
        // Clockwise rotation: (x, y) -> (y, -x) relative to center
        let mut result = base;
        for _ in 0..rotation.0 {
            result = Self::rotate_cells(result, self.rotation_center());
        }
        result
    }

    /// Returns the rotation center for this piece.
    const fn rotation_center(self) -> (f32, f32) {
        match self {
            Self::I => (1.5, 0.5), // Center of 4x1
            Self::O => (0.5, 0.5), // Center of 2x2
            _ => (1.0, 0.5),       // Center of 3x2 bounding box for T, S, Z, J, L
        }
    }

    /// Rotates cells 90 degrees clockwise around the given center.
    #[allow(clippy::cast_possible_truncation)]
    fn rotate_cells(cells: [(i8, i8); 4], center: (f32, f32)) -> [(i8, i8); 4] {
        let mut result = [(0i8, 0i8); 4];
        for (i, (x, y)) in cells.iter().enumerate() {
            // Translate to center
            let fx = f32::from(*x) - center.0;
            let fy = f32::from(*y) - center.1;
            // Rotate 90° clockwise: (x, y) -> (y, -x)
            let rx = fy;
            let ry = -fx;
            // Translate back and round
            result[i] = ((rx + center.0).round() as i8, (ry + center.1).round() as i8);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i_piece_rotations() {
        let i = Tetromino::I;
        // Rotation 0: horizontal
        let r0 = i.cells(Rotation(0));
        assert!(r0.contains(&(0, 0)));
        assert!(r0.contains(&(1, 0)));
        assert!(r0.contains(&(2, 0)));
        assert!(r0.contains(&(3, 0)));

        // Rotation 1: vertical
        let r1 = i.cells(Rotation(1));
        // Should be vertical now
        let cols: Vec<i8> = r1.iter().map(|(c, _)| *c).collect();
        assert!(cols.iter().all(|&c| c == cols[0])); // All same column
    }

    #[test]
    fn test_o_piece_no_rotation() {
        let o = Tetromino::O;
        let r0 = o.cells(Rotation(0));
        let r1 = o.cells(Rotation(1));
        let r2 = o.cells(Rotation(2));
        let r3 = o.cells(Rotation(3));
        assert_eq!(r0, r1);
        assert_eq!(r1, r2);
        assert_eq!(r2, r3);
    }

    #[test]
    fn test_falling_piece_movement() {
        let piece = FallingPiece::spawn(Tetromino::T);
        let moved = piece.moved(1, -1);
        assert_eq!(moved.col, piece.col + 1);
        assert_eq!(moved.row, piece.row - 1);
    }

    #[test]
    fn test_rotation_cycle() {
        let r = Rotation(0);
        assert_eq!(r.clockwise().clockwise().clockwise().clockwise(), r);
        assert_eq!(r.counter_clockwise(), Rotation(3));
    }
}
