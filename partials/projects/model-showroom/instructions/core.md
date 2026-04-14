---
partial_kind: core-instructions
project: model-showroom
---

# Core Instructions

Project-specific core instruction fragment for the `model-showroom` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical showroom scene:

```text
given no user input
when build_showroom_scene is called
then the returned src is assets/models/sample-box.glb
and the camera orbit is 45deg 55deg 2.5m
```

Suggested generic test name:

```text
build_showroom_scene_returns_the_canonical_sample_box_scene
```

### 2. Green: Make The First Test Pass

Create the smallest production code that returns the canonical source path and orbit.

### 3. Red: Add Viewer Control Defaults

Write the next failing test:

```text
given the canonical showroom scene
then camera_controls is true
and auto_rotate is true
```

### 4. Green: Make The Controls Test Pass

Make it pass while keeping the earlier test green.

### 5. Red: Add Alt Text

Write a failing test that requires the exact alt text `Simple sample box`.

### 6. Green: Make The Alt Text Test Pass

Make it pass while keeping the earlier tests green.
