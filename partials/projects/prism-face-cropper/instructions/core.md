---
partial_kind: core-instructions
project: prism-face-cropper
---

# Core Instructions

Project-specific core instruction fragment for the `prism-face-cropper` core repo.

### 1. Red: Validate The Face Map

Start with a failing test:

```text
given a face-selection map with front and top selected
when validate_face_selection_map is called
then front and top are reported present
and the other four faces are reported missing
```

### 2. Green: Make Validation Pass

Create the smallest production code that reports present and missing face slots correctly.

### 3. Red: Add Crop Plan Generation

Write the next failing test:

```text
given the canonical front rectangle and a 1000 by 700 image
when build_face_crop_plan is called
then the front crop is 300, 140, 200, 315
```

### 4. Green: Make Crop Planning Pass

Make it pass while keeping the earlier test green.

### 5. Red: Add Summary Formatting

Write a failing test that requires one summary line per canonical face slot in canonical order.

### 6. Green: Make The Summary Test Pass

Make it pass while keeping the earlier tests green.
