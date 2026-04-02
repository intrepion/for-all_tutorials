<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Computing Simple Interest](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `computing-simple-interest` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical simple-interest example:

```text
given principal_dollars 1500
and annual_rate_tenths_percent 43
and years 4
when calculate_simple_interest is called
then accrued_amount_cents is 175800
```

At this point, `calculate_simple_interest` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
calculate_simple_interest_returns_the_canonical_accrued_amount
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add A Half-Up Rounding Case

Write the next failing test:

```text
given principal_dollars 1
and annual_rate_tenths_percent 5
and years 1
when calculate_simple_interest is called
then accrued_amount_cents is 101
```

Suggested generic test name:

```text
calculate_simple_interest_rounds_half_up_to_the_nearest_cent
```

### 5. Green: Make The Rounding Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Report Formatter

Write the next failing test:

```text
given years 4
and annual_rate_tenths_percent 43
and accrued_amount_cents 175800
when format_simple_interest_report is called
then the returned message is:
After 4 years at 4.3%, the investment will be worth $1758.00.
```

Suggested generic test name:

```text
format_simple_interest_report_returns_the_canonical_sentence
```

### 8. Green: Make The Formatter Test Pass

Make it pass.

### 9. Refactor

Clean up the implementation while keeping all tests green.

### 10. Red: Prove The Rate Keeps One Decimal Place

Write the next failing test:

```text
given years 1
and annual_rate_tenths_percent 5
and accrued_amount_cents 101
when format_simple_interest_report is called
then the returned message is:
After 1 years at 0.5%, the investment will be worth $1.01.
```

Suggested generic test name:

```text
format_simple_interest_report_renders_the_rate_with_exactly_one_decimal_place
```

### 11. Green: Make The Rate-Formatting Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one calculation method and one formatter method
- one module plus a small money or investment record or struct

The important thing is that simple-interest calculation, cent rounding, and report formatting stay small, explicit, and directly testable.
