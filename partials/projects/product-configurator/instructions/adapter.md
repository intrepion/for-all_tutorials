---
partial_kind: adapter-instructions
project: product-configurator
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `product-configurator` adapter repo.

### 1. Red: Prove Variant Selection Rebuilds The Scene

Write a failing adapter test:

```text
given the user chooses tall-box
when the configurator rebuilds the viewer
then the adapter applies the tall-box source, orbit, and exposure from the core
```

### 2. Green: Render Variant Switching

Make the smallest adapter change that swaps the viewer scene by exact variant id.

### 3. Refactor

Keep the selection UI thin and leave catalog lookup in the core.
