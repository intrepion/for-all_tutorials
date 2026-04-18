---
partial_kind: project-spec
project: prism-face-cropper
---

# Spec

Canonical project contract for the `prism-face-cropper` project.

## Goal

Build a small project app that introduces:

- validating a normalized face-selection map
- turning face selections into a deterministic crop plan
- preserving missing-face information instead of guessing
- preparing independent face assets for later 2D and pseudo-3D rendering
- thin adapters

## Canonical Input Subject

Every tutorial run for this project should assume:

- the canonical `cereal-box-sheet` PNG from `prism-face-selector`
- a face-selection map equivalent to one produced by that project

## Core Logic Contract

The shared core contracts are:

```text
validate_face_selection_map(face_selection_map) -> face_selection_validation
build_face_crop_plan(image_width: number, image_height: number, face_selection_map) -> face_crop_plan[]
format_face_crop_summary(face_crop_plan[]) -> string[]
```

Where each `face_crop_plan` contains:

```text
face_name
pixel_left
pixel_top
pixel_width
pixel_height
is_missing
```

Canonical behavior:

- `validate_face_selection_map` reports which of the six canonical face slots are present or missing
- `build_face_crop_plan`:
  - converts normalized face rectangles back into integer pixel-space rectangles
  - preserves one crop plan per canonical face slot
  - marks slots with no selection as `is_missing: true`
- `format_face_crop_summary` returns one line per face in canonical face order

Examples:

- if `front` is `0.3, 0.2, 0.2, 0.45` on a `1000 x 700` image, the crop plan is:
  - `pixel_left: 300`
  - `pixel_top: 140`
  - `pixel_width: 200`
  - `pixel_height: 315`
- if `bottom` is unassigned, the `bottom` crop plan still exists and `is_missing` is `true`

## Non-Goals

This project does not include:

- prism dimensions
- pseudo-3D transforms
- orbit controls
- persistence

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same crop-plan contract instead of letting the adapter invent its own face metadata shape.

Adapters should:

- load the canonical source image and normalized face-selection map
- build one visible preview per face slot
- hide or mark missing faces without fabricating crops
- use the crop plan as the source of truth for later rendering

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `validate_face_selection_map`, `build_face_crop_plan`, and `format_face_crop_summary`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `validate_face_selection_map` reports present and missing faces correctly
- a test that `build_face_crop_plan` converts canonical normalized rectangles back into integer pixel-space crops
- a test that `build_face_crop_plan` preserves missing faces as missing instead of silently removing them
- a test that `format_face_crop_summary` preserves canonical face order
- tests for every adapter built in the chosen run that prove it renders six face-preview slots and uses the core-generated crop plan as the source of truth
