---
partial_kind: adapter-instructions
project: word-frequency-finder
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `word-frequency-finder` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter reads a source text file, delegates tokenization, counting, and histogram formatting to the core logic, and renders the returned lines in order.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- read the source file contents
- pass the contents to `split_text_into_words`
- pass the returned words to `count_word_frequencies`
- pass the resulting entries to `format_word_frequency_histogram`
- render the returned lines in order
- keep file input/output code out of the core frequency-counting logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

