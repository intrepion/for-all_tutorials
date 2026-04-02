<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Retirement Calculator](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `retirement-calculator` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical example calculation:

```text
given current age 25
and desired retirement age 65
and current year 2015
when calculate_retirement is called
then years_left is 40
and retirement_year is 2055
```

At this point, `calculate_retirement` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
calculate_retirement_returns_years_left_and_retirement_year_for_canonical_example
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Prove The Singular One-Year Case

Write the next failing test:

```text
given current age 64
and desired retirement age 65
and current year 2015
when calculate_retirement is called
then years_left is 1
and retirement_year is 2016
```

Suggested generic test name:

```text
calculate_retirement_handles_the_one_year_case
```

### 5. Green: Make The One-Year Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Report Formatter

Write the next failing test:

```text
given current year 2015
and years_left 40
and retirement_year 2055
when format_retirement_report is called
then the returned lines are:
You have 40 years left until you can retire.
It's 2015, so you can retire in 2055.
```

Suggested generic test name:

```text
format_retirement_report_returns_the_canonical_two_line_report
```

### 8. Green: Make The Formatter Test Pass

Make it pass.

### 9. Refactor

Clean up the implementation while keeping all tests green.

### 10. Red: Add The Singular Formatter Case

Write the next failing test:

```text
given current year 2015
and years_left 1
and retirement_year 2016
when format_retirement_report is called
then the returned lines are:
You have 1 year left until you can retire.
It's 2015, so you can retire in 2016.
```

Suggested generic test name:

```text
format_retirement_report_uses_singular_year_when_one_year_is_left
```

### 11. Green: Make The Singular Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one calculation method and one formatter method
- one module plus a small result record or struct

The important thing is that retirement calculation and report formatting stay small, explicit, and directly testable.
