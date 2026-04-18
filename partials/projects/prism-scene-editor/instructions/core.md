---
partial_kind: core-instructions
project: prism-scene-editor
---

# Core Instructions

Project-specific core instruction fragment for the `prism-scene-editor` core repo.

### 1. Red: Build The Default Scene

Start with a failing test:

```text
given no prior edits
when build_default_prism_scene is called
then the scene uses cereal-box-sheet
and dimensions 240, 360, 90
and camera 35, -20, 1.0
```

### 2. Green: Make The Default Scene Pass

Create the smallest production code that returns the canonical default scene.

### 3. Red: Add Immutable Scene Updates

Write the next failing tests:

```text
given the default scene
when dimensions are updated
then only the dimension fields change

when camera values are updated
then zoom is clamped into the canonical range
```

### 4. Green: Make Scene Updates Pass

Make them pass while keeping the earlier test green.

### 5. Red: Add Scene Serialization

Write a failing test that requires deterministic JSON output for the full prism scene document.

### 6. Green: Make Scene Serialization Pass

Make it pass while keeping the earlier tests green.
