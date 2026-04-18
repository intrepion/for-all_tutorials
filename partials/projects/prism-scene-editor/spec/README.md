---
partial_kind: project-spec
project: prism-scene-editor
---

# Spec

Canonical project contract for the `prism-scene-editor` project.

## Goal

Build a small project app that introduces:

- editing a full rectangular-prism scene in one place
- updating prism dimensions, face assignments, and camera state live
- validating incomplete scenes without crashing the preview
- serializing one deterministic scene document for future restore or export
- thin adapters

## Canonical Scene Subject

Every tutorial run for this project should use a scene equivalent to:

```text
image_id: cereal-box-sheet
prism_dimensions: 240 x 360 x 90
camera: yaw 35, pitch -20, zoom 1.0
```

## Core Logic Contract

The shared core contracts are:

```text
build_default_prism_scene() -> prism_scene
update_prism_scene_dimensions(prism_scene, width: number, height: number, depth: number) -> prism_scene
update_prism_scene_camera(prism_scene, yaw_degrees: number, pitch_degrees: number, zoom: number) -> prism_scene
serialize_prism_scene(prism_scene) -> string
```

Where `prism_scene` contains:

```text
image_id
face_selection_map
prism_dimensions
camera
```

Canonical behavior:

- `build_default_prism_scene` returns the canonical image id, dimensions, and camera values
- scene updates return new scene values instead of mutating the original
- `update_prism_scene_camera` uses the same zoom clamping rules as `prism-widget-renderer`
- `serialize_prism_scene` returns a deterministic JSON document that preserves the exact scene data

Examples:

- `build_default_prism_scene().prism_dimensions.width` returns `240`
- `update_prism_scene_dimensions(scene, 300, 420, 120)` returns a scene with only those dimension values changed
- `update_prism_scene_camera(scene, 40, -10, 4.0)` stores zoom as `2.0`

## Non-Goals

This project does not include:

- true 3D model export
- lighting simulation
- AR
- collaborative multi-user editing

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same scene-document and update behavior instead of letting the adapter invent ad hoc state shapes.

Adapters should:

- expose dimension and camera controls
- expose face-slot reassignment or reset controls
- render a live pseudo-3D preview from the core-owned scene data
- show validation or missing-face states without breaking the preview

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `build_default_prism_scene`, `update_prism_scene_dimensions`, `update_prism_scene_camera`, and `serialize_prism_scene`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `build_default_prism_scene` returns the canonical image id, dimensions, and camera
- a test that updating dimensions changes only the prism-dimension fields
- a test that updating camera values clamps zoom and preserves yaw and pitch
- a test that `serialize_prism_scene` preserves the exact scene data in a deterministic JSON document
- tests for every adapter built in the chosen run that prove it edits a live scene and keeps the preview in sync with the core-owned scene document
