# Summary: Output Format & Sorting

## Results
- **Sorted Output**: Recharge options are now sorted by `Rate` (highest to lowest), making it easy to spot the best deals.
- **Table Format**: Replaced raw text output with a structured ASCII table using `comfy-table`.
- **Columns**: `Pay`, `Get`, `Base`, `Bonus`, `Rate`.

## Decisions
- **Library**: Selected `comfy-table` because it's feature-rich, easy to use, and produces good-looking CLI tables without manual alignment logic.
- **Sorting**: Used `partial_cmp` for floating-point comparison to handle `f64` sorting safely.

## Next Steps
- Add support for multiple output formats (JSON, YAML) via command line flags if needed in the future.
- Color-code the "Best Value" rows.
