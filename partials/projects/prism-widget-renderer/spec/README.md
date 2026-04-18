---
partial_kind: project-spec
project: prism-widget-renderer
---

# Spec

Canonical project contract for the `prism-widget-renderer` project.

## Goal

Build a small project app that introduces:

- rendering six face widgets as a pseudo-3D rectangular prism
- rotating the prism around a deterministic orbit model
- zooming in and out within a bounded range
- hiding faces that should not be visible from the current view
- sorting visible faces into a deterministic render order
- thin adapters

## Canonical Camera Subject

Every tutorial run for this project should use camera values equivalent to:

```text
yaw_degrees: 35
pitch_degrees: -20
zoom: 1.0
```

## Core Logic Contract

The shared core contracts are:

```text
clamp_prism_zoom(zoom: number) -> number
build_prism_camera(yaw_degrees: number, pitch_degrees: number, zoom: number) -> prism_camera
is_face_visible(face_name: string, prism_camera) -> boolean
build_rendered_prism_faces(prism_net_layout_item[], prism_camera) -> rendered_prism_face[]
```

Where each `rendered_prism_face` contains:

```text
face_name
transform_hint
z_index
is_hidden
```

Canonical behavior:

- `clamp_prism_zoom` keeps zoom within a small bounded range such as `0.5` through `2.0`
- `build_prism_camera` preserves exact yaw and pitch values after zoom clamping
- `is_face_visible` hides the rear-facing panels for a given camera position
- `build_rendered_prism_faces`:
  - preserves canonical face order before sorting
  - returns deterministic `z_index` values for visible faces
  - marks invisible faces as `is_hidden: true`

Examples:

- `clamp_prism_zoom(0.2)` returns `0.5`
- `clamp_prism_zoom(3.5)` returns `2.0`
- with the canonical camera, the `front`, `right`, and `top` faces are visible

## Non-Goals

This project does not include:

- true mesh rendering
- texture baking
- lighting simulation
- persistence
- AR

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same pseudo-3D visibility and ordering behavior instead of hiding or transforming faces ad hoc in the adapter.

Adapters should:

- render each prism face as its own widget
- apply the core-owned transform hints to those widgets
- hide faces marked hidden by the core
- expose rotate and zoom interactions without redefining the camera rules

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `clamp_prism_zoom`, `build_prism_camera`, `is_face_visible`, and `build_rendered_prism_faces`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `clamp_prism_zoom` enforces the canonical zoom bounds
- a test that the canonical camera keeps `front`, `right`, and `top` visible
- a test that `build_rendered_prism_faces` marks hidden faces as hidden
- a test that `build_rendered_prism_faces` produces deterministic render ordering for visible faces
- tests for every adapter built in the chosen run that prove each visible prism side is rendered as its own widget with transform-based positioning and hiding
