/// Provides function to load file and read/write it.
use stable_eyre::eyre::Result;
use std::fs;

/// Loads file and returns vector of lines as Strings.
pub fn load(filename: &str) -> Result<Vec<String>> {
    Ok(fs::read_to_string(filename)?
        .lines()
        .map(|s| s.trim_end().to_string())
        .collect())
}

/// Writes lines to file.
pub fn save(filename: &str, lines: &[String], line_idx: usize) -> Result<()> {
    fs::write(
        filename,
        [&[format!("typing-reader: {}", line_idx)], lines]
            .concat()
            .join("\n"),
    )?;
    Ok(())
}
