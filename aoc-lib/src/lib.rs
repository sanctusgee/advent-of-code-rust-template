pub mod utils;

// Year modules - add your years here as you implement them
// Example: pub mod year2024;
pub mod year2024;

use anyhow::Result;

/// Type alias for solver functions
pub type SolverFn = fn() -> Result<()>;

/// Trait that all solution modules must implement
pub trait Solution {
    fn solve() -> Result<()>;
}

/// Registry of all available solutions
pub struct SolutionRegistry;

impl SolutionRegistry {
    /// Get solver function for a specific year and day
    pub fn get_solver(year: u16, day: u8) -> Option<SolverFn> {
        match year {
            2024 => Self::get_year_solver(year2024::DAYS, day),
            // Add more years here:
            // 2025 => Self::get_year_solver(year2025::DAYS, day),
            _ => None,
        }
    }

    /// Generic helper to get solver from a year's DAYS array
    fn get_year_solver(days: &[(&str, SolverFn)], day: u8) -> Option<SolverFn> {
        days.iter()
            .find(|(d, _)| d.parse::<u8>().ok() == Some(day))
            .map(|(_, solver)| *solver)
    }

    /// List all available years
    pub fn available_years() -> Vec<u16> {
        vec![
            2024,
            // Add more years here as you implement them:
            // 2025,
        ]
    }

    /// List all available days for a year
    pub fn available_days(year: u16) -> Vec<u8> {
        let days = match year {
            2024 => &year2024::DAYS,
            // Add more years here:
            // 2025 => &year2025::DAYS,
            _ => return vec![],
        };

        days.iter()
            .filter_map(|(d, _)| d.parse().ok())
            .collect()
    }
}
