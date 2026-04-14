---
partial_kind: adapter-instructions
project: lighting-studio
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `lighting-studio` adapter repo.

### 1. Red: Prove Slider Changes Flow Through The Core

Write a failing adapter test:

```text
given the user selects warm-studio and changes exposure and shadow controls
when the scene is rebuilt
then the adapter applies the clamped values returned by the core
```

### 2. Green: Wire The Controls

Make the smallest adapter change that routes slider and selector values through `clamp_lighting_controls` and `build_lighting_scene`.

### 3. Refactor

Keep the UI logic small and leave validation in the core.
