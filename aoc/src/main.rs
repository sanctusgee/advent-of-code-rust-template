use anyhow::{Context, Result};
use aoc_lib::SolutionRegistry;
use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "aoc")]
#[command(about = "Advent of Code Solutions Runner", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a solution for a specific day
    Run {
        /// Year (e.g., 2024)
        year: u16,
        /// Day (1-25)
        day: u8,
    },
    /// List all available solutions
    List {
        /// Optional year filter
        year: Option<u16>,
    },
    /// Download input for a specific day (requires AOC_SESSION env var)
    Download {
        /// Year (e.g., 2024)
        year: u16,
        /// Day (1-25)
        day: u8,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { year, day } => run_solution(year, day),
        Commands::List { year } => list_solutions(year),
        Commands::Download { year, day } => download_input(year, day),
    }
}

fn run_solution(year: u16, day: u8) -> Result<()> {
    if !(2015..=2099).contains(&year) {
        anyhow::bail!("Year must be between 2015 and 2099 (Advent of Code years)");
    }
    
    // ToDo: hardcoded: Starting 2025, AoC is now only 12 days
    //       intentionally leaving range of 1 to 25 so we can still use code for previous years
    //     **  In future, I'll figure out a way to create a dynamic variable
    //      that is set as part of the Day 1 template creation
    if !(1..=25).contains(&day) {
        anyhow::bail!("Day must be between 1 and 25");
    }

    let solver = SolutionRegistry::get_solver(year, day)
        .with_context(|| format!(
            "No solution found for year {} day {}\n\nTo create this day: cargo run --bin new-day {} {}\nIf the day exists: cargo run --bin registry-tool",
            year, day, year, day
        ))?;

    solver()
}

fn list_solutions(year_filter: Option<u16>) -> Result<()> {
    let years = if let Some(year) = year_filter {
        vec![year]
    } else {
        SolutionRegistry::available_years()
    };

    for year in years {
        let days = SolutionRegistry::available_days(year);
        if days.is_empty() {
            continue;
        }

        println!("{}", format!("Year {}", year).bright_cyan().bold());
        println!("{}", "─".repeat(40).bright_black());

        let mut day_ranges = vec![];
        let mut start = days[0];
        let mut end = days[0];

        for &day in &days[1..] {
            if day == end + 1 {
                end = day;
            } else {
                if start == end {
                    day_ranges.push(format!("{}", start));
                } else {
                    day_ranges.push(format!("{}-{}", start, end));
                }
                start = day;
                end = day;
            }
        }

        if start == end {
            day_ranges.push(format!("{}", start));
        } else {
            day_ranges.push(format!("{}-{}", start, end));
        }

        println!("Days: {}", day_ranges.join(", "));
        println!();
    }

    Ok(())
}

fn download_input(year: u16, day: u8) -> Result<()> {
    if !(2015..=2099).contains(&year) {
        anyhow::bail!("Year must be between 2015 and 2099 (Advent of Code years)");
    }
    
    // ToDo: hardcoded: Starting 2025, AoC is now only 12 days
    //       intentionally leaving range of 1 to 25 so we can still use code for previous years
    //     **  In future, I'll figure out a way to create a dynamic variable
    //      that is set as part of the Day 1 template creation
    if !(1..=25).contains(&day) {
        anyhow::bail!("Day must be between 1 and 25");
    }

    println!(
        "{}",
        format!("Downloading input for {} day {}", year, day)
            .bright_cyan()
    );

    let content = aoc_lib::utils::ensure_input(year, day)?;
    let lines = content.lines().count();

    println!("Downloaded {} lines", lines);

    Ok(())
}
