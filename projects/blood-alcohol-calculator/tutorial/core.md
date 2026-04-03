<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Blood Alcohol Calculator](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `blood-alcohol-calculator` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical not-legal example:

```text
given weight_pounds 175
and gender male
and number_of_drinks 4
and pure_alcohol_ounces_per_drink_hundredths 60
and hours_since_last_drink 1
when calculate_bac_hundredths is called
then the result is 8
```

At this point, `calculate_bac_hundredths` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
calculate_bac_hundredths_returns_the_canonical_not_legal_bac
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement legal-determination or formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Female Ratio Case

Write the next failing test:

```text
given weight_pounds 175
and gender female
and number_of_drinks 4
and pure_alcohol_ounces_per_drink_hundredths 60
and hours_since_last_drink 1
when calculate_bac_hundredths is called
then the result is 9
```

Suggested generic test name:

```text
calculate_bac_hundredths_uses_the_female_distribution_ratio
```

### 5. Green: Make The Female Ratio Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Negative-Value Clamp Case

Write the next failing test:

```text
given weight_pounds 200
and gender male
and number_of_drinks 1
and pure_alcohol_ounces_per_drink_hundredths 60
and hours_since_last_drink 10
when calculate_bac_hundredths is called
then the result is 0
```

Suggested generic test name:

```text
calculate_bac_hundredths_clamps_negative_values_to_zero
```

### 8. Green: Make The Clamp Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all BAC calculation tests green.

### 10. Red: Add The Exact Threshold Case

Write the next failing test:

```text
given bac_hundredths 8
when is_legal_to_drive_with_bac is called
then the result is false
```

Suggested generic test name:

```text
is_legal_to_drive_with_bac_returns_false_at_the_legal_threshold
```

### 11. Green: Make The Threshold Test Pass

Make it pass.

### 12. Refactor

Clean up the implementation while keeping all tests green.

### 13. Red: Add A Below-Threshold Case

Write the next failing test:

```text
given bac_hundredths 6
when is_legal_to_drive_with_bac is called
then the result is true
```

Suggested generic test name:

```text
is_legal_to_drive_with_bac_returns_true_below_the_legal_threshold
```

### 14. Green: Make The Below-Threshold Test Pass

Make it pass while keeping the earlier tests green.

### 15. Refactor

Clean up the implementation while keeping all calculation and comparison tests green.

### 16. Red: Add The Not-Legal Formatter

Write the next failing test:

```text
given bac_hundredths 8
and is_legal_to_drive false
when format_bac_report is called
then the returned lines are:
Your BAC is 0.08
It is not legal for you to drive.
```

Suggested generic test name:

```text
format_bac_report_returns_the_canonical_not_legal_report
```

### 17. Green: Make The Not-Legal Formatter Test Pass

Make it pass.

### 18. Refactor

Clean up the implementation while keeping all tests green.

### 19. Red: Add The Legal Formatter

Write the next failing test:

```text
given bac_hundredths 6
and is_legal_to_drive true
when format_bac_report is called
then the returned lines are:
Your BAC is 0.06
It is legal for you to drive.
```

Suggested generic test name:

```text
format_bac_report_returns_the_canonical_legal_report
```

### 20. Green: Make The Legal Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 21. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- three functions in one module
- one service object with one calculation method, one comparison method, and one formatter method
- one module plus a small BAC result or report record or struct

The important thing is that BAC calculation, legal-threshold comparison, and report formatting stay small, explicit, and directly testable.
