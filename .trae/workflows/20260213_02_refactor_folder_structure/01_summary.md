# Summary: Refactor Folder Structure

## Outcome
Successfully moved `models.rs` and `process.rs` into a new `src/recharge/` module.

## Changes
1.  **New Directory**: Created `src/recharge/`.
2.  **File Moves**:
    - `src/models.rs` -> `src/recharge/models.rs`
    - `src/process.rs` -> `src/recharge/process.rs`
3.  **New File**: `src/recharge/mod.rs` to export the submodules.
4.  **Refactoring**:
    - Updated `src/main.rs` to use `mod recharge` and import from it.
    - Updated `src/recharge/process.rs` to import models from `crate::recharge::models`.

## Verification
- Ran `cargo run -- calc` -> Success.
