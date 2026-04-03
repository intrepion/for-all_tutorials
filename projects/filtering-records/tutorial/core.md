<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Filtering Records](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `filtering-records` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical dataset:

```text
when default_employee_records is called
then the result is the exact six canonical employee records in source order
```

At this point, `default_employee_records` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
default_employee_records_returns_the_canonical_employee_dataset
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement filtering, sorting, or formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Search Filter

Write the next failing test:

```text
given the canonical employee dataset
and search string Jac
when filter_employee_records_by_search_string is called
then the returned full names are in this source order:
Jake Jacobson
Jacquelyn Jackson
```

Suggested generic test name:

```text
filter_employee_records_by_search_string_returns_the_canonical_match_set_in_source_order
```

### 5. Green: Make The Filter Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Case-Insensitive Match

Write the next failing test:

```text
given the canonical employee dataset
and search string jac
when filter_employee_records_by_search_string is called
then the returned full names are:
Jake Jacobson
Jacquelyn Jackson
```

Suggested generic test name:

```text
filter_employee_records_by_search_string_matches_case_insensitively
```

### 8. Green: Make The Case-Insensitive Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all dataset and filtering tests green.

### 10. Red: Add The Canonical Filtered Sort Order

Write the next failing test:

```text
given the filtered employee records for search string Jac
when sort_employee_records_by_last_name is called
then the returned full names are in this order:
Jacquelyn Jackson
Jake Jacobson
```

Suggested generic test name:

```text
sort_employee_records_by_last_name_returns_the_canonical_filtered_order
```

### 11. Green: Make The Sort Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor

Clean up the implementation while keeping all dataset, filtering, and sorting tests green.

### 13. Red: Add The Table Formatter

Write the next failing test:

```text
given the canonical sorted filtered employee records
when format_employee_record_table is called
then the returned lines are:
Name | Position | Separation Date
--------------------|-------------------|----------------
Jacquelyn Jackson | DBA | 
Jake Jacobson | Programmer | 
```

Suggested generic test name:

```text
format_employee_record_table_returns_the_canonical_filtered_table
```

### 14. Green: Make The Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 15. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- four functions in one module
- one service object with dataset, filter, sort, and formatter methods
- one module plus a small immutable record list and small helper functions

The important thing is that canonical-record construction, case-insensitive substring filtering, stable last-name sorting, and table formatting stay small, explicit, and directly testable.
