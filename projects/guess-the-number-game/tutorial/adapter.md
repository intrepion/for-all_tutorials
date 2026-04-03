<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Guess the Number Game](../../README.md) / [Tutorial](../README.md) / adapter.md
<!-- breadcrumbs:end -->

# Adapter Walkthrough

Project-specific red, green, refactor sequence for the `guess-the-number-game` adapter repo.

### 1. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter handles random secret selection, guess counting, and replay flow correctly.

### 2. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- render the equivalent of:
  - `Let's play Guess the Number.`
  - `Pick a difficulty level (1, 2, or 3):`
- pass the chosen difficulty to `resolve_difficulty_upper_bound`
- choose a random secret number between `1` and the returned upper bound, inclusive
- render the equivalent of:
  - `I have my number. What's your guess?`
- increment the guess count for every submitted guess
- pass each guess and the secret number to `evaluate_guess`
- when the result is `too_low` or `too_high`, render `format_guess_hint` and prompt again
- when the result is `correct`, render `format_victory_message`
- ask `Play again?`
- when the player chooses not to continue, render `format_goodbye_message`
- keep randomness, looping, replay control, transport, and input/output code out of the core game logic

### 3. Refactor

Clean up any remaining duplication while keeping the full suite green.
