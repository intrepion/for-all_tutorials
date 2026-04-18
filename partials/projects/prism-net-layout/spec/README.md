---
partial_kind: project-spec
project: prism-net-layout
---

# Spec

Canonical project contract for the `prism-net-layout` project.

## Goal

Build a small project app that introduces:

- defining a rectangular prism with independent width, height, and depth
- mapping each face crop to a 2D net position
- sizing face widgets from prism dimensions
- treating missing faces as hidden net panels instead of guessed textures
- thin adapters

## Canonical Prism Subject

Every tutorial run for this project should use prism dimensions equivalent to:

```text
width: 240
height: 360
depth: 90
```

## Core Logic Contract

The shared core contracts are:

```text
build_prism_dimensions(width: number, height: number, depth: number) -> prism_dimensions
face_display_size(face_name: string, prism_dimensions) -> face_size
build_prism_net_layout(face_crop_plan[], prism_dimensions) -> prism_net_layout_item[]
```

Where each `prism_net_layout_item` contains:

```text
face_name
x
y
display_width
display_height
is_hidden
```

Canonical behavior:

- `build_prism_dimensions` preserves the exact width, height, and depth values
- `face_display_size` returns:
  - `front` and `back`: `width x height`
  - `left` and `right`: `depth x height`
  - `top` and `bottom`: `width x depth`
- `build_prism_net_layout`:
  - places the six faces into one deterministic 2D net
  - preserves canonical face order in the output list
  - marks missing face crops as `is_hidden: true`

Examples:

- `face_display_size("front", 240x360x90)` returns `240 x 360`
- `face_display_size("left", 240x360x90)` returns `90 x 360`
- `face_display_size("top", 240x360x90)` returns `240 x 90`

## Non-Goals

This project does not include:

- perspective transforms
- orbit controls
- depth sorting
- persistence

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same prism-dimension and net-layout behavior instead of hard-coding layout math directly in the adapter.

Adapters should:

- accept one face-crop plan and one prism-dimensions value
- render the six canonical faces in one flat net
- hide missing faces deterministically
- keep dimension controls separate from the layout math

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `build_prism_dimensions`, `face_display_size`, and `build_prism_net_layout`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `face_display_size` returns `width x height` for `front`
- a test that `face_display_size` returns `depth x height` for `left`
- a test that `face_display_size` returns `width x depth` for `top`
- a test that `build_prism_net_layout` preserves canonical face order
- a test that `build_prism_net_layout` hides missing faces without dropping them
- tests for every adapter built in the chosen run that prove it renders a flat net using the core-generated dimensions and layout items
