use std::{env, io};

/// Minimal CLI argument parser available to all binaries.
pub struct Cli {
    args: Vec<String>,
}

impl Cli {
    /// Parses arguments from `std::env::args`.
    #[must_use]
    pub fn parse() -> Self {
        Self {
            args: env::args().collect(),
        }
    }

    /// Returns `true` if `--help` or `-h` was passed.
    #[must_use]
    pub fn help_requested(&self) -> bool {
        self.args.iter().any(|a| a == "--help" || a == "-h")
    }

    /// Returns the raw string value following `flag`, if present.
    #[must_use]
    pub fn get(&self, flag: &str) -> Option<&str> {
        self.args
            .iter()
            .position(|a| a == flag)
            .and_then(|i| self.args.get(i + 1))
            .map(String::as_str)
    }

    /// Returns all values following repeated occurrences of `flag`.
    #[must_use]
    pub fn get_all(&self, flag: &str) -> Vec<&str> {
        self.args
            .iter()
            .enumerate()
            .filter(|(_, a)| a.as_str() == flag)
            .filter_map(|(i, _)| self.args.get(i + 1))
            .map(String::as_str)
            .collect()
    }

    /// Parses a string value into `T`, producing a user-friendly error on failure.
    ///
    /// # Errors
    ///
    /// Returns `InvalidInput` if the value cannot be parsed.
    pub fn parse_value<T: std::str::FromStr>(&self, flag: &str, value: &str) -> io::Result<T>
    where
        T::Err: std::fmt::Display,
    {
        value.parse().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid value for {flag}: {e}"),
            )
        })
    }
}

/// Applies CLI flags to struct fields in a single declarative block.
///
/// For each `"--flag" => field` pair, if the flag is present on the command line
/// its value is parsed into the field's type.
///
/// # Example
///
/// ```ignore
/// let mut config = OptimizeConfig::default();
/// apply_flags!(cli, {
///     "--iterations" => config.iterations,
///     "--bandwidth"  => config.bandwidth,
/// });
/// ```
#[macro_export]
macro_rules! apply_flags {
    ($cli:expr, { $($flag:expr => $field:expr),* $(,)? }) => {
        $(
            if let Some(val) = $cli.get($flag) {
                $field = $cli.parse_value($flag, val)?;
            }
        )*
    };
}
