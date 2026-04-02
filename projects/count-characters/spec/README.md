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

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same counting and message-formatting behavior instead of redefining it.

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `count_characters` and `format_character_count_message`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for `count_characters` on normal, empty, whitespace-only, and whitespace-preserving inputs
- tests for `format_character_count_message` on zero, one, and many
- tests that `format_character_count_message` preserves the original input exactly
- tests for every adapter built in the chosen run that prove it delegates to the core logic correctly
