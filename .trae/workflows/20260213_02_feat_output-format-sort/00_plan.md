# Plan: Output Format & Sorting

## Goal
Enhance the game currency calculator to:
1. Sort recharge options by rate (highest to lowest).
2. Display the output in a clean, aligned table format.

## Steps

### 1. Research Table Libraries
- **Task**: Identify the best Rust library for CLI tables.
- **Candidates**: `comfy-table`, `tabled`, `cli-table`.
- **Selection Criteria**: Ease of use, alignment features, popularity.

### 2. Update Dependencies
- **Action**: Add the selected library to `Cargo.toml`.

### 3. Implement Sorting and Table Output
- **File**: `src/main.rs`.
- **Logic**:
  - Store calculated results in a vector.
  - Sort the vector by `rate` (descending).
  - Use the table library to render the sorted vector.

### 4. Verify
- **Action**: Run `cargo run` and check the output against the `honor_of_kings.json` data.
