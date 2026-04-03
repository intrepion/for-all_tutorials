<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Password Strength Indicator](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `password-strength-indicator` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one password string from that surface
- pass the password to `calculate_password_strength_score`
- pass the resulting score to `classify_password_strength`
- pass the password and resulting strength to `format_password_strength_message`
- return, render, or print the formatted result in the form that surface requires
- keep transport, parsing, and input/output code out of the core password-scoring and classification logic

For prompt-driven adapters, the prompt should be equivalent to:

```text
Enter a password:
```

The adapter must pass the collected password through exactly as entered.

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
