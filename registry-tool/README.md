# Registry Tool

Independent tool for regenerating the Advent of Code solution registry.

## Purpose

This tool scans `aoc-lib/src/` for year modules and automatically updates:
- `aoc-lib/src/lib.rs` - Adds/removes `pub mod yearXXXX;` declarations
- `aoc-lib/src/registry_generated.rs` - Regenerates the solution registry

## Why Independent?

This tool is a **standalone crate** with no dependencies on `aoc-lib`. This means:
- It can run even when `aoc-lib` is broken (e.g., missing year modules)
- It's self-healing - fixes broken states automatically
- No circular dependency issues

## Usage

```bash
cargo run --bin registry-tool
```

## When It Runs

The registry tool runs automatically:
- After creating a new day with `cargo run --bin new-day`
- When manually invoked

## What It Does

1. Scans `aoc-lib/src/` for directories matching `yearYYYY` pattern
2. Verifies each year has a `mod.rs` file
3. Updates `lib.rs` to match detected years (removes orphaned, adds missing)
4. Regenerates `registry_generated.rs` with all detected years
