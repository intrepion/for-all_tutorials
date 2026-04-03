<!-- breadcrumbs:start -->
[for-all_tutorials](../../../README.md) / [Projects](../../README.md) / [Password Generator](../README.md) / Spec
<!-- breadcrumbs:end -->

# Spec

Canonical project contract for the `password-generator` project.

## Goal

Build a small project app that introduces:

- turning password requirements into an explicit generation plan
- defining fixed character pools for letters, digits, and special characters
- keeping random character selection and shuffling out of the core logic
- separating generation rules and output formatting from randomization
- test-first planning and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
password_generation_plan(
  minimum_length: integer,
  special_character_count: integer,
  number_count: integer
) -> { total_length, letter_count, special_character_count, number_count }
letter_character_pool() -> string[]
digit_character_pool() -> string[]
special_character_pool() -> string[]
format_generated_password(password: string) -> string[]
```

Canonical behavior:

- treat `minimum_length`, `special_character_count`, and `number_count` as non-negative whole numbers
- `password_generation_plan` returns:
  - `total_length = max(minimum_length, special_character_count + number_count)`
  - `letter_count = total_length - special_character_count - number_count`
  - the same `special_character_count` supplied to the function
  - the same `number_count` supplied to the function
- `letter_character_pool` returns these exact lowercase ASCII letters in this exact order:
  - `a` through `z`
- `digit_character_pool` returns these exact ASCII digits in this exact order:
  - `0` through `9`
- `special_character_pool` returns these exact ASCII special characters in this exact order:
  - `!`
  - `@`
  - `#`
  - `$`
  - `%`
  - `^`
  - `&`
  - `*`
  - `?`
- `format_generated_password` returns these lines in this exact order:
  - `Your password is`
  - `<password>`

Examples:

- `password_generation_plan(8, 2, 2)` returns `{ total_length: 8, letter_count: 4, special_character_count: 2, number_count: 2 }`
- `password_generation_plan(3, 2, 2)` returns `{ total_length: 4, letter_count: 0, special_character_count: 2, number_count: 2 }`
- `letter_character_pool()` returns `a` through `z`
- `digit_character_pool()` returns `0` through `9`
- `special_character_pool()` returns `!, @, #, $, %, ^, &, *, ?`
- `format_generated_password("aurn2$1s#")` returns:
  - `Your password is`
  - `aurn2$1s#`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- random character selection in the core
- shuffling in the core
- uppercase letters in the canonical letter pool
- password-strength scoring
- secure secret storage or masked terminal input

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same generation-plan, character-pool, and output-formatting behavior instead of redefining it.

Adapters should:

- prompt for minimum length
- prompt for special-character count
- prompt for number count
- parse those values into whole numbers before calling the core
- call `password_generation_plan`
- call the three pool functions
- randomly choose:
  - `letter_count` characters from `letter_character_pool`
  - `special_character_count` characters from `special_character_pool`
  - `number_count` characters from `digit_character_pool`
- combine those chosen characters
- randomly shuffle the combined character list
- join the shuffled characters into one password string
- pass the generated password string to `format_generated_password`
- render the returned lines in order

For prompt-driven adapters, the prompts should be equivalent to:

```text
What's the minimum length?
How many special characters?
How many numbers?
```

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `password_generation_plan`, `letter_character_pool`, `digit_character_pool`, `special_character_pool`, and `format_generated_password`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- a test that `password_generation_plan(8, 2, 2)` returns the canonical `8 / 4 / 2 / 2` plan
- a test that `password_generation_plan(3, 2, 2)` grows the total length to `4`
- tests that the three pool functions return the exact canonical character sets
- a test that `format_generated_password` preserves the exact two-line canonical output
- tests for every adapter built in the chosen run that prove it uses the core plan, chooses the correct number of each character type, shuffles the final characters, and delegates formatting to the core logic correctly
