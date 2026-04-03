<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Karvonen Heart Rate](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `karvonen-heart-rate` core repo.

### 1. Red: Write The First Failing Test

Start with the first canonical intensity:

```text
given age 22
and resting_pulse 65
and intensity_percent 55
when calculate_target_heart_rate is called
then the result is 138
```

At this point, `calculate_target_heart_rate` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
calculate_target_heart_rate_returns_the_canonical_fifty_five_percent_rate
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement the full table or formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add More Canonical Intensities

Write failing tests for two more canonical rows:

```text
given age 22
and resting_pulse 65
and intensity_percent 60
when calculate_target_heart_rate is called
then the result is 145
```

```text
given age 22
and resting_pulse 65
and intensity_percent 95
when calculate_target_heart_rate is called
then the result is 191
```

Suggested generic test names:

```text
calculate_target_heart_rate_returns_the_canonical_sixty_percent_rate
calculate_target_heart_rate_returns_the_canonical_ninety_five_percent_rate
```

### 5. Green: Make The Canonical Intensity Tests Pass

Make the smallest safe change that gets all existing tests green.

### 6. Refactor

Clean up the implementation while keeping all current tests green.

### 7. Red: Add A Half-Up Rounding Case

Write the next failing test:

```text
given age 22
and resting_pulse 65
and intensity_percent 65
when calculate_target_heart_rate is called
then the result is 151
```

Suggested generic test name:

```text
calculate_target_heart_rate_rounds_half_up_to_the_nearest_whole_bpm
```

### 8. Green: Make The Rounding Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all per-row calculation tests green.

### 10. Red: Add The Full Table Generator

Write failing tests for the generated table:

```text
given age 22
and resting_pulse 65
when generate_karvonen_table is called
then the row count is 9
```

```text
given age 22
and resting_pulse 65
when generate_karvonen_table is called
then the first row is:
intensity_percent 55
rate_bpm 138
```

```text
given age 22
and resting_pulse 65
when generate_karvonen_table is called
then the last row is:
intensity_percent 95
rate_bpm 191
```

Suggested generic test names:

```text
generate_karvonen_table_returns_nine_rows
generate_karvonen_table_starts_at_fifty_five_percent
generate_karvonen_table_ends_at_ninety_five_percent
```

### 11. Green: Make The Table Tests Pass

Make them pass while keeping the earlier tests green.

### 12. Refactor

Clean up the implementation while keeping all generation tests green.

### 13. Red: Add The Formatter

Write the next failing test:

```text
given resting_pulse 65
and age 22
and rows [
  { intensity_percent: 55, rate_bpm: 138 },
  { intensity_percent: 60, rate_bpm: 145 }
]
when format_karvonen_table is called
then the returned lines are:
Resting Pulse: 65 Age: 22
Intensity | Rate
-------------|--------
55% | 138 bpm
60% | 145 bpm
```

Suggested generic test name:

```text
format_karvonen_table_returns_the_canonical_header_and_row_format
```

### 14. Green: Make The Formatter Test Pass

Make it pass.

### 15. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- three functions in one module
- one service object with one calculation method, one table-generation method, and one formatter method
- one module plus a small row record or struct

The important thing is that per-row heart-rate calculation, stepped table generation, and table formatting stay small, explicit, and directly testable.
