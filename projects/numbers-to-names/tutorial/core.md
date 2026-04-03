<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Numbers to Names](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `numbers-to-names` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical month example:

```text
given month_number 3
when lookup_month_name is called
then the result is March
```

At this point, `lookup_month_name` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
lookup_month_name_returns_march_for_three
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Boundary Cases

Write failing tests for the month boundaries:

```text
given month_number 1
when lookup_month_name is called
then the result is January
```

and

```text
given month_number 12
when lookup_month_name is called
then the result is December
```

Suggested generic test names:

```text
lookup_month_name_returns_january_for_one
lookup_month_name_returns_december_for_twelve
```

### 5. Green: Make The Boundary Tests Pass

Make the smallest safe change that gets all existing tests green.

### 6. Refactor

Clean up the implementation while keeping the tests green.

### 7. Red: Add The Error Case

Write the next failing test:

```text
given month_number 13
when lookup_month_name is called
then the result is null
```

Suggested generic test name:

```text
lookup_month_name_returns_null_for_out_of_range_values
```

### 8. Green: Make The Error Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all lookup tests green.

### 10. Red: Add The Success Formatter

Write the next failing test:

```text
given month_name March
when format_month_lookup_message is called
then the returned message is:
The name of the month is March.
```

Suggested generic test name:

```text
format_month_lookup_message_returns_the_canonical_success_sentence
```

### 11. Green: Make The Success Formatter Test Pass

Make it pass.

### 12. Refactor

Clean up the implementation while keeping all tests green.

### 13. Red: Add The Error Formatter

Write the next failing test:

```text
given month_name null
when format_month_lookup_message is called
then the returned message is:
That is not a valid month number.
```

Suggested generic test name:

```text
format_month_lookup_message_returns_the_canonical_error_sentence
```

### 14. Green: Make The Error Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 15. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one lookup method and one formatter method
- one module plus a small lookup-result record or struct

The important thing is that month lookup and message formatting stay small, explicit, and directly testable.
