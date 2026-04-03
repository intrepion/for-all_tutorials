---
partial_kind: project-spec
project: guess-the-number-game
---

# Spec

Canonical project contract for the `guess-the-number-game` project.

## Goal

Build a small project app that introduces:

- mapping three difficulty levels to three numeric ranges
- comparing guesses to a hidden target number
- separating difficulty mapping, guess evaluation, and message formatting
- keeping randomness and replay control in the adapter
- test-first game-rule and formatting logic in small core functions
- thin surface adapters

## Core Logic Contract

The shared core contracts are:

```text
resolve_difficulty_upper_bound(level: integer) -> upper_bound | null
evaluate_guess(secret_number: integer, guess: integer) -> outcome
format_guess_hint(outcome: string) -> string
format_victory_message(guess_count: integer) -> string
format_goodbye_message() -> string
```

Canonical behavior:

- `resolve_difficulty_upper_bound` returns:
  - `10` for difficulty level `1`
  - `100` for difficulty level `2`
  - `1000` for difficulty level `3`
  - `null` for any other level
- `evaluate_guess` returns:
  - `too_low` when `guess` is less than `secret_number`
  - `too_high` when `guess` is greater than `secret_number`
  - `correct` when `guess` is equal to `secret_number`
- `format_guess_hint` returns:
  - `Too low. Guess again:` when `outcome` is `too_low`
  - `Too high. Guess again:` when `outcome` is `too_high`
- `format_victory_message` returns:
  - `You got it in <guess_count> guesses!`
- `format_goodbye_message` returns:
  - `Goodbye!`

Examples:

- `resolve_difficulty_upper_bound(1)` returns `10`
- `resolve_difficulty_upper_bound(2)` returns `100`
- `resolve_difficulty_upper_bound(3)` returns `1000`
- `resolve_difficulty_upper_bound(4)` returns `null`
- `evaluate_guess(2, 1)` returns `too_low`
- `evaluate_guess(2, 5)` returns `too_high`
- `evaluate_guess(2, 2)` returns `correct`
- `format_guess_hint("too_low")` returns `Too low. Guess again:`
- `format_guess_hint("too_high")` returns `Too high. Guess again:`
- `format_victory_message(3)` returns `You got it in 3 guesses!`
- `format_goodbye_message()` returns `Goodbye!`

## Non-Goals

This project does not include:

- persistence
- localization
- authentication
- network access
- configuration files
- difficulty levels beyond `1`, `2`, and `3`
- input validation for non-numeric guesses in the core
- random number generation in the core
- scoreboards or best-score tracking
- cheating detection

## Surface Expectations

This spec follows the shared surface and setup-path rules in [Projects](../../README.md#shared-spec-conventions).

For this project, every tutorial run should adapt the same difficulty mapping, guess evaluation, and formatting behavior instead of redefining it.

Adapters should:

- prompt for the difficulty level
- resolve the difficulty range with `resolve_difficulty_upper_bound`
- choose a random secret number between `1` and the returned upper bound, inclusive
- prompt for guesses
- increment the guess count on every submitted guess
- call `evaluate_guess` for each guess
- render `format_guess_hint` after low and high guesses
- render `format_victory_message` after a correct guess
- ask whether to play again
- render `format_goodbye_message` when play ends

## Output Repository Expectations

This project follows the shared output model in [Projects](../../README.md#shared-output-model).

For this project, the core repo owns `resolve_difficulty_upper_bound`, `evaluate_guess`, `format_guess_hint`, `format_victory_message`, and `format_goodbye_message`, and adapter repos must not reimplement them.

## Testing And Coverage Contract

This spec follows the shared coverage policy in [Projects](../../README.md#shared-coverage-policy).

This project should be built in a spec-driven and test-driven way.

Minimum test expectations:

- tests for all three difficulty mappings plus an invalid level
- tests for `evaluate_guess` on low, high, and correct guesses
- tests that `format_guess_hint` preserves the exact low and high hint sentences
- tests that `format_victory_message` preserves the exact canonical sentence
- tests that `format_goodbye_message` preserves the exact canonical sentence
- tests for every adapter built in the chosen run that prove it chooses a secret number in the resolved range, tracks guess counts correctly, and handles replay flow correctly

