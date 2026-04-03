---
partial_kind: adapter-instructions
project: anagram-checker
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `anagram-checker` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one first-input string from that surface
- collect one second-input string from that surface
- pass both values to `are_anagrams`
- pass the original values and resulting boolean to `format_anagram_result`
- return, render, or print the formatted result in the form that surface requires
- keep transport, parsing, and input/output code out of the core anagram logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
Enter two strings and I'll tell you if they
are anagrams:
Enter the first string:
Enter the second string:
```

The adapter must pass both collected strings through exactly as entered.

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

