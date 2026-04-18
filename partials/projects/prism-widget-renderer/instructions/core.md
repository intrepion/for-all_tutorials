---
partial_kind: core-instructions
project: prism-widget-renderer
---

# Core Instructions

Project-specific core instruction fragment for the `prism-widget-renderer` core repo.

### 1. Red: Clamp Zoom

Start with a failing test:

```text
given zoom values below 0.5 and above 2.0
when clamp_prism_zoom is called
then the returned value stays inside that range
```

### 2. Green: Make Zoom Clamping Pass

Create the smallest production code that clamps zoom into the canonical range.

### 3. Red: Add Canonical Visibility

Write the next failing test:

```text
given the canonical camera
then front, right, and top are visible
and back is hidden
```

### 4. Green: Make Visibility Pass

Make it pass while keeping the earlier test green.

### 5. Red: Add Render Ordering

Write a failing test that requires visible faces to receive deterministic render-order values and hidden faces to stay hidden.

### 6. Green: Make Render Ordering Pass

Make it pass while keeping the earlier tests green.
