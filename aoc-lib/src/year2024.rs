// Example day - kept as reference implementation
// Remove or add more days as needed
mod day01;

// Type alias for cleaner code
type SolverFn = fn() -> anyhow::Result<()>;

// Map day numbers to their solve functions
pub const DAYS: &[(&str, SolverFn)] = &[
    ("1", day01::solve),
    // Add more days here as you implement them:
    // ("2", day02::solve),
    // ("3", day03::solve),
    // etc.
];