<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Validating Inputs](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `validating-inputs` core repo.

### 1. Red: Write The First Failing Test

Start with the first-name required-field rule:

```text
given first_name ""
when validate_first_name is called
then the result is:
The first name must be filled in.
```

At this point, `validate_first_name` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
validate_first_name_requires_a_value
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first validation case. Do not jump ahead and implement the other validators or formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The First-Name Length Rule

Write the next failing test:

```text
given first_name "J"
when validate_first_name is called
then the result is:
"J" is not a valid first name. It is too short.
```

Suggested generic test name:

```text
validate_first_name_rejects_one_character_values
```

### 5. Green: Make The First-Name Length Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Last-Name Rules

Write failing tests for the two last-name branches:

```text
given last_name ""
when validate_last_name is called
then the result is:
The last name must be filled in.
```

```text
given last_name "J"
when validate_last_name is called
then the result is:
"J" is not a valid last name. It is too short.
```

Suggested generic test names:

```text
validate_last_name_requires_a_value
validate_last_name_rejects_one_character_values
```

### 8. Green: Make The Last-Name Tests Pass

Make them pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all name-validation tests green.

### 10. Red: Add The ZIP Code Rule

Write the next failing test:

```text
given zip_code "ABCDE"
when validate_zip_code is called
then the result is:
The ZIP code must be numeric.
```

Suggested generic test name:

```text
validate_zip_code_rejects_non_numeric_values
```

### 11. Green: Make The ZIP Code Test Pass

Make it pass.

### 12. Refactor

Clean up the implementation while keeping all current tests green.

### 13. Red: Add The Employee ID Rule

Write failing tests for one invalid and one valid ID:

```text
given employee_id "A12-1234"
when validate_employee_id is called
then the result is:
A12-1234 is not a valid ID.
```

```text
given employee_id "TK-4210"
when validate_employee_id is called
then the result is null
```

Suggested generic test names:

```text
validate_employee_id_rejects_values_that_do_not_match_the_fixed_pattern
validate_employee_id_accepts_two_letters_hyphen_four_digits
```

### 14. Green: Make The Employee ID Tests Pass

Make them pass while keeping the earlier tests green.

### 15. Refactor

Clean up the implementation while keeping all field-validator tests green.

### 16. Red: Add The Error Aggregator

Write the next failing test:

```text
given first_name "J"
and last_name ""
and zip_code "ABCDE"
and employee_id "A12-1234"
when collect_validation_errors is called
then the returned lines are:
"J" is not a valid first name. It is too short.
The last name must be filled in.
The ZIP code must be numeric.
A12-1234 is not a valid ID.
```

Suggested generic test name:

```text
collect_validation_errors_returns_the_canonical_invalid_example_in_order
```

### 17. Green: Make The Aggregator Test Pass

Make it pass without changing the canonical error order.

### 18. Refactor

Clean up the implementation while keeping all tests green.

### 19. Red: Add The Report Formatter

Write failing tests for the two report branches:

```text
given errors []
when format_validation_report is called
then the returned lines are:
There were no errors found.
```

```text
given errors [
  "\"J\" is not a valid first name. It is too short.",
  "The last name must be filled in."
]
when format_validation_report is called
then the returned lines are unchanged
```

Suggested generic test names:

```text
format_validation_report_returns_the_success_line_when_there_are_no_errors
format_validation_report_preserves_existing_error_lines
```

### 20. Green: Make The Formatter Tests Pass

Make them pass while keeping the earlier tests green.

### 21. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- separate validator functions plus one aggregator and one formatter
- one validation service object with several narrow methods
- one module plus small helper functions for pattern checks

The important thing is that field-level validation, error aggregation, and report formatting stay separate, small, and directly testable.
