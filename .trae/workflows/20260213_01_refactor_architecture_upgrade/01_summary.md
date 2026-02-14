# Summary: Architecture Upgrade to Clap Subcommands

## Outcome
Successfully upgraded the project to use `clap` for CLI argument parsing and subcommand management.

## Changes
1.  **Dependencies**: Added `clap` to `Cargo.toml`.
2.  **Refactoring**:
    -   Moved data structures to `src/models.rs`.
    -   Moved processing logic to `src/process.rs`.
    -   Updated `src/main.rs` to be the CLI entry point using `clap`.
3.  **New Command**:
    -   `l-games calc`: Runs the existing calculator logic.
    -   Supports `--data-dir` argument (default: "data").

## How to Extend
-   **Add a new feature**:
    1.  Create a new module (e.g., `src/my_feature.rs`).
    2.  Add a new variant to the `Commands` enum in `src/main.rs`.
    3.  Call the module's function in the `match` block in `main`.

## Verification
-   Ran `cargo run -- calc` -> Success.
-   Ran `cargo run -- --help` -> Success.
