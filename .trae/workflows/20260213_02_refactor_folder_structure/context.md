# Workflow Context

- **ID**: 20260213_02_refactor_folder_structure
- **Type**: Refactor
- **Status**: In Progress
- **Current Step**: Analysis
- **Goal**: Rename and move `models.rs` and `process.rs` into a dedicated folder for game currency recharge calculation.

## User Request
- Move current functionality (`models`, `process`) into a separate folder.
- Name should be descriptive (e.g., related to game currency recharge calculation).
- Suggestion: `recharge` or similar.

## Plan
1. [ ] Analysis: Confirm current file dependencies.
2. [ ] Rename: Decide on the folder name (e.g., `recharge`).
3. [ ] Move: Move files to `src/recharge/`.
4. [ ] Refactor: Update `mod.rs` and imports.
5. [ ] Verify: Ensure `cargo run -- calc` still works.
