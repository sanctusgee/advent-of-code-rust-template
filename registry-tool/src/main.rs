// registry-tool/src/main.rs
// Standalone tool to regenerate the solution registry
// This tool is INDEPENDENT of aoc-lib and can run even when aoc-lib is broken

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("Scanning for year modules...");
    
    let years = scan_years()?;
    
    if years.is_empty() {
        println!("WARNING: No year modules found in aoc-lib/src/");
        println!("Expected directories like: aoc-lib/src/year2024_old/");
        return Ok(());
    }
    
    println!("Found {} year(s): {}", 
        years.len(),
        years.iter().map(|y| y.to_string()).collect::<Vec<_>>().join(", ")
    );
    
    // Update lib.rs
    update_lib_rs(&years)?;
    
    // Update registry_generated.rs
    update_registry_generated(&years)?;
    
    println!("Registry regeneration complete");
    
    Ok(())
}

// Scan aoc-lib/src for directories matching yearYYYY pattern
fn scan_years() -> Result<Vec<u16>> {
    let src_dir = PathBuf::from("aoc-lib/src");
    
    if !src_dir.exists() {
        anyhow::bail!("aoc-lib/src directory not found. Run this from the workspace root.");
    }
    
    let mut years = Vec::new();
    
    for entry in fs::read_dir(&src_dir)
        .with_context(|| format!("failed to read {}", src_dir.display()))? 
    {
        let entry = entry?;
        let path = entry.path();
        
        if !path.is_dir() {
            continue;
        }
        
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.starts_with("year") && name.len() == 8 {
                if let Ok(year) = name[4..].parse::<u16>() {
                    // Verify mod.rs exists
                    if path.join("mod.rs").exists() {
                        years.push(year);
                    }
                }
            }
        }
    }
    
    years.sort();
    Ok(years)
}

// Update aoc-lib/src/lib.rs to match detected years
fn update_lib_rs(years: &[u16]) -> Result<()> {
    let lib_path = PathBuf::from("aoc-lib/src/lib.rs");
    
    if !lib_path.exists() {
        anyhow::bail!("aoc-lib/src/lib.rs not found");
    }
    
    let content = fs::read_to_string(&lib_path)
        .with_context(|| format!("failed to read {}", lib_path.display()))?;
    
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    
    // Remove all existing "pub mod yearXXXX;" lines
    lines.retain(|line| {
        let trimmed = line.trim();
        !(trimmed.starts_with("pub mod year") && trimmed.ends_with(';'))
    });
    
    // Find where to insert year modules (after "pub mod utils;")
    let insert_idx = lines.iter().position(|line| line.trim() == "pub mod utils;")
        .map(|i| i + 1)
        .unwrap_or(0);
    
    // Insert all detected years
    for (i, year) in years.iter().enumerate() {
        lines.insert(insert_idx + i, format!("pub mod year{};", year));
    }
    
    let updated = lines.join("\n") + "\n";
    
    fs::write(&lib_path, updated)
        .with_context(|| format!("failed to write {}", lib_path.display()))?;
    
    println!("Updated aoc-lib/src/lib.rs with {} year module(s)", years.len());
    
    Ok(())
}

// Update aoc-lib/src/registry_generated.rs
fn update_registry_generated(years: &[u16]) -> Result<()> {
    let registry_content = build_registry_file(years);
    let registry_path = PathBuf::from("aoc-lib/src/registry_generated.rs");
    
    fs::write(&registry_path, registry_content)
        .with_context(|| format!("failed to write {}", registry_path.display()))?;
    
    println!("Updated aoc-lib/src/registry_generated.rs");
    
    Ok(())
}

// Build the complete registry_generated.rs file content
fn build_registry_file(years: &[u16]) -> String {
    let mut output = String::new();
    
    output.push_str("// AUTO-GENERATED - DO NOT EDIT MANUALLY\n");
    output.push_str("// Regenerate with: cargo run --bin registry-tool\n\n");
    output.push_str("use anyhow::Result;\n\n");
    
    output.push_str("// Import all detected year modules\n");
    for year in years {
        output.push_str(&format!("use crate::year{};\n", year));
    }
    output.push('\n');
    
    output.push_str("// Type alias for day registry entries\n");
    output.push_str("type DayEntry = (&'static str, fn() -> Result<()>);\n\n");
    
    output.push_str("pub struct SolutionRegistry;\n\n");
    
    output.push_str("// Helper: convert DAYS entries like (\"01\", solver) to Vec<u8>\n");
    output.push_str("fn days_to_u8(days: &[DayEntry]) -> Vec<u8> {\n");
    output.push_str("    days.iter().filter_map(|(d, _)| d.parse::<u8>().ok()).collect()\n");
    output.push_str("}\n\n");
    
    output.push_str("// Helper: find solver for a given day in a year's DAYS\n");
    output.push_str("fn find_solver(days: &[DayEntry], day: u8) -> Option<fn() -> Result<()>> {\n");
    output.push_str("    let day_str = day.to_string();\n");
    output.push_str("    days.iter().find(|(d, _)| *d == day_str).map(|(_, s)| *s)\n");
    output.push_str("}\n\n");
    
    output.push_str("impl SolutionRegistry {\n");
    
    output.push_str("    pub fn get_solver(year: u16, day: u8) -> Option<fn() -> Result<()>> {\n");
    output.push_str("        match year {\n");
    for year in years {
        output.push_str(&format!("            {} => find_solver(&year{}::DAYS, day),\n", year, year));
    }
    output.push_str("            _ => None,\n");
    output.push_str("        }\n");
    output.push_str("    }\n\n");
    
    output.push_str("    pub fn available_years() -> Vec<u16> {\n");
    output.push_str("        vec![");
    output.push_str(&years.iter().map(|y| y.to_string()).collect::<Vec<_>>().join(", "));
    output.push_str("]\n");
    output.push_str("    }\n\n");
    
    output.push_str("    pub fn available_days(year: u16) -> Vec<u8> {\n");
    output.push_str("        match year {\n");
    for year in years {
        output.push_str(&format!("            {} => days_to_u8(&year{}::DAYS),\n", year, year));
    }
    output.push_str("            _ => vec![],\n");
    output.push_str("        }\n");
    output.push_str("    }\n");
    
    output.push_str("}\n");
    
    output
}
