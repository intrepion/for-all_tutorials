<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Computing Statistics](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `computing-statistics` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical mean calculation:

```text
given response_times_ms:
100
200
1000
300
when mean_hundredths is called
then the result is 40000
```

At this point, `mean_hundredths` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
mean_hundredths_returns_the_canonical_average
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement the other statistics or formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add Minimum And Maximum

Write failing tests for the minimum and maximum:

```text
given response_times_ms:
100
200
1000
300
when minimum_response_time is called
then the result is 100
```

```text
given response_times_ms:
100
200
1000
300
when maximum_response_time is called
then the result is 1000
```

Suggested generic test names:

```text
minimum_response_time_returns_the_canonical_minimum
maximum_response_time_returns_the_canonical_maximum
```

### 5. Green: Make The Minimum And Maximum Tests Pass

Make the smallest safe change that gets all existing tests green.

### 6. Refactor

Clean up the implementation while keeping the existing tests green.

### 7. Red: Add The Canonical Standard Deviation

Write the next failing test:

```text
given response_times_ms:
100
200
1000
300
when standard_deviation_hundredths is called
then the result is 35355
```

Suggested generic test name:

```text
standard_deviation_hundredths_returns_the_canonical_population_standard_deviation
```

### 8. Green: Make The Standard Deviation Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all calculation tests green.

### 10. Red: Add The Statistics Formatter

Write the next failing test:

```text
given response_times_ms:
100
200
1000
300
and mean_hundredths 40000
and minimum_ms 100
and maximum_ms 1000
and standard_deviation_hundredths 35355
when format_statistics_report is called
then the returned lines are:
Numbers: 100, 200, 1000, 300
The average is 400.
The minimum is 100.
The maximum is 1000.
The standard deviation is 353.55.
```

Suggested generic test name:

```text
format_statistics_report_returns_the_canonical_report
```

### 11. Green: Make The Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- five functions in one module
- one service object with four statistics methods and one formatter method
- one module plus a small helper for rounded hundredths rendering

The important thing is that mean, minimum, maximum, standard deviation, and report formatting stay small, explicit, and directly testable.
