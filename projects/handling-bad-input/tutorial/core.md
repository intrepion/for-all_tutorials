<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Handling Bad Input](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `handling-bad-input` core repo.

### 1. Red: Write The First Failing Test

Start with the zero-input validation rule:

```text
given rate_input "0"
when parse_rate_of_return is called
then the result is null
```

At this point, `parse_rate_of_return` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
parse_rate_of_return_rejects_zero
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first validation case. Do not jump ahead and implement calculation or formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Non-Numeric Validation Case

Write the next failing test:

```text
given rate_input "ABC"
when parse_rate_of_return is called
then the result is null
```

Suggested generic test name:

```text
parse_rate_of_return_rejects_non_numeric_input
```

### 5. Green: Make The Non-Numeric Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Valid Parse Case

Write the next failing test:

```text
given rate_input "4"
when parse_rate_of_return is called
then the result is 4
```

Suggested generic test name:

```text
parse_rate_of_return_accepts_positive_whole_numbers
```

### 8. Green: Make The Valid Parse Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all parsing tests green.

### 10. Red: Add The Canonical Rule-of-72 Example

Write the next failing test:

```text
given rate_of_return 4
when calculate_years_to_double is called
then the result is 18
```

Suggested generic test name:

```text
calculate_years_to_double_returns_the_canonical_year_count
```

### 11. Green: Make The Canonical Calculation Test Pass

Make it pass.

### 12. Refactor

Clean up the implementation while keeping all current tests green.

### 13. Red: Add A Half-Up Rounding Case

Write the next failing test:

```text
given rate_of_return 11
when calculate_years_to_double is called
then the result is 7
```

Suggested generic test name:

```text
calculate_years_to_double_rounds_to_the_nearest_whole_year_with_half_up_rounding
```

### 14. Green: Make The Rounding Test Pass

Make it pass while keeping the earlier tests green.

### 15. Refactor

Clean up the implementation while keeping all calculation tests green.

### 16. Red: Add The Invalid-Input Message

Write the next failing test:

```text
when format_invalid_rate_message is called
then the result is:
Sorry. That's not a valid input.
```

Suggested generic test name:

```text
format_invalid_rate_message_returns_the_canonical_sentence
```

### 17. Green: Make The Invalid-Message Test Pass

Make it pass.

### 18. Refactor

Clean up the implementation while keeping all tests green.

### 19. Red: Add The Success Formatter

Write the next failing test:

```text
given years 18
when format_rule_of_72_report is called
then the result is:
It will take 18 years to double your initial investment.
```

Suggested generic test name:

```text
format_rule_of_72_report_returns_the_canonical_sentence
```

### 20. Green: Make The Success Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 21. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- four functions in one module
- one service object with parsing, calculation, and formatter methods
- one module plus a small numeric helper for rounding

The important thing is that input parsing, rule-of-72 calculation, and output formatting stay separate, small, and directly testable.
