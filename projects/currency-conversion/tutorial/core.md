<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Currency Conversion](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `currency-conversion` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical conversion example:

```text
given amount_euros 81
and exchange_rate_hundredths 13751
when calculate_currency_conversion is called
then usd_cents is 11138
```

At this point, `calculate_currency_conversion` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
calculate_currency_conversion_returns_the_canonical_usd_amount
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add A Half-Up Rounding Case

Write the next failing test:

```text
given amount_euros 1
and exchange_rate_hundredths 10050
when calculate_currency_conversion is called
then usd_cents is 101
```

Suggested generic test name:

```text
calculate_currency_conversion_rounds_half_up_to_the_nearest_cent
```

### 5. Green: Make The Rounding Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Report Formatter

Write the next failing test:

```text
given amount_euros 81
and exchange_rate_hundredths 13751
and usd_cents 11138
when format_currency_conversion_report is called
then the returned lines are:
81 euros at an exchange rate of 137.51 is
111.38 U.S. dollars.
```

Suggested generic test name:

```text
format_currency_conversion_report_returns_the_canonical_two_line_report
```

### 8. Green: Make The Formatter Test Pass

Make it pass.

### 9. Refactor

Clean up the implementation while keeping all tests green.

### 10. Red: Prove The Exchange Rate Keeps Two Decimals

Write the next failing test:

```text
given amount_euros 1
and exchange_rate_hundredths 10050
and usd_cents 101
when format_currency_conversion_report is called
then the first line is:
1 euros at an exchange rate of 100.50 is
```

Suggested generic test name:

```text
format_currency_conversion_report_renders_the_exchange_rate_with_exactly_two_decimal_places
```

### 11. Green: Make The Rate-Formatting Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one calculation method and one formatter method
- one module plus a small money or conversion record or struct

The important thing is that currency conversion, cent rounding, and report formatting stay small, explicit, and directly testable.
