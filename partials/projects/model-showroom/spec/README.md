---
partial_kind: project-spec
project: model-showroom
---

# Spec

Canonical project contract for the `model-showroom` project.

## Goal

Build a small project app that introduces:

- loading one local 3D model asset into an interactive viewer
- applying a deterministic initial camera orbit
- enabling user camera controls and automatic rotation
- keeping scene defaults in tested core logic
- thin adapters

## Canonical Viewer Subject

Every tutorial run for this project should use one canonical local model equivalent to:

```text
model_id: sample-box
src: assets/models/sample-box.glb
alt_text: Simple sample box
camera_orbit: 45deg 55deg 2.5m
camera_controls: true
auto_rotate: true
```

## Core Logic Contract

The shared core contract is:

```text
build_showroom_scene() -> viewer_scene_config
```

Where `viewer_scene_config` contains:

```text
model_id
src
alt_text
camera_orbit
camera_controls
auto_rotate
```

Canonical behavior:

- `build_showroom_scene` returns the canonical `sample-box` configuration
- `src` points to one local `.glb` asset path
- `alt_text` is `Simple sample box`
- `camera_orbit` is exactly `45deg 55deg 2.5m`
- `camera_controls` is `true`
- `auto_rotate` is `true`

Examples:

- `build_showroom_scene().model_id` returns `sample-box`
- `build_showroom_scene().camera_orbit` returns `45deg 55deg 2.5m`
- `build_showroom_scene().auto_rotate` returns `true`

## Non-Goals

This project does not include:

- remote model loading
- progress indicators
- multiple models or dynamic switching
- custom lighting controls
- persistence
- AR launch behavior

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same canonical showroom scene instead of redefining the default orbit or model asset.

Adapters should:

- load one local `.glb` asset
- apply the canonical initial orbit before first render
- expose interactive camera controls
- keep automatic rotation enabled until the user interacts

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns the canonical showroom scene values and adapter repos must not hard-code different defaults behind the core's back.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for the canonical model id, source path, and alt text
- tests for the exact camera orbit string
- tests for camera controls and auto-rotate flags
- tests for every adapter built in the chosen run that prove it applies the core-owned scene values
