---
partial_kind: project-spec
project: prism-face-selector
---

# Spec

Canonical project contract for the `prism-face-selector` project.

## Goal

Build a small project app that introduces:

- loading one canonical PNG sheet for a box-like package
- selecting rectangular regions for `front`, `back`, `left`, `right`, `top`, and `bottom`
- converting raw image-space coordinates into normalized rectangles
- assigning one normalized rectangle to each face slot
- keeping the selection contract deterministic and testable
- thin adapters

## Canonical Image Subject

Every tutorial run for this project should use one canonical image equivalent to:

```text
image_id: cereal-box-sheet
src: assets/images/cereal-box-sheet.png
alt_text: Flat cereal box artwork sheet
image_width: 1000
image_height: 700
```

## Canonical Face Slots

Every tutorial run for this project should use these exact face names:

```text
front
back
left
right
top
bottom
```

## Core Logic Contract

The shared core contracts are:

```text
default_face_selection_map() -> face_selection_map
normalize_selection_rect(image_width: number, image_height: number, left: number, top: number, width: number, height: number) -> normalized_rect
assign_face_selection(face_selection_map, face_name: string, normalized_rect) -> face_selection_map
```

Where `normalized_rect` contains:

```text
left
top
width
height
```

Canonical behavior:

- `default_face_selection_map` returns all six face names with `null` selections
- `normalize_selection_rect`:
  - divides `left` and `width` by `image_width`
  - divides `top` and `height` by `image_height`
  - preserves rectangular shape
- `assign_face_selection`:
  - replaces only the requested face slot
  - preserves the other slots exactly
  - rejects unknown face names

Examples:

- `normalize_selection_rect(1000, 700, 300, 140, 200, 315)` returns:
  - `left: 0.3`
  - `top: 0.2`
  - `width: 0.2`
  - `height: 0.45`
- assigning that rectangle to `front` leaves `back`, `left`, `right`, `top`, and `bottom` unchanged

## Non-Goals

This project does not include:

- actual image cropping
- prism dimensions
- pseudo-3D rendering
- zoom and orbit controls
- persistence

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same face-slot and normalization behavior instead of redefining it in the adapter.

Adapters should:

- load one PNG image
- let the user draw or adjust one rectangle at a time
- label the six face slots clearly
- convert pixel-space selection rectangles through `normalize_selection_rect`
- persist the working face-selection map only in adapter state unless a later project adds persistence

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `default_face_selection_map`, `normalize_selection_rect`, and `assign_face_selection`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `default_face_selection_map` returns all six canonical face slots with empty selections
- a test that `normalize_selection_rect` converts canonical pixel coordinates into normalized values
- a test that `assign_face_selection` updates only one face slot
- a test that `assign_face_selection` rejects or reports unknown face names
- tests for every adapter built in the chosen run that prove it loads the canonical image, lets the user assign a face slot, and stores normalized rectangles rather than raw pixels in the core contract
