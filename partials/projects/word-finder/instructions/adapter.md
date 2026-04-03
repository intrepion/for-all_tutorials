---
partial_kind: adapter-instructions
project: word-finder
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `word-finder` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter reads a source file, delegates the text transformation to the core logic, and writes the transformed contents to a new output file.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- read the source file contents
- pass the contents to `replace_exact_word_utilize_with_use`
- write the returned transformed contents to a new output file
- keep file input/output code out of the core text-transformation logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

