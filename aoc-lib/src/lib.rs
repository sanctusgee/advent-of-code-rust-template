// aoc-lib/src/lib.rs

// For every new year created you must manually update this file in 4 locations:
// 1) Add `pub mod yearYYYY;`
// 2) Add year YYYY arm in `SolutionRegistry::get_solver(...)`
// 3) Add year YYYY in `SolutionRegistry::available_years()`
// 4) Add year YYYY arm in `SolutionRegistry::available_days(year)`
// These are one-time updates per year
//     ** I may automate this in the future;

pub mod utils;
pub mod year2024;

use anyhow::Result;

pub struct SolutionRegistry;

// I created Helper function here to simplify preceding code:
// :this convert `DAYS` entries like ("01", solver) to Vec<u8>
fn days_to_u8(days: &[(&'static str, fn() -> anyhow::Result<()> )]) -> Vec<u8> {
    days.iter().filter_map(|(d, _)| d.parse::<u8>().ok()).collect()
}

// Helper: find solver for a given day in a year's `DAYS`
fn find_solver(days: &[(&'static str, fn() -> Result<()> )], day: u8) -> Option<fn() -> Result<()>> {
    let day_str = day.to_string();
    days.iter().find(|(d, _)| *d == day_str).map(|(_, s)| *s)
}

impl SolutionRegistry {
    pub fn get_solver(year: u16, day: u8) -> Option<fn() -> Result<()>> {
        match year {
            // Manually add each new year here
            2024 => find_solver(&year2024::DAYS, day),
            // 2035 => find_solver(&year2035::DAYS, day),
            _ => None,
        }
    }

    pub fn available_years() -> Vec<u16> {
        vec![2024] // Manually update this list for each new year added
        // example is vec![2024, 2015, 2026, 2035, ...] // Ordering is not important
    }

    pub fn available_days(year: u16) -> Vec<u8> {
        match year {
            // Manually add each new year here. Ordering nice but not important
            2024 => days_to_u8(&year2024::DAYS),
            // 2035 => days_to_u8(&year2035::DAYS),
            _ => vec![],
        }
    }
}
