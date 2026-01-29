use std::fmt::Write as _;
use std::path::Path;
use std::{fs, io};

use crate::agent::ScoringMode;

/// Number of evaluation function weights.
pub const NUM_WEIGHTS: usize = 16;

const HEADER_PREFIX: &str = "# scoring-mode: ";

/// Loads weights from a text file, returning the weights and the scoring mode.
///
/// Files may optionally start with a `# scoring-mode: <MODE>` header line.
/// Lines starting with `#` are skipped when parsing weight values.
/// Files without the header default to [`ScoringMode::Full`].
///
/// # Errors
///
/// Returns an error if the file cannot be read, contains non-float values,
/// or does not contain exactly [`NUM_WEIGHTS`] values.
pub fn load(path: &Path) -> io::Result<([f64; NUM_WEIGHTS], ScoringMode)> {
    let contents = fs::read_to_string(path)?;

    let mut scoring_mode = ScoringMode::Full;

    for line in contents.lines() {
        let trimmed = line.trim();
        if let Some(mode_str) = trimmed.strip_prefix(HEADER_PREFIX) {
            scoring_mode = mode_str
                .trim()
                .parse()
                .map_err(|e: String| io::Error::new(io::ErrorKind::InvalidData, e))?;
            break;
        }
        // Only check the first non-empty line for the header
        if !trimmed.is_empty() {
            break;
        }
    }

    let values: Vec<f64> = contents
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.is_empty() && !t.starts_with('#')
        })
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
    Ok((weights, scoring_mode))
}

/// Saves weights to a text file with a `# scoring-mode:` header.
///
/// # Errors
///
/// Returns an error if the file cannot be written.
pub fn save(
    path: &Path,
    weights: &[f64; NUM_WEIGHTS],
    scoring_mode: ScoringMode,
) -> io::Result<()> {
    let mut contents = format!("{HEADER_PREFIX}{scoring_mode}\n");
    for w in weights {
        let _ = writeln!(contents, "{w}");
    }
    fs::write(path, contents)
}
