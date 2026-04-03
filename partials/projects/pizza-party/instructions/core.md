---
partial_kind: core-instructions
project: pizza-party
---

# Core Instructions

Project-specific core instruction fragment for the `pizza-party` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical example calculation:

```text
given 8 people
and 2 pizzas
and 8 slices per pizza
when calculate_pizza_distribution is called
then total_slices is 16
and slices_per_person is 2
and leftover_slices is 0
```

At this point, `calculate_pizza_distribution` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
calculate_pizza_distribution_returns_even_distribution_for_canonical_example
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add A Leftover-Slices Case

Write the next failing test:

```text
given 8 people
and 2 pizzas
and 9 slices per pizza
when calculate_pizza_distribution is called
then total_slices is 18
and slices_per_person is 2
and leftover_slices is 2
```

Suggested generic test name:

```text
calculate_pizza_distribution_uses_integer_division_and_remainder
```

### 5. Green: Make The Leftover Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Report Formatter

Write the next failing test:

```text
given 8 people
and 2 pizzas
and distribution total_slices 16, slices_per_person 2, leftover_slices 0
when format_pizza_distribution_report is called
then the returned lines are:
8 people with 2 pizzas
Each person gets 2 pieces of pizza.
There are 0 leftover pieces.
```

Suggested generic test name:

```text
format_pizza_distribution_report_returns_the_canonical_three_line_report
```

### 8. Green: Make The Formatter Test Pass

Make it pass.

### 9. Refactor

Clean up the implementation while keeping all tests green.

### 10. Red: Prove The Report Preserves Leftovers

Write the next failing test:

```text
given 8 people
and 2 pizzas
and distribution total_slices 18, slices_per_person 2, leftover_slices 2
when format_pizza_distribution_report is called
then the third line is:
There are 2 leftover pieces.
```

Suggested generic test name:

```text
format_pizza_distribution_report_preserves_leftover_count
```

### 11. Green: Make The Leftover Report Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one calculation method and one formatter method
- one module plus a small result record or struct

The important thing is that pizza distribution and report formatting stay small, explicit, and directly testable.

