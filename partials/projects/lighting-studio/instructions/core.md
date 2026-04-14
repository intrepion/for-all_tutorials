---
partial_kind: core-instructions
project: lighting-studio
---

# Core Instructions

Project-specific core instruction fragment for the `lighting-studio` core repo.

### 1. Red: Clamp The First Lighting Values

Start with this failing test:

```text
given exposure 3.1 and shadow_intensity -0.2 and environment_id unknown
when clamp_lighting_controls is called
then the returned values are 2.0, 0.0, and neutral-studio
```

### 2. Green: Make The Clamp Test Pass

Create the smallest production code that clamps and falls back correctly.

### 3. Red: Map A Known Environment

Write a failing test that requires `warm-studio` to map to `assets/environments/warm-studio.hdr`.

### 4. Green: Make The Environment Test Pass

Make it pass while keeping the earlier clamp behavior green.

### 5. Red: Build The Full Lighting Scene

Write a failing test that requires the canonical model source plus the chosen lighting values.

### 6. Green: Make The Scene Test Pass

Make it pass while keeping the earlier tests green.
