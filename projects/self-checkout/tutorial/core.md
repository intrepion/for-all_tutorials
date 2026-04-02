<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Self-Checkout](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `self-checkout` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical calculation example:

```text
given items:
item 1 price 2500 cents, quantity 2
item 2 price 1000 cents, quantity 1
item 3 price 400 cents, quantity 1
when calculate_checkout_summary is called
then line totals are 5000, 1000, and 400
and subtotal_cents is 6400
and tax_cents is 352
and total_cents is 6752
```

At this point, `calculate_checkout_summary` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
calculate_checkout_summary_returns_canonical_line_totals_and_totals
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add A Tax-Rounding Boundary Case

Write the next failing test:

```text
given items:
item 1 price 25 cents, quantity 1
item 2 price 25 cents, quantity 1
item 3 price 50 cents, quantity 1
when calculate_checkout_summary is called
then subtotal_cents is 100
and tax_cents is 6
and total_cents is 106
```

Suggested generic test name:

```text
calculate_checkout_summary_rounds_tax_half_up_to_the_nearest_cent
```

### 5. Green: Make The Tax-Rounding Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Checkout Report Formatter

Write the next failing test:

```text
given the canonical checkout summary
when format_checkout_report is called
then the returned lines are:
Item 1: 2 x $25.00 = $50.00
Item 2: 1 x $10.00 = $10.00
Item 3: 1 x $4.00 = $4.00
Subtotal: $64.00
Tax: $3.52
Total: $67.52
```

Suggested generic test name:

```text
format_checkout_report_returns_the_canonical_six_line_report
```

### 8. Green: Make The Formatter Test Pass

Make it pass.

### 9. Refactor

Clean up the implementation while keeping all tests green.

### 10. Red: Prove Money Formatting Stays Fixed-Precision

Write the next failing test:

```text
given a summary with a line total of 400 cents
when format_checkout_report is called
then that line contains:
Item 3: 1 x $4.00 = $4.00
```

Suggested generic test name:

```text
format_checkout_report_renders_money_with_exactly_two_decimal_places
```

### 11. Green: Make The Fixed-Precision Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one calculation method and one formatter method
- one module plus small money or summary records or structs

The important thing is that checkout calculation, tax rounding, and report formatting stay small, explicit, and directly testable.
