# Release Notes - v0.2.0

## Automatic Registry Management

This release eliminates manual module maintenance. Previously, adding a new year required updating `lib.rs` in 4 different places. Now it's fully automatic.

### What's New

#### Auto-Generated Registry System
- `registry_generated.rs` - Auto-generated file containing all routing logic
- `registry-tool` - Independent binary that scans file structure and rebuilds the registry
- `new-day` now handles all registry updates automatically

The registry system also handles deletions - if you remove a year folder, `registry-tool` detects it and cleans up all references in lib.rs and the generated registry.


#### Self-Healing Architecture
`registry-tool` is standalone with zero dependencies on `aoc-lib`:
- Runs even when `aoc-lib` won't compile
- Fixes broken states from manually deleted year folders
- Scans actual directory structure and syncs everything

#### Year Validation
- Years must be between 2015 and 2099
- Catches typos: `35`, `2125`, `0035` rejected
- Enforces 4-digit years

#### Improved Error Messages
- Actionable suggestions when solutions not found
- Directs users to `new-day` or `registry-tool` commands

### Breaking Changes

#### File Structure
- **New file:** `aoc-lib/src/registry_generated.rs` (auto-generated, do not edit)
- **New crate:** `registry-tool/` (independent registry manager)
- **Modified:** `aoc-lib/src/lib.rs` (simplified from 50+ lines to 7 lines)

#### Removed Files
- None

#### Command Changes
- **Added:** `cargo run --bin registry-tool` (fix broken registry states)

### Migration from v0.1

**Your solution files are safe.** This upgrade only changes tooling infrastructure.

#### What Changes

Infrastructure files only:
- `lib.rs` - Simplified from 50+ lines to 7 lines
- `registry_generated.rs` - New auto-generated file
- `registry-tool/` - New standalone crate
- `new-day` binary - Updated to call registry-tool

Your `aoc-lib/src/yearYYYY/dayDD.rs` solution files are never touched.

#### Upgrade Steps

1. Pull v0.2.0 changes
2. Run `cargo run --bin registry-tool`
3. Verify with `cargo run --bin aoc list`


### Before and After

**v0.1:** Creating a new year required manually editing `lib.rs` in 4 places:
1. Add `pub mod yearYYYY;`
2. Add year case to `get_solver()`
3. Add year to `available_years()`
4. Add year case to `available_days()`

**v0.2.0:** Run `cargo run --bin new-day 2024 1` and everything updates automatically.

### Workflow

**Create and run solutions:**
```bash
cargo run --bin new-day 2024 1
cargo run --bin aoc download 2024 1
cargo run --bin aoc run 2024 1
```

**Fix broken registry:**
```bash
cargo run --bin registry-tool
```

### Technical Details

**Registry Architecture:**
- `lib.rs` - Module declarations only (auto-managed)
- `registry_generated.rs` - SolutionRegistry struct and routing logic
- `yearYYYY/mod.rs` - DAYS array mapping days to solvers

**How `new-day` works:**
1. Creates solution file with template
2. Updates year module to register the day
3. Adds module declaration to `lib.rs` (if new year)
4. Calls `registry-tool` to regenerate routing
5. Creates input placeholder

**How `registry-tool` works:**
1. Scans `aoc-lib/src/` for `yearYYYY` directories
2. Verifies each year has `mod.rs`
3. Syncs `lib.rs` module declarations
4. Regenerates `registry_generated.rs`

### Benefits

- Zero manual editing of `lib.rs`
- Self-healing when files are deleted
- Fail-safe (registry-tool works even when aoc-lib won't compile)
- Cleaner code (lib.rs: 50+ lines to 7 lines)

---

**Full Changelog:** v0.1.0...v0.2.0
