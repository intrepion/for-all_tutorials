<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Karvonen Heart Rate](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `karvonen-heart-rate` project.

## Goal

Build a small project app that introduces:

- converting prompted age and resting pulse into numeric values
- applying the Karvonen formula for one intensity level
- generating a stepped table from `55%` through `95%`
- rounding target heart rates to the nearest whole beat per minute with half-up rounding
- separating per-row calculation, table generation, and output formatting
- test-first calculation and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
calculate_target_heart_rate(age: integer, resting_pulse: integer, intensity_percent: integer) -> rate_bpm
generate_karvonen_table(age: integer, resting_pulse: integer) -> rows
format_karvonen_table(resting_pulse: integer, age: integer, rows) -> string[]
```

Where each row in `rows` contains:

```text
intensity_percent
rate_bpm
```

Canonical behavior:

- treat `age` as a positive whole number of years
- treat `resting_pulse` as a positive whole number of beats per minute
- treat `intensity_percent` as a whole-number percentage such as `55`, `60`, or `95`
- `calculate_target_heart_rate` applies the Karvonen formula:
  - `target = (((220 - age) - resting_pulse) * intensity) + resting_pulse`
  - `intensity` is `intensity_percent / 100`
- after computing `target`, round the result to the nearest whole beat per minute using half-up rounding
- `generate_karvonen_table` returns rows for every intensity from `55` through `95`, inclusive, stepping by `5`
- the row order must be:
  - `55`
  - `60`
  - `65`
  - `70`
  - `75`
  - `80`
  - `85`
  - `90`
  - `95`
- each row contains the intensity percent and the corresponding rounded target heart rate
- `format_karvonen_table` returns lines in this exact order:
  - `Resting Pulse: <resting_pulse> Age: <age>`
  - `Intensity | Rate`
  - `-------------|--------`
  - one line per row in this exact format:
    - `<intensity_percent>% | <rate_bpm> bpm`

Examples:

- `calculate_target_heart_rate(22, 65, 55)` returns `138`
- `calculate_target_heart_rate(22, 65, 60)` returns `145`
- `calculate_target_heart_rate(22, 65, 65)` returns `151`
- `calculate_target_heart_rate(22, 65, 85)` returns `178`
- `calculate_target_heart_rate(22, 65, 90)` returns `185`
- `calculate_target_heart_rate(22, 65, 95)` returns `191`
- the first generated row for `(22, 65)` is `{ intensity_percent: 55, rate_bpm: 138 }`
- the last generated row for `(22, 65)` is `{ intensity_percent: 95, rate_bpm: 191 }`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- decimal age or resting-pulse input values
- user-supplied intensity ranges or step sizes
- alternate table layouts beyond the canonical header and row format
- fitness advice beyond generating the table

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same Karvonen calculation, table-generation, and table-formatting behavior instead of redefining it.

Adapters should parse the prompted age and resting pulse into whole numbers before calling the core logic.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `calculate_target_heart_rate`, `generate_karvonen_table`, and `format_karvonen_table`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `calculate_target_heart_rate` on the canonical `55%`, `60%`, and `95%` examples
- tests that prove half-up rounding on a mid-table example such as `65% -> 151`
- tests that `generate_karvonen_table` returns exactly `9` rows in the correct order
- tests that the generated table starts at `55%` and ends at `95%`
- tests that `format_karvonen_table` preserves the exact header lines
- tests that `format_karvonen_table` preserves the exact row format
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly
