---
partial_kind: adapter-instructions
project: mad-lib
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `mad-lib` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one noun string from that surface
- collect one verb string from that surface
- collect one adjective string from that surface
- collect one adverb string from that surface
- pass all four values to `format_mad_lib`
- return, render, or print the result in the form that surface requires
- keep transport, parsing, and input/output code out of the core story-formatting logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
Enter a noun:
Enter a verb:
Enter an adjective:
Enter an adverb:
```

The adapter must pass the collected words through exactly as entered.

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

