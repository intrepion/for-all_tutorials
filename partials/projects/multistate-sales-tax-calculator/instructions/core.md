---
partial_kind: core-instructions
project: multistate-sales-tax-calculator
---

# Core Instructions

Project-specific core instruction fragment for the `multistate-sales-tax-calculator` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical Wisconsin example:

```text
given order_amount_cents 1000
and state Wisconsin
and county null
when calculate_multistate_sales_tax is called
then tax_cents is 50
and total_cents is 1050
and tax_applied is true
and effective_rate_thousandths is 50
```

At this point, `calculate_multistate_sales_tax` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
calculate_multistate_sales_tax_applies_base_wisconsin_tax_for_the_canonical_example
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Eau Claire County Case

Write the next failing test:

```text
given order_amount_cents 1000
and state Wisconsin
and county Eau Claire
when calculate_multistate_sales_tax is called
then tax_cents is 55
and total_cents is 1055
and effective_rate_thousandths is 55
```

Suggested generic test name:

```text
calculate_multistate_sales_tax_adds_the_eau_claire_county_surcharge
```

### 5. Green: Make The Eau Claire Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Dunn County Case

Write the next failing test:

```text
given order_amount_cents 1000
and state Wisconsin
and county Dunn
when calculate_multistate_sales_tax is called
then tax_cents is 54
and total_cents is 1054
and effective_rate_thousandths is 54
```

Suggested generic test name:

```text
calculate_multistate_sales_tax_adds_the_dunn_county_surcharge
```

### 8. Green: Make The Dunn Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all Wisconsin calculation tests green.

### 10. Red: Add The Illinois Case

Write the next failing test:

```text
given order_amount_cents 1000
and state Illinois
and county null
when calculate_multistate_sales_tax is called
then tax_cents is 80
and total_cents is 1080
and effective_rate_thousandths is 80
```

Suggested generic test name:

```text
calculate_multistate_sales_tax_applies_the_illinois_rate
```

### 11. Green: Make The Illinois Test Pass

Make it pass.

### 12. Refactor

Clean up the implementation while keeping all tests green.

### 13. Red: Add The Untaxed Other-State Case

Write the next failing test:

```text
given order_amount_cents 1000
and state Minnesota
and county null
when calculate_multistate_sales_tax is called
then tax_cents is 0
and total_cents is 1000
and tax_applied is false
and effective_rate_thousandths is 0
```

Suggested generic test name:

```text
calculate_multistate_sales_tax_skips_tax_for_other_states
```

### 14. Green: Make The Other-State Test Pass

Make it pass while keeping the earlier tests green.

### 15. Refactor

Clean up the implementation while keeping all calculation tests green.

### 16. Red: Add The Taxed Report Formatter

Write the next failing test:

```text
given a taxed summary with tax_cents 50, total_cents 1050, tax_applied true, effective_rate_thousandths 50
when format_multistate_sales_tax_report is called
then the returned lines are:
The tax is $0.50.
The total is $10.50.
```

Suggested generic test name:

```text
format_multistate_sales_tax_report_returns_the_taxed_two_line_report
```

### 17. Green: Make The Taxed Formatter Test Pass

Make it pass.

### 18. Refactor

Clean up the implementation while keeping all tests green.

### 19. Red: Add The Untaxed Report Formatter

Write the next failing test:

```text
given an untaxed summary with tax_cents 0, total_cents 1000, tax_applied false, effective_rate_thousandths 0
when format_multistate_sales_tax_report is called
then the returned lines are:
The total is $10.00.
```

Suggested generic test name:

```text
format_multistate_sales_tax_report_returns_the_untaxed_one_line_report
```

### 20. Green: Make The Untaxed Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 21. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one calculation method and one formatter method
- one module plus a small order-summary record or struct

The important thing is that nested state-and-county tax calculation, cent rounding, and report formatting stay small, explicit, and directly testable.

