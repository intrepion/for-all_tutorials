---
partial_kind: core-instructions
project: prism-face-selector
---

# Core Instructions

Project-specific core instruction fragment for the `prism-face-selector` core repo.

### 1. Red: Write The First Failing Test

Start with the face-slot map:

```text
given no prior selections
when default_face_selection_map is called
then the returned map contains front, back, left, right, top, and bottom
and every slot is empty
```

Suggested generic test name:

```text
default_face_selection_map_returns_all_empty_canonical_face_slots
```

### 2. Green: Make The First Test Pass

Create the smallest production code that returns the canonical six-slot empty map.

### 3. Red: Add Rectangle Normalization

Write the next failing test:

```text
given a 1000 by 700 image
when normalize_selection_rect is called with 300, 140, 200, 315
then the normalized rectangle is 0.3, 0.2, 0.2, 0.45
```

### 4. Green: Make The Normalization Test Pass

Make it pass while keeping the earlier test green.

### 5. Red: Add Face Assignment

Write a failing test that proves `assign_face_selection` updates only `front` and preserves the other five slots.

### 6. Green: Make Face Assignment Pass

Make it pass while keeping the earlier tests green.
