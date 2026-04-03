---
partial_kind: core-instructions
project: guess-the-number-game
---

# Core Instructions

Project-specific core instruction fragment for the `guess-the-number-game` core repo.

### 1. Red: Write The First Failing Test

Start with the first difficulty level:

```text
given difficulty level 1
when resolve_difficulty_upper_bound is called
then the result is 10
```

At this point, `resolve_difficulty_upper_bound` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
resolve_difficulty_upper_bound_returns_ten_for_level_one
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement guess evaluation or formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Remaining Difficulty Levels

Write failing tests for the remaining levels and an invalid one:

```text
given difficulty level 2
when resolve_difficulty_upper_bound is called
then the result is 100
```

```text
given difficulty level 3
when resolve_difficulty_upper_bound is called
then the result is 1000
```

```text
given difficulty level 4
when resolve_difficulty_upper_bound is called
then the result is null
```

Suggested generic test names:

```text
resolve_difficulty_upper_bound_returns_one_hundred_for_level_two
resolve_difficulty_upper_bound_returns_one_thousand_for_level_three
resolve_difficulty_upper_bound_returns_null_for_invalid_levels
```

### 5. Green: Make The Difficulty Tests Pass

Make the smallest safe change that gets all existing tests green.

### 6. Refactor

Clean up the implementation while keeping all difficulty tests green.

### 7. Red: Add Guess Evaluation

Write failing tests for the three guess outcomes:

```text
given secret_number 2
and guess 1
when evaluate_guess is called
then the result is too_low
```

```text
given secret_number 2
and guess 5
when evaluate_guess is called
then the result is too_high
```

```text
given secret_number 2
and guess 2
when evaluate_guess is called
then the result is correct
```

Suggested generic test names:

```text
evaluate_guess_returns_too_low_for_small_guesses
evaluate_guess_returns_too_high_for_large_guesses
evaluate_guess_returns_correct_for_exact_matches
```

### 8. Green: Make The Guess Tests Pass

Make them pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all game-rule tests green.

### 10. Red: Add The Hint Formatter

Write failing tests for the hint messages:

```text
given outcome too_low
when format_guess_hint is called
then the result is:
Too low. Guess again:
```

```text
given outcome too_high
when format_guess_hint is called
then the result is:
Too high. Guess again:
```

Suggested generic test names:

```text
format_guess_hint_returns_the_low_hint
format_guess_hint_returns_the_high_hint
```

### 11. Green: Make The Hint Tests Pass

Make them pass while keeping the earlier tests green.

### 12. Refactor

Clean up the implementation while keeping all tests green.

### 13. Red: Add The Victory And Goodbye Messages

Write failing tests for the remaining messages:

```text
given guess_count 3
when format_victory_message is called
then the result is:
You got it in 3 guesses!
```

```text
when format_goodbye_message is called
then the result is:
Goodbye!
```

Suggested generic test names:

```text
format_victory_message_returns_the_canonical_sentence
format_goodbye_message_returns_the_canonical_sentence
```

### 14. Green: Make The Message Tests Pass

Make them pass while keeping the earlier tests green.

### 15. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- five functions in one module
- one service object with difficulty, evaluation, and formatter methods
- one module plus a small outcome enum or tagged value

The important thing is that difficulty mapping, guess evaluation, and message formatting stay separate, small, and directly testable.

