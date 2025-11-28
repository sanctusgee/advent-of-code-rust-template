// aoc-lib/src/year2024/mod.rs


// Example day - kept as reference implementation. Remove or add more days as needed
mod day01;

pub const DAYS: &[(&str, fn() -> anyhow::Result<()>)] =
&[
    ("1", day01::solve),
];
