<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [BMI Calculator](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `bmi-calculator` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical ideal-range example:

```text
given weight_pounds 144
and height_inches 72
when calculate_bmi_tenths is called
then the result is 195
```

At this point, `calculate_bmi_tenths` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
calculate_bmi_tenths_returns_the_canonical_ideal_range_bmi
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement classification or formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Overweight Example

Write the next failing test:

```text
given weight_pounds 178
and height_inches 62
when calculate_bmi_tenths is called
then the result is 325
```

Suggested generic test name:

```text
calculate_bmi_tenths_returns_the_canonical_overweight_bmi
```

### 5. Green: Make The Overweight Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Underweight Example

Write the next failing test:

```text
given weight_pounds 100
and height_inches 68
when calculate_bmi_tenths is called
then the result is 152
```

Suggested generic test name:

```text
calculate_bmi_tenths_returns_an_underweight_bmi
```

### 8. Green: Make The Underweight Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all BMI calculation tests green.

### 10. Red: Add The Classification Boundaries

Write failing tests for the BMI range boundaries:

```text
given bmi_tenths 185
when classify_bmi is called
then the result is ideal
```

and

```text
given bmi_tenths 250
when classify_bmi is called
then the result is ideal
```

Suggested generic test names:

```text
classify_bmi_treats_the_lower_boundary_as_ideal
classify_bmi_treats_the_upper_boundary_as_ideal
```

### 11. Green: Make The Boundary Tests Pass

Make them pass.

### 12. Refactor

Clean up the implementation while keeping all tests green.

### 13. Red: Add The Ideal Report Formatter

Write the next failing test:

```text
given bmi_tenths 195
and classification ideal
when format_bmi_report is called
then the returned lines are:
Your BMI is 19.5.
You are within the ideal weight range.
```

Suggested generic test name:

```text
format_bmi_report_returns_the_canonical_ideal_report
```

### 14. Green: Make The Ideal Formatter Test Pass

Make it pass.

### 15. Refactor

Clean up the implementation while keeping all tests green.

### 16. Red: Add The Underweight And Overweight Reports

Write failing tests for the remaining report branches:

```text
given bmi_tenths 152
and classification underweight
when format_bmi_report is called
then the returned lines are:
Your BMI is 15.2.
You are underweight. You should see your doctor.
```

and

```text
given bmi_tenths 325
and classification overweight
when format_bmi_report is called
then the returned lines are:
Your BMI is 32.5.
You are overweight. You should see your doctor.
```

Suggested generic test names:

```text
format_bmi_report_returns_the_underweight_report
format_bmi_report_returns_the_overweight_report
```

### 17. Green: Make The Remaining Formatter Tests Pass

Make them pass while keeping the earlier tests green.

### 18. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- three functions in one module
- one service object with one calculation method, one classification method, and one formatter method
- one module plus a small BMI result or report record or struct

The important thing is that BMI calculation, range classification, and report formatting stay small, explicit, and directly testable.
