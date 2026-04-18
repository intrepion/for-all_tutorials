---
partial_kind: adapter-instructions
project: prism-face-cropper
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `prism-face-cropper` adapter repo.

### 1. Red: Prove The Adapter Uses The Crop Plan

Write a failing adapter test:

```text
given a normalized face-selection map
when the face preview surface is rendered
then the adapter builds six preview slots from the core-generated crop plan
```

### 2. Green: Render The Face Previews

Make the smallest adapter change that shows one preview slot per canonical face, including a missing-face state.

### 3. Refactor

Clean up the adapter while keeping face-image extraction thin and deterministic.
