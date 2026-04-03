---
partial_kind: core-instructions
project: picking-a-winner
---

# Core Instructions

Project-specific core instruction fragment for the `picking-a-winner` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical contestant count:

```text
given contestant_names:
Homer
Bart
Maggie
Lisa
Moe
when contestant_count is called
then the result is 5
```

At this point, `contestant_count` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
contestant_count_returns_five_for_the_canonical_contestant_list
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement winner lookup or formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Canonical Winner Lookup

Write the next failing test:

```text
given contestant_names:
Homer
Bart
Maggie
Lisa
Moe
and choice_number 3
when winner_name_by_choice_number is called
then the result is Maggie
```

Suggested generic test name:

```text
winner_name_by_choice_number_returns_maggie_for_the_canonical_choice
```

### 5. Green: Make The Winner Lookup Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Out-Of-Range Cases

Write failing tests for out-of-range positions:

```text
given the canonical contestant list
and choice_number 0
when winner_name_by_choice_number is called
then the result is null
```

```text
given the canonical contestant list
and choice_number 6
when winner_name_by_choice_number is called
then the result is null
```

Suggested generic test names:

```text
winner_name_by_choice_number_returns_null_below_the_first_position
winner_name_by_choice_number_returns_null_above_the_last_position
```

### 8. Green: Make The Out-Of-Range Tests Pass

Make them pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all count and lookup tests green.

### 10. Red: Add The Winner Formatter

Write the next failing test:

```text
given winner_name Maggie
when format_winner_message is called
then the result is:
The winner is... Maggie.
```

Suggested generic test name:

```text
format_winner_message_returns_the_canonical_sentence
```

### 11. Green: Make The Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- three functions in one module
- one service object with count, winner-lookup, and formatter methods
- one module plus a small list helper for 1-based indexing

The important thing is that contestant counting, winner lookup, and winner formatting stay small, explicit, and directly testable.

