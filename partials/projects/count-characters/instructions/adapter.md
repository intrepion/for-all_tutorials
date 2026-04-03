---
partial_kind: adapter-instructions
project: count-characters
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `count-characters` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one input string from that surface
- pass that input to `count_characters`
- pass the original input plus the returned count to `format_character_count_message`
- return, render, or print the result in the form that surface requires
- keep transport, parsing, and input/output code out of the core counting logic

For prompt-driven adapters, the prompt should be equivalent to:

```text
What is the input string?
```

The adapter must pass the collected input through exactly as entered, without trimming or normalizing whitespace.

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

