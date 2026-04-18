---
partial_kind: adapter-instructions
project: prism-face-selector
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `prism-face-selector` adapter repo.

### 1. Red: Prove The Adapter Uses Normalized Face Selections

Write a failing adapter test:

```text
given the canonical cereal-box image
when the user selects a rectangle for the front face
then the adapter stores a normalized rectangle in the core-owned face selection map
```

### 2. Green: Build The Face Selection Surface

Make the smallest adapter change that renders the image, lets the user choose a face slot, and routes the selected rectangle through `normalize_selection_rect` and `assign_face_selection`.

### 3. Refactor

Clean up the adapter while keeping the drawing and face-slot wiring thin.
