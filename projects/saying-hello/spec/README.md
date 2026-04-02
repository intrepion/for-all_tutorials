<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Saying Hello](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `saying-hello` project.

## Goal

Build the smallest useful project app that introduces:

- a core logic contract
- test-first implementation
- thin surface adapters
- the repo-wide coverage expectations

## Core Logic Contract

The shared contract is:

```text
greet(name: string) -> string
```

Canonical behavior:

- trim leading and trailing whitespace from `name`
- if the trimmed name is non-empty, return `Hello, <name>!`
- if the trimmed name is empty, return `Hello!`

Examples:

- `greet("Ada")` returns `Hello, Ada!`
- `greet("  Ada  ")` returns `Hello, Ada!`
- `greet("")` returns `Hello!`
- `greet("   ")` returns `Hello!`

## Non-Goals

This first project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- multiple commands or routes

## Surface Expectations

Every tutorial run should adapt the same `greet` behavior instead of redefining it.

A single run only needs one chosen surface and target path.

The supported setup paths for a given moment belong in the curriculum map and setup guides, not in this spec.

## Testing And Coverage Contract

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for non-empty input
- tests for trimming behavior
- tests for empty and whitespace-only input
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly

Coverage policy:

- baseline: `90%` code coverage and `85%` branch coverage
- validation and service logic: `100%` code coverage and `100%` branch coverage

For this project, the core `greet` logic should meet the stricter `100%` / `100%` standard.
