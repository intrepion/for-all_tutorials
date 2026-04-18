---
partial_kind: adapter-instructions
project: prism-widget-renderer
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `prism-widget-renderer` adapter repo.

### 1. Red: Prove Each Visible Face Is A Widget

Write a failing adapter test:

```text
given a rendered prism scene from the core repo
when the pseudo-3D viewer is rendered
then the visible prism faces appear as separate widgets
and hidden faces are not painted
```

### 2. Green: Build The Widget Prism Renderer

Make the smallest adapter change that renders the prism using layered widgets, transform hints, and hide/show behavior from the core.

### 3. Refactor

Clean up the adapter while keeping transforms, zoom, and face ordering thin.
