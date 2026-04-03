---
partial_kind: core-instructions
project: sorting-records
---

# Core Instructions

Project-specific core instruction fragment for the `sorting-records` core repo.

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

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement sorting or formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Canonical Sort Order

Write the next failing test:

```text
given the canonical employee dataset
when sort_employee_records_by_last_name is called
then the returned full names are in this order:
Jacquelyn Jackson
Jake Jacobson
John Johnson
Michaela Michaelson
Sally Weber
Tou Xiong
```

Suggested generic test name:

```text
sort_employee_records_by_last_name_returns_the_canonical_order
```

### 5. Green: Make The Sort Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Non-Mutation Test

Write the next failing test:

```text
given an input list of employee records
when sort_employee_records_by_last_name is called
then the returned list is a new list
and the original input order is unchanged
```

Suggested generic test name:

```text
sort_employee_records_by_last_name_returns_a_new_list_without_mutating_the_input
```

### 8. Green: Make The Non-Mutation Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all dataset and sorting tests green.

### 10. Red: Add The Table Formatter

Write the next failing test:

```text
given the canonical sorted employee records
when format_employee_record_table is called
then the returned lines are:
Name | Position | Separation Date
--------------------|-------------------|----------------
Jacquelyn Jackson | DBA | 
Jake Jacobson | Programmer | 
John Johnson | Manager | 2016-12-31
Michaela Michaelson | District Manager | 2015-12-19
Sally Weber | Web Developer | 2015-12-18
Tou Xiong | Software Engineer | 2016-10-05
```

Suggested generic test name:

```text
format_employee_record_table_returns_the_canonical_sorted_table
```

### 11. Green: Make The Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- three functions in one module
- one service object with dataset, sorting, and formatter methods
- one module plus a small immutable record list and a sort helper

The important thing is that canonical-record construction, stable last-name sorting, and table formatting stay small, explicit, and directly testable.

