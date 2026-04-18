---
partial_kind: adapter-instructions
project: prism-net-layout
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `prism-net-layout` adapter repo.

### 1. Red: Prove The Adapter Uses The Net Layout

Write a failing adapter test:

```text
given a face-crop plan and prism dimensions
when the flat net is rendered
then each face widget uses the core-generated display size and net position
```

### 2. Green: Render The Flat Prism Net

Make the smallest adapter change that renders the six net panels and hides missing faces deterministically.

### 3. Refactor

Clean up the adapter while keeping all placement math delegated to `build_prism_net_layout`.
