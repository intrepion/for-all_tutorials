---
partial_kind: project-spec
project: picking-a-winner
---

# Spec

Canonical project contract for the `picking-a-winner` project.

## Goal

Build a small project app that introduces:

- collecting a dynamic list of contestant names
- using a blank entry as the sentinel that ends collection
- selecting one contestant by numeric position
- separating winner lookup and final formatting from random number generation and input/output
- test-first winner-selection and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
contestant_count(contestant_names: string[]) -> integer
winner_name_by_choice_number(contestant_names: string[], choice_number: integer) -> string | null
format_winner_message(winner_name: string) -> string
```

Canonical behavior:

- `contestant_count` returns the number of contestant names in the supplied list
- `winner_name_by_choice_number`:
  - uses `1`-based indexing
  - returns the contestant name at the requested position
  - returns `null` when `choice_number` is less than `1`
  - returns `null` when `choice_number` is greater than the number of contestants
  - does not trim, normalize, or rewrite contestant names
  - preserves contestant names exactly as supplied
- `format_winner_message` returns:
  - `The winner is... <winner_name>.`

Examples:

- `contestant_count(["Homer", "Bart", "Maggie", "Lisa", "Moe"])` returns `5`
- `winner_name_by_choice_number(["Homer", "Bart", "Maggie", "Lisa", "Moe"], 3)` returns `Maggie`
- `winner_name_by_choice_number(["Homer", "Bart", "Maggie", "Lisa", "Moe"], 0)` returns `null`
- `winner_name_by_choice_number(["Homer", "Bart", "Maggie", "Lisa", "Moe"], 6)` returns `null`
- `format_winner_message("Maggie")` returns `The winner is... Maggie.`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- random number generation in the core
- deduplicating contestant names
- trimming or validating blank contestant names in the core
- replay flow
- weighted drawings or multiple winners

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same contestant-count, winner-lookup, and winner-formatting behavior instead of redefining it.

Adapters should:

- prompt for contestant names repeatedly
- stop collecting names when the entered name is blank
- keep the collected contestant names in entry order
- call `contestant_count` to determine the upper bound for random selection
- choose a random whole number between `1` and that count, inclusive
- pass the contestant list and the chosen number to `winner_name_by_choice_number`
- pass the returned winner name to `format_winner_message`
- render, return, or print the formatted winner message in the form that surface requires

For prompt-driven adapters, the contestant prompt should be equivalent to:

```text
Enter a name:
```

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `contestant_count`, `winner_name_by_choice_number`, and `format_winner_message`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `contestant_count` returns `5` for the canonical contestant list
- a test that `winner_name_by_choice_number` returns `Maggie` for the canonical list and choice `3`
- tests that `winner_name_by_choice_number` returns `null` for out-of-range values such as `0` and `6`
- a test that `format_winner_message` preserves the exact canonical sentence
- tests for every adapter built in the chosen run that prove it stops on a blank entry, uses the contestant count as the random upper bound, and delegates winner lookup to the core logic correctly

