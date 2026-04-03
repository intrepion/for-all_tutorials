<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Magic 8 Ball](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `magic-8-ball` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter collects the user question, chooses a random response index within the supported range, and delegates response lookup to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- render the equivalent of:
  - `What's your question?`
- collect the raw question input
- call `magic_8_ball_response_count`
- choose a random whole number between `1` and the returned count, inclusive
- pass that chosen number to `magic_8_ball_response`
- render, return, or print the resulting response in the form that surface requires
- keep randomness, transport, and input/output code out of the core response-catalog logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
