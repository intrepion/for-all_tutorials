<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Tax Calculator](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `tax-calculator` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical Wisconsin example:

```text
given order_amount_cents 1000
and state WI
when calculate_tax_for_order is called
then subtotal_cents is 1000
and tax_cents is 55
and total_cents is 1055
and tax_applied is true
```

At this point, `calculate_tax_for_order` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
calculate_tax_for_order_applies_wisconsin_tax_for_the_canonical_example
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add A Non-Wisconsin Branch Case

Write the next failing test:

```text
given order_amount_cents 1000
and state MN
when calculate_tax_for_order is called
then subtotal_cents is 1000
and tax_cents is 0
and total_cents is 1000
and tax_applied is false
```

Suggested generic test name:

```text
calculate_tax_for_order_skips_tax_for_non_wisconsin_states
```

### 5. Green: Make The Non-Wisconsin Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add A Tax-Rounding Case

Write the next failing test:

```text
given order_amount_cents 100
and state WI
when calculate_tax_for_order is called
then tax_cents is 6
and total_cents is 106
```

Suggested generic test name:

```text
calculate_tax_for_order_rounds_wisconsin_tax_half_up_to_the_nearest_cent
```

### 8. Green: Make The Rounding Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all calculation tests green.

### 10. Red: Add The Taxed Report Formatter

Write the next failing test:

```text
given a taxed summary with subtotal_cents 1000, tax_cents 55, total_cents 1055, tax_applied true
when format_tax_report is called
then the returned lines are:
The subtotal is $10.00.
The tax is $0.55.
The total is $10.55.
```

Suggested generic test name:

```text
format_tax_report_returns_the_canonical_taxed_three_line_report
```

### 11. Green: Make The Taxed Formatter Test Pass

Make it pass.

### 12. Refactor

Clean up the implementation while keeping all tests green.

### 13. Red: Add The Untaxed Report Formatter

Write the next failing test:

```text
given an untaxed summary with subtotal_cents 1000, tax_cents 0, total_cents 1000, tax_applied false
when format_tax_report is called
then the returned lines are:
The total is $10.00.
```

Suggested generic test name:

```text
format_tax_report_returns_the_canonical_untaxed_one_line_report
```

### 14. Green: Make The Untaxed Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 15. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one calculation method and one formatter method
- one module plus a small order-summary record or struct

The important thing is that tax calculation, branching, cent rounding, and report formatting stay small, explicit, and directly testable.
