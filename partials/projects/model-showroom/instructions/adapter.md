---
partial_kind: adapter-instructions
project: model-showroom
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `model-showroom` adapter repo.

### 1. Red: Prove The Adapter Uses The Core Scene

Write a failing adapter test:

```text
given the canonical viewer scene from the core repo
when the showroom widget or page is rendered
then the model viewer receives the same src, orbit, and control flags
```

### 2. Green: Render The Canonical Scene

Make the smallest adapter change that renders the one local model and delegates all scene defaults to `build_showroom_scene`.

### 3. Refactor

Clean up the adapter while keeping the viewer wiring thin.
