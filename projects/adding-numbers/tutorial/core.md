<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Adding Numbers](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `adding-numbers` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical example:

```text
given numbers [1, 2, 3, 4, 5]
when sum_numbers is called
then the result is 15
```

At this point, `sum_numbers` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
sum_numbers_returns_the_canonical_total
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add A Mixed-Signed Example

Write the next failing test:

```text
given numbers [10, -3, 0, 8, 5]
when sum_numbers is called
then the result is 20
```

Suggested generic test name:

```text
sum_numbers_handles_positive_negative_and_zero_values
```

### 5. Green: Make The Mixed-Signed Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Report Formatter

Write the next failing test:

```text
given total 15
when format_total_report is called
then the returned message is:
The total is 15.
```

Suggested generic test name:

```text
format_total_report_returns_the_canonical_sentence
```

### 8. Green: Make The Formatter Test Pass

Make it pass.

### 9. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one calculation method and one formatter method
- one module plus a small numeric-list helper

The important thing is that numeric accumulation and report formatting stay small, explicit, and directly testable.
