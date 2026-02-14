# Summary: Game Calculator MVP

## Results
- **Project Initialized**: Rust project created in root.
- **Dependencies**: `serde`, `serde_json` added.
- **Feature Implemented**:
  - Reads JSON files from `data/`.
  - Calculates unit rate (Coins per Currency Unit).
  - Handles `extras` correctly.
- **Verification**:
  - Sample `honor_of_kings.json` processed correctly.
  - Rate: 105.00 (10500 / 100).

## Decisions
- **Language**: Rust chosen for type safety and performance (matches user preference/default).
- **Structure**: Single `main.rs` for MVP simplicity.
- **Data**: Stored in `data/` folder for extensibility.

## Next Steps
- Add unit tests.
- Support command line arguments (e.g., specific file, output format).
- Add more game data.
