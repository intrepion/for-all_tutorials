---
partial_kind: adapter-instructions
project: picking-a-winner
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `picking-a-winner` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter collects contestant names until a blank entry, chooses a random winner position within the supported range, and delegates winner lookup and final formatting to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- render the equivalent of:
  - `Enter a name:`
- collect contestant names in order
- stop collecting names when the entered name is blank
- pass the collected list to `contestant_count`
- choose a random whole number between `1` and the returned count, inclusive
- pass the contestant list and chosen number to `winner_name_by_choice_number`
- pass the returned winner name to `format_winner_message`
- render, return, or print the formatted winner message in the form that surface requires
- keep randomness, looping, transport, and input/output code out of the core winner-selection logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

