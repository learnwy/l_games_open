# Plan: Architecture Upgrade to Clap Subcommands

## Objective
Upgrade the project architecture to support multiple distinct features/games and shared capabilities using the `clap` subcommand pattern.

## Rationale
We chose **Option 1: Clap Subcommands** because:
- **Unified Interface**: A single entry point (`l-games`) is easier for users than managing multiple binaries.
- **Shared Capabilities**: It's trivial to share code (config parsing, formatting) across subcommands.
- **Extensibility**: New features can be added as new modules and registered as subcommands.

## Proposed Structure
We will refactor the single `main.rs` into a modular structure:

```text
src/
  bin/
    l-games.rs      # Entry point (clap CLI parsing)
  lib.rs            # Library root (exports modules)
  common/           # Shared capabilities
    mod.rs
    config.rs       # GameConfig, Step definitions
    output.rs       # Table formatting
  games/            # Specific game logic (if needed beyond config)
    mod.rs
    calculator.rs   # The current logic (refactored)
```

## Steps
1.  **Setup Dependencies**: Add `clap` (with `derive` feature) to `Cargo.toml`.
2.  **Extract Common Logic**: Move structs (`Step`, `GameConfig`, `CalculatedStep`) and processing logic to modules.
3.  **Implement CLI**: Create the `clap` struct definitions for commands.
    - `Calc`: The existing functionality (reads json, prints table).
4.  **Refactor Main**: Update `main` to parse args and dispatch to the correct module.
5.  **Verify**: Ensure existing functionality works via the new command (e.g., `cargo run -- calc`).

## Future Proofing
- If a game needs custom logic (not just generic config), we can add `src/games/my_new_game.rs` and a corresponding subcommand.
- Shared helpers go into `src/common/`.
