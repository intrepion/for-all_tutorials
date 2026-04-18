---
partial_kind: core-instructions
project: prism-net-layout
---

# Core Instructions

Project-specific core instruction fragment for the `prism-net-layout` core repo.

### 1. Red: Size The Front Face

Start with a failing test:

```text
given prism dimensions 240, 360, 90
when face_display_size is called for front
then the returned size is 240 by 360
```

### 2. Green: Make The First Size Test Pass

Create the smallest production code that returns the canonical front-face size.

### 3. Red: Add Side And Top Sizing

Write the next failing tests:

```text
given prism dimensions 240, 360, 90
then left is 90 by 360
and top is 240 by 90
```

### 4. Green: Make The Remaining Size Tests Pass

Make them pass while keeping the earlier test green.

### 5. Red: Add The Net Layout

Write a failing test that requires a canonical six-face 2D net with missing faces preserved as hidden panels.

### 6. Green: Make The Net Layout Pass

Make it pass while keeping the earlier tests green.
