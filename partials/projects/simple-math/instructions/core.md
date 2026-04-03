---
partial_kind: core-instructions
project: simple-math
---

# Core Instructions

Project-specific core instruction fragment for the `simple-math` core repo.

### 1. Red: Write The First Failing Test

Start with the sum from the canonical example:

```text
given first number 10
and second number 5
when calculate_simple_math is called
then sum is 15
```

At this point, `calculate_simple_math` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
calculate_simple_math_returns_sum_for_canonical_example
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles the first case. Do not jump ahead and implement every operation before the next test exists.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Difference

Write the next failing test:

```text
given first number 10
and second number 5
when calculate_simple_math is called
then difference is 5
```

Suggested generic test name:

```text
calculate_simple_math_returns_difference_for_canonical_example
```

### 5. Green: Make The Difference Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Product

Write the next failing test:

```text
given first number 10
and second number 5
when calculate_simple_math is called
then product is 50
```

Suggested generic test name:

```text
calculate_simple_math_returns_product_for_canonical_example
```

### 8. Green: Make The Product Test Pass

Make it pass.

### 9. Refactor

Clean up the implementation while keeping all tests green.

### 10. Red: Add The Quotient

Write the next failing test:

```text
given first number 10
and second number 5
when calculate_simple_math is called
then quotient is 2
```

Suggested generic test name:

```text
calculate_simple_math_returns_exact_integer_quotient_for_canonical_example
```

### 11. Green: Make The Quotient Test Pass

Make it pass for the exact-division case.

### 12. Refactor

Clean up the implementation while keeping all tests green.

### 13. Red: Prove Negative Numbers Work

Write the next failing test:

```text
given first number -6
and second number 3
when calculate_simple_math is called
then the result is sum -3, difference -9, product -18, quotient -2
```

Suggested generic test name:

```text
calculate_simple_math_handles_negative_numbers
```

### 14. Green: Make The Negative-Number Test Pass

Make it pass while keeping the earlier tests green.

### 15. Refactor

Clean up the implementation while keeping all calculation tests green.

### 16. Red: Add The Report Formatter

Write the next failing test:

```text
given first number 10
and second number 5
and results sum 15, difference 5, product 50, quotient 2
when format_simple_math_report is called
then the returned lines are:
10 + 5 = 15
10 - 5 = 5
10 * 5 = 50
10 / 5 = 2
```

Suggested generic test name:

```text
format_simple_math_report_returns_four_lines_in_canonical_order
```

### 17. Green: Make The Formatter Test Pass

Make it pass.

### 18. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one calculation method and one formatter method
- one module plus a small result record or struct

The important thing is that arithmetic and report formatting stay small, explicit, and directly testable.

