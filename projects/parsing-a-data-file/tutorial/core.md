<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Parsing a Data File](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `parsing-a-data-file` core repo.

### 1. Red: Write The First Failing Test

Start with parsing the canonical CSV lines:

```text
given the canonical input lines
when parse_employee_salary_records is called
then the returned parsed records preserve the exact source order
and the salary field is converted into whole-number salary_dollars values
```

At this point, `parse_employee_salary_records` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
parse_employee_salary_records_preserves_the_canonical_input_order_and_values
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles the canonical parsing case. Do not jump ahead and implement formatter behavior before that test exists.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Table Formatter

Write the next failing test:

```text
given the canonical parsed employee salary records
when format_employee_salary_table is called
then the returned lines are:
Last     First    Salary
------------------------
Ling     Mai      55900
Johnson  Jim      56500
Jones    Aaron    46000
Jones    Chris    34500
Swift    Geoffrey 14200
Xiong    Fong     65000
Zarnecki Sabrina  51500
```

Suggested generic test name:

```text
format_employee_salary_table_returns_the_canonical_table_output
```

### 5. Green: Make The Formatter Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with parser and formatter methods
- one module plus a small parsed-record type

The important thing is that CSV parsing and fixed-width table formatting stay small, explicit, and directly testable.
