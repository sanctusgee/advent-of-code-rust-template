# Advent of Code Template (Rust)

[![Use this template](https://img.shields.io/badge/use-this%20template-blue?logo=github)](https://github.com/sanctusgee/advent-of-code-rust-template/generate)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# Advent of Code Rust Template

This repository provides a structured [Rust workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html) for solving Advent of Code puzzles.  
The layout is designed to stay manageable across all AoC days while keeping the code easy to navigate and extend.

---

## Workspace Structure

    aoc-lib/
    ├── lib.rs                # Year/day registry
    ├── year2024.rs           # Example year module
    ├── year2024/
    │   └── day01.rs          # Example solution
    └── utils/
        ├── mod.rs
        ├── input.rs          # Local + remote input loading
        └── output.rs         # Formatting helpers

    aoc/
    ├── main.rs               # Run/list/download commands
    └── bin/
        └── new-day.rs        # Generates year/day modules (prints lib.rs updates)

    benches/
    └── all_days.rs           # Criterion benchmarks

The template consists of just nine Rust files split across a small Cargo workspace.

---

## Prerequisites

- A Rust toolchain (`rustup` recommended)  
- An Advent of Code account  
- Your personal AoC session cookie (`session=...`)  

### How to get your AoC session cookie

1. Log in at https://adventofcode.com  
2. Open developer tools → Network  
3. Refresh the page  
4. Select any request to `adventofcode.com`  
5. Look at the `cookie` header  
6. Copy the value that begins with `session=`  

Export it:

    export AOC_SESSION="paste_the_value_here"

Keep this value private and **do not** commit it.

---

## Setup

    git clone https://github.com/sanctusgee/advent-of-code-rust-template
    cd advent-of-code-rust-template
    cargo build

After the initial build, you can start generating new years and days.

---

## Create a New Year / Day

### Day 1 (for example, 2025)

    cargo run - bin new-day 2025 1

This script:

- creates `aoc-lib/src/year2025.rs` if it doesn’t exist  
- creates `aoc-lib/src/year2025/day01.rs`  
- creates `input/2025/day01/`  
- prints the exact lines to add to `aoc-lib/src/lib.rs`  

Updating `lib.rs` is a manual step so you can control ordering and visibility.

### Next day

    cargo run - bin new-day 2025 2

Use the same command pattern for each new day.

---

## Download Inputs

    cargo run - bin aoc download 2025 1

Inputs are stored under:

    input/<year>/<day>/input.txt

They’re ignored by Git by default so you won’t commit personal data.

---

## Run a Solution

    cargo run - bin aoc run 2025 1

List available solvers:

    cargo run - bin aoc list

---

## Generated Day Scaffold

Each new day includes the same starter structure so you can focus directly on the puzzle:

    use crate::utils;
    use anyhow::Result;

    pub fn solve() -> Result<()> {
     let input = utils::load_input(2025, 5)?;
     
     let part1 = solve_part1(&input)?;
     let part2 = solve_part2(&input)?;
     
     println!("Day 5 / Year 2025");
     println!("Part 1: {}", part1);
     println!("Part 2: {}", part2);
     
     Ok(())
    }

    fn solve_part1(input: &str) -> Result<impl std::fmt::Display> {
     Ok(0)
    }

    fn solve_part2(input: &str) -> Result<impl std::fmt::Display> {
     Ok(0)
    }

The only convention is that each day exposes a `solve()` function returning `anyhow::Result<()>`.

---

## Testing

    cargo test day02

You can place tests directly alongside each day’s solution module.

---

## Benchmarking (Optional)

    cargo bench

Criterion will generate a report under:

    target/criterion/report/index.html

Benchmarking can help when experimenting with different solution approaches.

---

## Dependencies

Core libraries:

- `anyhow`
- `clap`
- `colored`
- `reqwest`
- `criterion`

Optional utilities (commented out by default):

- `regex`
- `itertools`
- `ahash`
- `atoi`
- `once_cell`

The workspace passes:

    cargo clippy --all-targets --all-features

without warnings to ensure a clean baseline.

---

## License

MIT
