<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Counting the Number of Characters](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `count-characters` project.

## Goal

Build a small project app that introduces:

- deriving output from one text input
- exact string preservation
- test-first counting and formatting logic in separate core functions
- thin surface adapters
- the repo-wide coverage expectations

## Core Logic Contract

The shared core contracts are:

```text
count_characters(input: string) -> integer
format_character_count_message(input: string, count: integer) -> string
```

Canonical behavior:

- count every character in plain-text ASCII input exactly as entered
- count leading, trailing, internal, and whitespace-only input exactly as entered
- include spaces, tabs, and punctuation in the count
- do not trim, collapse, or normalize whitespace at any layer
- preserve `input` exactly in the echoed output
- return `<input> has no characters.` when the count is `0`
- return `<input> has 1 character.` when the count is `1`
- return `<input> has <count> characters.` when the count is greater than `1`
- build the final response by passing the result of `count_characters` into `format_character_count_message`

Examples:

- `count_characters("Homer")` returns `5`
- `count_characters("  Hi")` returns `4`
- `count_characters("   ")` returns `3`
- `count_characters("")` returns `0`
- `format_character_count_message("", 0)` returns ` has no characters.`
- `format_character_count_message("X", 1)` returns `X has 1 character.`
- `format_character_count_message("Homer", 5)` returns `Homer has 5 characters.`
- `format_character_count_message("  Hi", 4)` preserves the two leading spaces in the echoed output

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- advanced Unicode grapheme-cluster counting
- multiple inputs, history, or editing workflows

## Surface Expectations

Every tutorial run should adapt the same counting and message-formatting behavior instead of redefining it.

A single run only needs one chosen surface and target path.

The supported setup paths for a given moment belong in the curriculum map and setup guides, not in this spec.

## Output Repository Expectations

The code produced from this tutorial should live outside this curriculum repo.

A normal run should separate responsibilities like this:

- one core library repo owns the `count_characters` and `format_character_count_message` rules
- each adapter repo depends on that core library
- adapter repos must not reimplement the counting or formatting rules

## Testing And Coverage Contract

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `count_characters` on normal, empty, whitespace-only, and whitespace-preserving inputs
- tests for `format_character_count_message` on zero, one, and many
- tests that `format_character_count_message` preserves the original input exactly
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly

Coverage policy:

- baseline: `90%` code coverage and `85%` branch coverage
- validation and service logic: `100%` code coverage and `100%` branch coverage

For this project, the core `count_characters` and `format_character_count_message` logic should meet the stricter `100%` / `100%` standard.
