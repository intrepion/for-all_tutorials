---
partial_kind: adapter-instructions
project: computing-statistics
---

# Adapter Instructions

Project-specific adapter instruction fragment for the `computing-statistics` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter collects numeric response times until the sentinel value `done`, preserves their order, and delegates the statistical calculations and final report formatting to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- render the equivalent of:
  - `Enter a number:`
- collect numeric response times in order
- stop collecting values when the entered input is exactly `done`
- do not call the core until at least one numeric value has been collected
- pass the collected list to `mean_hundredths`
- pass the same list to `minimum_response_time`
- pass the same list to `maximum_response_time`
- pass the same list to `standard_deviation_hundredths`
- pass the collected list and the calculated values to `format_statistics_report`
- render the returned lines in order
- keep looping, sentinel handling, parsing, transport, and input/output code out of the core statistics logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.

