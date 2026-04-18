---
partial_kind: adapter-instructions
project: prism-scene-editor
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `prism-scene-editor` adapter repo.

### 1. Red: Prove The Editor Uses The Core Scene

Write a failing adapter test:

```text
given the default prism scene
when the editor changes dimensions or camera controls
then the adapter re-renders the preview from the updated core-owned scene
```

### 2. Green: Build The Scene Editor

Make the smallest adapter change that wires editable controls, validation states, and the live widget prism preview to one core-owned scene document.

### 3. Refactor

Clean up the adapter while keeping the editor and preview synchronized through the core scene contract.
