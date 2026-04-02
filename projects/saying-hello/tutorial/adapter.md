<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Saying Hello](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `saying-hello` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- accept the smallest useful input for that surface
- pass that input to `greet`
- return, render, or print the result in the form that surface requires
- keep transport, parsing, and input/output code out of the core greeting logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
