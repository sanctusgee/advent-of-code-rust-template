use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <year> <day>", args[0]);
        std::process::exit(1);
    }

    let year: u16 = args[1].parse().context("Invalid year")?;
    let day: u8 = args[2].parse().context("Invalid day")?;

    if !(1..=25).contains(&day) {
        anyhow::bail!("Day must be between 1 and 25");
    }

    create_day_template(year, day)?;

    println!("✓ Created template for year {} day {}", year, day);
    println!("\nNext steps:");
    println!("  1. Add input to: input/year{}/day{:02}.txt", year, day);
    println!(
        "  2. Implement solution in: aoc-lib/src/year{}/day{:02}.rs",
        year, day
    );
    println!("  3. Run with: cargo run --bin aoc run {} {}", year, day);

    Ok(())
}

fn create_day_template(year: u16, day: u8) -> Result<()> {
    // Create solution file
    let solution_path = PathBuf::from(format!("aoc-lib/src/year{}/day{:02}.rs", year, day));

    if solution_path.exists() {
        anyhow::bail!("Solution file already exists: {}", solution_path.display());
    }

    let template = generate_solution_template(year, day);

    if let Some(parent) = solution_path.parent() {
        fs::create_dir_all(parent).context("Failed to create year directory")?;
    }

    fs::write(&solution_path, template).context("Failed to write solution file")?;

    // Update year module file
    update_year_module(year, day)?;

    // Create input directory
    let input_dir = PathBuf::from(format!("input/year{}", year));
    fs::create_dir_all(&input_dir).context("Failed to create input directory")?;

    // Create empty input file
    let input_path = input_dir.join(format!("day{:02}.txt", day));
    if !input_path.exists() {
        fs::write(&input_path, "").context("Failed to create input file")?;
    }

    Ok(())
}

fn generate_solution_template(year: u16, day: u8) -> String {
    let template = r#"use crate::utils;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = utils::load_input(YEAR, DAY)?;

    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;

    println!("Day DAY / Year YEAR");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 1
    Ok(0)
}

fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
    // TODO: Implement part 2
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_part1() {
        let result = solve_part1(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "0");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(EXAMPLE).unwrap();
        assert_eq!(result.to_string(), "0");
    }
}
"#;

    template
        .replace("YEAR", &year.to_string())
        .replace("DAY", &day.to_string())
}

fn update_year_module(year: u16, day: u8) -> Result<()> {
    let module_path = PathBuf::from(format!("aoc-lib/src/year{}.rs", year));

    // If year module doesn't exist, create it
    if !module_path.exists() {
        let initial_content = format!(
            "mod day{:02};\n\npub const DAYS: &[(&str, fn() -> anyhow::Result<()>)] = &[\n    (\"{}\", day{:02}::solve),\n];\n",
            day, day, day
        );

        fs::write(&module_path, initial_content).context("Failed to create year module")?;

        // Update lib.rs to include this year if needed
        update_lib_rs(year)?;

        return Ok(());
    }

    // Read existing content
    let content = fs::read_to_string(&module_path).context("Failed to read year module")?;

    // Check if day already exists
    let day_mod = format!("mod day{:02};", day);
    if content.contains(&day_mod) {
        anyhow::bail!("Day {} already registered in year module", day);
    }

    // Simple approach: insert module at the end of mod declarations
    // and add to DAYS array before the closing bracket
    let mut result = String::new();
    let mut in_days_array = false;
    let mut inserted_mod = false;

    for line in content.lines() {
        if line.starts_with("mod day") && !inserted_mod {
            // Check if we should insert before this line
            if let Some(current_day_str) = line.strip_prefix("mod day").and_then(|s| s.strip_suffix(';')) {
                if let Ok(current_day) = current_day_str.parse::<u8>() {
                    if day < current_day {
                        result.push_str(&day_mod);
                        result.push('\n');
                        inserted_mod = true;
                    }
                }
            }
        }

        result.push_str(line);
        result.push('\n');

        // Check if this is the last mod line
        if line.starts_with("mod day") && !inserted_mod {
            // Check if next line is not a mod line by peeking
            let mut should_insert = true;
            for future_line in content.lines().skip_while(|l| l != &line).skip(1) {
                if !future_line.trim().is_empty() {
                    if future_line.starts_with("mod day") {
                        should_insert = false;
                    }
                    break;
                }
            }
            if should_insert {
                result.push_str(&day_mod);
                result.push('\n');
                inserted_mod = true;
            }
        }

        if line.contains("pub const DAYS:") {
            in_days_array = true;
        }

        if in_days_array && line.trim() == "];" {
            // Insert before the closing bracket
            let indent = "    ";
            result.pop(); // Remove the ];
            result.pop(); // Remove the newline
            result.push('\n');
            result.push_str(&format!("{}(\"{}\", day{:02}::solve),\n", indent, day, day));
            result.push_str("];\n");
            in_days_array = false;
        }
    }

    fs::write(&module_path, result).context("Failed to update year module")?;

    Ok(())
}

fn update_lib_rs(year: u16) -> Result<()> {
    let lib_path = PathBuf::from("aoc-lib/src/lib.rs");
    let content = fs::read_to_string(&lib_path).context("Failed to read lib.rs")?;

    // Check if year module already exists
    let year_mod = format!("pub mod year{};", year);
    if content.contains(&year_mod) {
        return Ok(());
    }

    // This is a new year, manual intervention recommended
    println!("WARNING: New year {} detected. You'll need to manually:", year);
    println!("  1. Add 'pub mod year{};' to aoc-lib/src/lib.rs", year);
    println!("  2. Add year {} case to SolutionRegistry", year);

    Ok(())
}
