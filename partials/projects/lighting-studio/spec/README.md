---
partial_kind: project-spec
project: lighting-studio
---

# Spec

Canonical project contract for the `lighting-studio` project.

## Goal

Build a small project app that introduces:

- a canonical 3D model scene with adjustable lighting
- exposure and shadow tuning
- choosing one named environment image from a small catalog
- deterministic clamping and environment mapping logic in the core
- thin adapters

## Canonical Lighting Inputs

Every tutorial run for this project should treat these values as canonical:

```text
base_model_src: assets/models/sample-box.glb
default_environment_id: neutral-studio
default_environment_src: assets/environments/neutral-studio.hdr
default_exposure: 1.0
default_shadow_intensity: 0.6
```

Supported environment ids:

- `neutral-studio` -> `assets/environments/neutral-studio.hdr`
- `warm-studio` -> `assets/environments/warm-studio.hdr`
- `cool-studio` -> `assets/environments/cool-studio.hdr`

## Core Logic Contract

The shared core contracts are:

```text
clamp_lighting_controls(exposure: number, shadow_intensity: number, environment_id: string) -> lighting_controls
build_lighting_scene(lighting_controls) -> viewer_scene_config
```

Canonical behavior:

- `clamp_lighting_controls`:
  - clamps `exposure` into the inclusive range `0.0` to `2.0`
  - clamps `shadow_intensity` into the inclusive range `0.0` to `1.0`
  - maps unknown `environment_id` values to `neutral-studio`
- `build_lighting_scene`:
  - uses `assets/models/sample-box.glb` as the source model
  - applies the clamped exposure and shadow intensity values
  - maps the chosen environment id to its canonical `.hdr` asset path

Examples:

- `clamp_lighting_controls(1.2, 0.5, "warm-studio")` keeps all three values unchanged
- `clamp_lighting_controls(3.1, -0.2, "unknown")` returns:
  - `exposure: 2.0`
  - `shadow_intensity: 0.0`
  - `environment_id: neutral-studio`

## Non-Goals

This project does not include:

- remote model loading
- progress indicators
- multiple model variants
- persistence of slider state
- AR launch behavior

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same lighting rules instead of reimplementing control clamping in the adapter.

Adapters should:

- render controls for exposure and shadow intensity
- render a selector for the named environment image catalog
- delegate control validation and clamping to the core
- apply the returned scene configuration to the viewer

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns the valid lighting ranges and environment mapping rules and adapter repos must not silently use different ranges or fallback ids.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for clamping high and low exposure values
- tests for clamping high and low shadow intensity values
- tests for mapping each supported environment id
- tests for falling back unknown environment ids to `neutral-studio`
- tests for every adapter built in the chosen run that prove it applies the core-owned lighting values
