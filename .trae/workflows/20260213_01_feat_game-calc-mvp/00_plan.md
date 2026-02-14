# Plan: Game Calculator MVP

## Goal
Initialize the `l-games` project as a Rust application and implement a feature to calculate game currency exchange rates from JSON configuration files.

## Steps

### 1. Initialize Project
- **Action**: Initialize a new Rust package in the current directory.
- **Dependencies**: `serde`, `serde_json` (for JSON parsing).

### 2. Implement Data Structures & Logic
- **File**: `src/main.rs` (or `src/lib.rs` if we split it, but `main.rs` is fine for MVP).
- **Structs**:
  - `RechargeStep`: `source` (cost), `target` (base coins), `extras` (bonus coins).
  - `GameConfig`: `game` name, `source` currency name, `target` currency name, list of `steps`.
- **Logic**:
  - Iterate through files in a `data` directory.
  - Parse JSON.
  - For each step, calculate `rate = (target + sum(extras)) / source`.
  - Print the results nicely.

### 3. Create Sample Data
- **Directory**: `data/`
- **File**: `data/honor_of_kings.json` (Sample provided by user).

### 4. Run & Verify
- **Action**: Run `cargo run`.
- **Verification**: Check if output matches manual calculation.
