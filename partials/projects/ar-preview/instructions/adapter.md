---
partial_kind: adapter-instructions
project: ar-preview
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `ar-preview` adapter repo.

### 1. Red: Prove Unsupported Devices Fall Back Cleanly

Write a failing adapter test:

```text
given AR is unavailable
when the adapter renders the preview
then the standard interactive 3D viewer still works
and the canonical unavailable message is shown
```

### 2. Green: Add The Fallback Path

Make the smallest adapter change that preserves a non-AR experience on unsupported devices.

### 3. Red: Prove Supported Devices Can Launch AR

Write a failing adapter test for the happy path.

### 4. Green: Wire The AR Launch Path

Make it pass while keeping the fallback path green.
