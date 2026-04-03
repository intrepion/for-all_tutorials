<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Filtering Values](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `filtering-values` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical filtering example:

```text
given numbers:
1 2 3 4 5 6 7 8
when filter_even_numbers is called
then the result is:
2 4 6 8
```

At this point, `filter_even_numbers` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
filter_even_numbers_returns_the_canonical_even_values
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add An Order-Preservation Case

Write the next failing test:

```text
given numbers:
-4 -3 0 7 12
when filter_even_numbers is called
then the result is:
-4 0 12
```

Suggested generic test name:

```text
filter_even_numbers_preserves_order_for_mixed_signed_values
```

### 5. Green: Make The Order-Preservation Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Empty Result Case

Write the next failing test:

```text
given numbers:
1 3 5
when filter_even_numbers is called
then the result is an empty list
```

Suggested generic test name:

```text
filter_even_numbers_returns_an_empty_list_when_no_values_are_even
```

### 8. Green: Make The Empty Result Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all filtering tests green.

### 10. Red: Add The Non-Empty Formatter

Write the next failing test:

```text
given even_numbers:
2 4 6 8
when format_even_numbers_report is called
then the result is:
The even numbers are 2 4 6 8.
```

Suggested generic test name:

```text
format_even_numbers_report_returns_the_canonical_sentence
```

### 11. Green: Make The Non-Empty Formatter Test Pass

Make it pass.

### 12. Refactor

Clean up the implementation while keeping all tests green.

### 13. Red: Add The Empty Formatter

Write the next failing test:

```text
given even_numbers is an empty list
when format_even_numbers_report is called
then the result is:
The even numbers are.
```

Suggested generic test name:

```text
format_even_numbers_report_returns_the_defined_empty_result_sentence
```

### 14. Green: Make The Empty Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 15. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one filtering method and one formatter method
- one module plus a small helper for joining numeric values

The important thing is that even-number filtering and report formatting stay small, explicit, and directly testable.
