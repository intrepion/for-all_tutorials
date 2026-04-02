<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Area of a Rectangular Room](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `area-of-a-rectangular-room` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one length input string from that surface
- collect one width input string from that surface
- parse those strings into numeric values that match the core contract
- pass the parsed numbers to `calculate_room_area`
- pass the original numeric values plus the returned area values to `format_room_area_report`
- return, render, or print the four lines in the form that surface requires
- keep transport, parsing, and input/output code out of the core area and conversion logic

For prompt-driven adapters, the prompts should be equivalent to:

```text
What is the length of the room in feet?
What is the width of the room in feet?
```

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
