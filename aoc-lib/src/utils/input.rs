use anyhow::{Context, Result};
use std::path::PathBuf;

/// Get the path to an input file for a specific year and day
pub fn get_input_path(year: u16, day: u8) -> PathBuf {
    PathBuf::from(format!("input/year{}/day{:02}.txt", year, day))
}

/// Load input file as a single string
pub fn load_input(year: u16, day: u8) -> Result<String> {
    let path = get_input_path(year, day);
    std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read input file: {}", path.display()))
}

/// Load input file as lines
pub fn load_input_lines(year: u16, day: u8) -> Result<Vec<String>> {
    let content = load_input(year, day)?;
    Ok(content.lines().map(String::from).collect())
}

/// Download input from Advent of Code website
/// Requires AOC_SESSION environment variable to be set
pub fn download_input(year: u16, day: u8) -> Result<String> {
    let session = std::env::var("AOC_SESSION")
        .context("AOC_SESSION environment variable not set")?;

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = reqwest::blocking::Client::new();

    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()
        .context("Failed to send request to AoC")?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to download input: HTTP {}", response.status());
    }

    response.text().context("Failed to read response text")
}

/// Download and cache input file
pub fn ensure_input(year: u16, day: u8) -> Result<String> {
    let path = get_input_path(year, day);

    // If file exists, read it
    if path.exists() {
        return load_input(year, day);
    }

    // Otherwise, download it
    let content = download_input(year, day)?;

    // Create directory if needed
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .context("Failed to create input directory")?;
    }

    // Save to file
    std::fs::write(&path, &content)
        .with_context(|| format!("Failed to write input to {}", path.display()))?;

    Ok(content)
}

/// Parse lines by delimiter (e.g., "value: 1 2 3" -> (value, [1, 2, 3]))
pub fn parse_lines_with_delimiter<T, U>(
    lines: &[String],
    delimiter: &str,
) -> Result<Vec<(T, Vec<U>)>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + Send + Sync + 'static,
{
    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let parts: Vec<&str> = line.split(delimiter).collect();
            if parts.len() != 2 {
                anyhow::bail!("Line {} has invalid format", i + 1);
            }

            let first = parts[0].trim().parse::<T>()
                .context(format!("Failed to parse first part on line {}", i + 1))?;

            let second = parts[1]
                .split_whitespace()
                .map(|s| s.parse::<U>())
                .collect::<Result<Vec<_>, _>>()
                .context(format!("Failed to parse second part on line {}", i + 1))?;

            Ok((first, second))
        })
        .collect()
}

/// Parse lines of whitespace-separated values
pub fn parse_lines<T>(lines: &[String]) -> Result<Vec<Vec<T>>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.split_whitespace()
                .map(|s| s.parse::<T>())
                .collect::<Result<Vec<_>, _>>()
                .context(format!("Failed to parse line {}", i + 1))
        })
        .collect()
}
