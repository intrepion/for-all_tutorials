---
partial_kind: project-spec
project: ar-preview
---

# Spec

Canonical project contract for the `ar-preview` project.

## Goal

Build a small project app that introduces:

- enabling AR mode for a 3D viewer
- selecting between canonical AR scale modes
- graceful unsupported-device fallback behavior
- keeping adapter-side platform wiring thin
- thin adapters

## Canonical AR Defaults

Every tutorial run for this project should treat these values as canonical:

```text
model_src: assets/models/sample-box.glb
default_ar_scale_mode: fixed
supported_ar_scale_modes:
  - fixed
  - auto
canonical_unavailable_message: AR preview is unavailable on this device. Continue using the interactive 3D viewer instead.
```

## Core Logic Contract

The shared core contracts are:

```text
select_ar_scale_mode(mode_name: string) -> ar_scale_mode
can_launch_ar(device_supports_ar: boolean, has_camera_permission: boolean) -> boolean
format_ar_unavailable_message() -> string
```

Canonical behavior:

- `select_ar_scale_mode`:
  - returns `fixed` for `fixed`
  - returns `auto` for `auto`
  - falls back to `fixed` for unknown values
- `can_launch_ar` returns `true` only when both inputs are `true`
- `format_ar_unavailable_message` returns:
  - `AR preview is unavailable on this device. Continue using the interactive 3D viewer instead.`

Examples:

- `select_ar_scale_mode("fixed")` returns `fixed`
- `select_ar_scale_mode("unknown")` returns `fixed`
- `can_launch_ar(true, false)` returns `false`

## Non-Goals

This project does not include:

- persistence of the last chosen AR mode
- multi-model configurators
- remote model downloads
- custom lighting controls

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same AR gating and fallback rules instead of rewording unsupported-device behavior in the adapter.

Adapters should:

- enable AR mode for the chosen viewer technology
- configure the native platform requirements needed for AR launch
- expose at least the canonical `fixed` and `auto` scale modes
- fall back gracefully to the non-AR viewer when AR is unavailable

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns the AR scale fallback and user-facing unavailable message and adapter repos must not silently change that behavior.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `fixed`, `auto`, and unknown scale modes
- tests for AR launch eligibility across all boolean combinations
- tests for the canonical unavailable message
- tests for every adapter built in the chosen run that prove unsupported devices degrade gracefully instead of crashing
