<!-- breadcrumbs:start -->
[for-all_tutorials](../../README.md) / [Specs](../README.md) / Saying Hello
<!-- breadcrumbs:end -->

# Saying Hello

Canonical project contract for the first project app in the curriculum.

## Goal

Build the smallest useful project app that introduces:

- a core logic contract
- test-first implementation
- thin surface adapters
- the repo-wide coverage expectations

## Core Logic Contract

The shared library-level contract is:

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

Every surface tutorial should adapt the same `greet` behavior instead of redefining it.

Initial required surface:

- `command-line/all`

Future optional surfaces may include:

- `web/back-end`
- `web/front-end`
- `web/full-stack`
- `desktop/all`
- `desktop/macos`
- `desktop/linux`
- `desktop/windows`
- `mobile/all`
- `mobile/ios`
- `mobile/android`

## Testing And Coverage Contract

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for non-empty input
- tests for trimming behavior
- tests for empty and whitespace-only input
- tests for every surface adapter that prove it delegates to the core logic correctly

Coverage policy:

- baseline: `90%` code coverage and `85%` branch coverage
- validation and service logic: `100%` code coverage and `100%` branch coverage

For this project, the core `greet` logic should meet the stricter `100%` / `100%` standard.
