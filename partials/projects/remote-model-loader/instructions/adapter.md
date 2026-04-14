---
partial_kind: adapter-instructions
project: remote-model-loader
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `remote-model-loader` adapter repo.

### 1. Red: Prove Loading Progress Reaches The UI

Write a failing adapter test:

```text
given a remote model download reports partial progress
when the loading callback fires
then the adapter renders the core-owned loading message
and the viewer remains non-interactive
```

### 2. Green: Render Loading Progress

Make the smallest adapter change that uses the core percentage and message helpers.

### 3. Red: Prove Broken URLs Show The Friendly Error

Write a failing adapter test for the broken URL path.

### 4. Green: Render The Error State

Make it pass while keeping the loading path green.
