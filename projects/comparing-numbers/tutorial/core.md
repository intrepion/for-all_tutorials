<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Comparing Numbers](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `comparing-numbers` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical distinctness case:

```text
given first_number 1
and second_number 51
and third_number 2
when all_numbers_are_different is called
then the result is true
```

At this point, `all_numbers_are_different` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
all_numbers_are_different_returns_true_for_three_distinct_numbers
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement largest-number selection or formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add Duplicate Cases

Write failing tests for duplicate inputs:

```text
given first_number 1
and second_number 1
and third_number 2
when all_numbers_are_different is called
then the result is false
```

```text
given first_number 2
and second_number 3
and third_number 2
when all_numbers_are_different is called
then the result is false
```

Suggested generic test names:

```text
all_numbers_are_different_returns_false_when_the_first_two_numbers_match
all_numbers_are_different_returns_false_when_the_first_and_third_numbers_match
```

### 5. Green: Make The Duplicate Tests Pass

Make the smallest safe change that gets all distinctness tests green.

### 6. Refactor

Clean up the implementation while keeping all distinctness tests green.

### 7. Red: Add The Canonical Largest-Number Test

Write the next failing test:

```text
given first_number 1
and second_number 51
and third_number 2
when largest_of_three is called
then the result is 51
```

Suggested generic test name:

```text
largest_of_three_returns_the_canonical_largest_value
```

### 8. Green: Make The Canonical Largest Test Pass

Make it pass.

### 9. Refactor

Clean up the implementation while keeping the earlier tests green.

### 10. Red: Add The Remaining Largest Positions

Write failing tests for the other positions:

```text
given first_number 51
and second_number 1
and third_number 2
when largest_of_three is called
then the result is 51
```

```text
given first_number 1
and second_number 2
and third_number 51
when largest_of_three is called
then the result is 51
```

Suggested generic test names:

```text
largest_of_three_returns_the_first_number_when_it_is_largest
largest_of_three_returns_the_third_number_when_it_is_largest
```

### 11. Green: Make The Remaining Largest Tests Pass

Make them pass while keeping the earlier tests green.

### 12. Refactor

Clean up the implementation while keeping all comparison tests green.

### 13. Red: Add The Success Formatter

Write the next failing test:

```text
given largest_number 51
when format_largest_number_message is called
then the result is:
The largest number is 51.
```

Suggested generic test name:

```text
format_largest_number_message_returns_the_canonical_sentence
```

### 14. Green: Make The Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 15. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- three functions in one module
- one service object with distinctness, largest-number, and formatter methods
- one module plus a small helper for pairwise comparisons

The important thing is that distinctness checks, largest-number selection, and output formatting stay small, explicit, and directly testable.
