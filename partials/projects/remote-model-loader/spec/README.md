---
partial_kind: project-spec
project: remote-model-loader
---

# Spec

Canonical project contract for the `remote-model-loader` project.

## Goal

Build a small project app that introduces:

- loading one remote 3D model asset
- exposing precise loading progress
- rendering a friendly loading message and a friendly error message
- deferring user interaction until the model is ready
- thin adapters

## Canonical Remote Sources

Every tutorial run for this project should treat these source values as canonical equivalents:

```text
valid_model_label: sample-box
valid_model_url: https://example.com/models/sample-box.glb
broken_model_url: https://example.com/models/missing.glb
```

## Core Logic Contract

The shared core contracts are:

```text
calculate_load_percentage(loaded_bytes: integer, total_bytes: integer) -> integer
format_loading_message(load_percentage: integer) -> string
format_model_load_error(model_label: string) -> string
can_interact_with_model(has_loaded: boolean, has_error: boolean) -> boolean
```

Canonical behavior:

- `calculate_load_percentage`:
  - returns an integer in the inclusive range `0` to `100`
  - rounds down fractional progress
  - returns `100` when `loaded_bytes >= total_bytes`
  - returns `0` when `total_bytes <= 0`
- `format_loading_message` returns `Loading model... <percentage>%`
- `format_model_load_error("sample-box")` returns `Failed to load sample-box. Please try again.`
- `can_interact_with_model` returns `true` only when `has_loaded` is `true` and `has_error` is `false`

Examples:

- `calculate_load_percentage(25, 100)` returns `25`
- `calculate_load_percentage(199, 200)` returns `99`
- `calculate_load_percentage(300, 200)` returns `100`
- `format_loading_message(42)` returns `Loading model... 42%`

## Non-Goals

This project does not include:

- multiple model variants
- lighting controls
- persistence
- AR launch behavior

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same progress, interaction-gating, and error-message rules instead of reimplementing them in the adapter.

Adapters should:

- initiate a remote model download from one valid URL
- show precise progress updates during loading
- hide or disable viewer interaction until the model is ready
- show a friendly error view when the remote URL fails

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns the progress math and user-facing messages and adapter repos must not quietly use different percentages or fallback text.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `0`, intermediate, and `100` progress values
- tests for `total_bytes <= 0`
- tests for the canonical loading message text
- tests for the canonical error message text
- tests for interaction gating before and after load success
- tests for every adapter built in the chosen run that prove it reflects loading, success, and failure states correctly
