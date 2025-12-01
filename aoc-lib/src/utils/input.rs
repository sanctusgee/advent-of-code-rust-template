// `aoc-lib/src/utils/input.rs`

use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;

// Get the path to an input file for a specific year and day
pub fn get_input_path(year: u16, day: u8) -> PathBuf {
    PathBuf::from(format!("input/year{}/day{:02}.txt", year, day))
}

// Load input file as a single string
pub fn load_input(year: u16, day: u8) -> Result<String> {
    let path = get_input_path(year, day);

    if !path.exists() {
        return Err(anyhow!(
            "Input file not found - {}\n\n\
            To download it automatically, run:\n    \
            cargo run --bin aoc download {} {}\n\n\
            Or create the file manually if you want to paste input by hand.",
            path.display(),
            year,
            day
        ));
    }
    std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read input file: {}", path.display()))
}


// Load input file as lines
pub fn load_input_lines(year: u16, day: u8) -> Result<Vec<String>> {
    let content = load_input(year, day)?;
    Ok(content.lines().map(String::from).collect())
}

/// Download input from Advent of Code website
/// Requires AOC_SESSION env var; accepts either raw token or "session=<token>"
pub fn download_input(year: u16, day: u8) -> Result<String> {
    // basic day guard
    if day == 0 || day > 25 {
        return Err(anyhow!("Day must be between 1 and 25"));
    }

    let session = std::env::var("AOC_SESSION")
        .context("AOC_SESSION environment variable not set")?;
    // allow both formats
    let session = session.strip_prefix("session=").unwrap_or(&session);

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = reqwest::blocking::Client::builder()
        .user_agent("github.com/sanctusgee/aoc-rust (Rust reqwest)")
        .build()
        .context("Failed to build HTTP client")?;

    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()
        .context("Failed to send request to AoC")?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to download input: HTTP {}", response.status());
    }

    let text = response.text().context("Failed to read response text")?;

    // detect empty or HTML login page
    if text.trim().is_empty() || text.trim_start().starts_with("<!DOCTYPE") {
        anyhow::bail!(
            "Downloaded empty or HTML content. Verify AOC_SESSION token and puzzle availability."
        );
    }

    Ok(text)
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
