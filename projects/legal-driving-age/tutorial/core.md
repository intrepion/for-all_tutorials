<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Legal Driving Age](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `legal-driving-age` core repo.

### 1. Red: Write The First Failing Test

Start with the under-threshold example:

```text
given age 15
when is_old_enough_to_drive is called
then the result is false
```

At this point, `is_old_enough_to_drive` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
is_old_enough_to_drive_returns_false_below_the_legal_threshold
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Exact Threshold Case

Write the next failing test:

```text
given age 16
when is_old_enough_to_drive is called
then the result is true
```

Suggested generic test name:

```text
is_old_enough_to_drive_returns_true_at_the_legal_threshold
```

### 5. Green: Make The Threshold Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add An Above-Threshold Case

Write the next failing test:

```text
given age 35
when is_old_enough_to_drive is called
then the result is true
```

Suggested generic test name:

```text
is_old_enough_to_drive_returns_true_above_the_legal_threshold
```

### 8. Green: Make The Above-Threshold Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all comparison tests green.

### 10. Red: Add The Underage Formatter

Write the next failing test:

```text
given is_old_enough false
when format_driving_eligibility_message is called
then the returned message is:
You are not old enough to legally drive.
```

Suggested generic test name:

```text
format_driving_eligibility_message_returns_the_underage_sentence
```

### 11. Green: Make The Underage Formatter Test Pass

Make it pass.

### 12. Refactor

Clean up the implementation while keeping all tests green.

### 13. Red: Add The Eligible Formatter

Write the next failing test:

```text
given is_old_enough true
when format_driving_eligibility_message is called
then the returned message is:
You are old enough to legally drive.
```

Suggested generic test name:

```text
format_driving_eligibility_message_returns_the_eligible_sentence
```

### 14. Green: Make The Eligible Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 15. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one comparison method and one formatter method
- one module plus a small eligibility record or struct

The important thing is that threshold comparison and message formatting stay small, explicit, and directly testable.
