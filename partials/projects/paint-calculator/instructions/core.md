---
partial_kind: core-instructions
project: paint-calculator
---

# Core Instructions

Project-specific core instruction fragment for the `paint-calculator` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical example calculation:

```text
given length 15 feet
and width 24 feet
when calculate_paint_requirements is called
then square_feet is 360
and gallons_needed is 2
```

At this point, `calculate_paint_requirements` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
calculate_paint_requirements_returns_the_canonical_area_and_gallon_count
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add An Exact-Coverage Boundary Case

Write the next failing test:

```text
given length 14 feet
and width 25 feet
when calculate_paint_requirements is called
then square_feet is 350
and gallons_needed is 1
```

Suggested generic test name:

```text
calculate_paint_requirements_returns_one_gallon_for_exact_coverage
```

### 5. Green: Make The Exact-Coverage Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Prove The Calculation Rounds Up

Write the next failing test:

```text
given length 13 feet
and width 27 feet
when calculate_paint_requirements is called
then square_feet is 351
and gallons_needed is 2
```

Suggested generic test name:

```text
calculate_paint_requirements_rounds_up_when_area_exceeds_exact_coverage
```

### 8. Green: Make The Round-Up Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all calculation tests green.

### 10. Red: Add The Purchase-Message Formatter

Write the next failing test:

```text
given square_feet 360
and gallons_needed 2
when format_paint_purchase_message is called
then the returned message is:
You will need to purchase 2 gallons of paint to cover 360 square feet.
```

Suggested generic test name:

```text
format_paint_purchase_message_returns_the_canonical_plural_message
```

### 11. Green: Make The Formatter Test Pass

Make it pass.

### 12. Refactor

Clean up the implementation while keeping all tests green.

### 13. Red: Add The Singular Gallon Case

Write the next failing test:

```text
given square_feet 350
and gallons_needed 1
when format_paint_purchase_message is called
then the returned message is:
You will need to purchase 1 gallon of paint to cover 350 square feet.
```

Suggested generic test name:

```text
format_paint_purchase_message_uses_singular_gallon_for_one
```

### 14. Green: Make The Singular Test Pass

Make it pass while keeping the earlier tests green.

### 15. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one calculation method and one formatter method
- one module plus a small result record or struct

The important thing is that area calculation, gallon rounding, and purchase-message formatting stay small, explicit, and directly testable.

