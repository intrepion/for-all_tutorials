---
partial_kind: core-instructions
project: multiplication-table
---

# Core Instructions

Project-specific core instruction fragment for the `multiplication-table` core repo.

### 1. Red: Write The First Failing Test

Start with the very first row:

```text
when generate_multiplication_table is called
then the first row is:
left_factor 0
right_factor 0
product 0
```

At this point, `generate_multiplication_table` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
generate_multiplication_table_starts_with_zero_times_zero
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement the full table or formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add A Middle-Table Example

Write the next failing test:

```text
when generate_multiplication_table is called
then one row includes:
left_factor 7
right_factor 8
product 56
```

Suggested generic test name:

```text
generate_multiplication_table_includes_seven_times_eight
```

### 5. Green: Make The Middle-Row Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Last Row And Size

Write failing tests for the table boundary and total size:

```text
when generate_multiplication_table is called
then the last row is:
left_factor 12
right_factor 12
product 144
```

```text
when generate_multiplication_table is called
then the row count is 169
```

Suggested generic test names:

```text
generate_multiplication_table_ends_with_twelve_times_twelve
generate_multiplication_table_returns_thirteen_by_thirteen_rows
```

### 8. Green: Make The Boundary Tests Pass

Make them pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all generation tests green.

### 10. Red: Add The Formatter

Write the next failing test:

```text
given rows [
  { left_factor: 0, right_factor: 0, product: 0 },
  { left_factor: 12, right_factor: 12, product: 144 }
]
when format_multiplication_table is called
then the returned lines are:
0 x 0 = 0
12 x 12 = 144
```

Suggested generic test name:

```text
format_multiplication_table_returns_lines_in_canonical_format
```

### 11. Green: Make The Formatter Test Pass

Make it pass.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one generation method and one formatter method
- one module plus a small row record or struct

The important thing is that table generation and output formatting stay small, explicit, and directly testable.

