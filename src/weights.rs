use std::fmt::Write as _;
use std::path::Path;
use std::{fs, io};

/// Number of evaluation function weights.
pub const NUM_WEIGHTS: usize = 16;

/// Loads weights from a plain text file (one f64 per line).
///
/// # Errors
///
/// Returns an error if the file cannot be read, contains non-float values,
/// or does not contain exactly [`NUM_WEIGHTS`] values.
pub fn load(path: &Path) -> io::Result<[f64; NUM_WEIGHTS]> {
    let contents = fs::read_to_string(path)?;
    let values: Vec<f64> = contents
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            l.trim()
                .parse::<f64>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect::<io::Result<Vec<f64>>>()?;

    if values.len() != NUM_WEIGHTS {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("expected {NUM_WEIGHTS} weights, found {}", values.len()),
        ));
    }

    let mut weights = [0.0; NUM_WEIGHTS];
    weights.copy_from_slice(&values);
    Ok(weights)
}

/// Saves weights to a plain text file (one f64 per line).
///
/// # Errors
///
/// Returns an error if the file cannot be written.
pub fn save(path: &Path, weights: &[f64; NUM_WEIGHTS]) -> io::Result<()> {
    let contents = weights.iter().fold(String::new(), |mut s, w| {
        let _ = writeln!(s, "{w}");
        s
    });
    fs::write(path, contents)
}
