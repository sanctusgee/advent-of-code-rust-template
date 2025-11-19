use colored::*;
use std::fmt::Display;
use std::time::Duration;

/// Standard output format for solution results
pub struct SolutionOutput {
    pub year: u16,
    pub day: u8,
    pub part1: Option<String>,
    pub part2: Option<String>,
    pub elapsed: Option<Duration>,
}

impl SolutionOutput {
    pub fn new(year: u16, day: u8) -> Self {
        Self {
            year,
            day,
            part1: None,
            part2: None,
            elapsed: None,
        }
    }

    pub fn part1<T: Display>(mut self, result: T) -> Self {
        self.part1 = Some(result.to_string());
        self
    }

    pub fn part2<T: Display>(mut self, result: T) -> Self {
        self.part2 = Some(result.to_string());
        self
    }

    pub fn elapsed(mut self, duration: Duration) -> Self {
        self.elapsed = Some(duration);
        self
    }

    pub fn print(&self) {
        let title = format!("Day {} / Year {}", self.day, self.year);
        println!("{}", title.bright_cyan().bold());
        println!("{}", "─".repeat(title.len()).bright_black());

        if let Some(p1) = &self.part1 {
            println!("{} {}", "Part 1:".bright_green(), p1.bold());
        }

        if let Some(p2) = &self.part2 {
            println!("{} {}", "Part 2:".bright_green(), p2.bold());
        }

        if let Some(elapsed) = self.elapsed {
            let time_str = if elapsed.as_secs() > 0 {
                format!("{:.2}s", elapsed.as_secs_f64())
            } else if elapsed.as_millis() > 0 {
                format!("{}ms", elapsed.as_millis())
            } else {
                format!("{}μs", elapsed.as_micros())
            };
            println!("{} {}", "Time:".bright_black(), time_str.bright_black());
        }
        println!();
    }
}

// Helper macro for timing a block of code
#[macro_export]
macro_rules! timed {
    ($expr:expr) => {{
        let start = std::time::Instant::now();
        let result = $expr;
        let elapsed = start.elapsed();
        (result, elapsed)
    }};
}
