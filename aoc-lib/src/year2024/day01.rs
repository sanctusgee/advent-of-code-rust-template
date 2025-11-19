// Auto-generated day stub. Do not delete solve()
// Add you code to solve(), or implement other fn and call from solve().

use anyhow::Result;
use crate::utils;


// Example template.

pub fn solve() -> Result<()> {
// Load your input file.
	let input = utils::load_input(2024, 1)?;

	let part1 = solve_part1(&input)?;
	let part2 = solve_part2(&input)?;

	println!("Day 1 / Year 2024");
	println!("Part 1: {}", part1);
	println!("Part 2: {}", part2);

	Ok(())
}

// Rename _input variable in fn signature back to input after implementing the solution
fn solve_part1(_input: &str) -> Result<impl std::fmt::Display> {
	// add your Part 1 solution here
	Ok(0)
}

// Rename _input variable in fn signature back to input after implementing the solution
fn solve_part2(_input: &str) -> Result<impl std::fmt::Display> {
	// add your Part 2 solution here
	Ok(0)
}