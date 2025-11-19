// AUTO-GENERATED - DO NOT EDIT MANUALLY
// Regenerate with: cargo run --bin registry-tool

use anyhow::Result;

// Import all detected year modules
use crate::year2024;
use crate::year2025;

// Type alias for day registry entries
type DayEntry = (&'static str, fn() -> Result<()>);

pub struct SolutionRegistry;

// Helper: convert DAYS entries like ("01", solver) to Vec<u8>
fn days_to_u8(days: &[DayEntry]) -> Vec<u8> {
    days.iter().filter_map(|(d, _)| d.parse::<u8>().ok()).collect()
}

// Helper: find solver for a given day in a year's DAYS
fn find_solver(days: &[DayEntry], day: u8) -> Option<fn() -> Result<()>> {
    let day_str = day.to_string();
    days.iter().find(|(d, _)| *d == day_str).map(|(_, s)| *s)
}

impl SolutionRegistry {
    pub fn get_solver(year: u16, day: u8) -> Option<fn() -> Result<()>> {
        match year {
            2024 => find_solver(&year2024::DAYS, day),
            2025 => find_solver(&year2025::DAYS, day),
            _ => None,
        }
    }

    pub fn available_years() -> Vec<u16> {
        vec![2024, 2025]
    }

    pub fn available_days(year: u16) -> Vec<u8> {
        match year {
            2024 => days_to_u8(&year2024::DAYS),
            2025 => days_to_u8(&year2025::DAYS),
            _ => vec![],
        }
    }
}
