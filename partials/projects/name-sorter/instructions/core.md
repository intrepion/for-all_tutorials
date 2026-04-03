---
partial_kind: core-instructions
project: name-sorter
---

# Core Instructions

Project-specific core instruction fragment for the `name-sorter` core repo.

### 1. Red: Write The First Failing Test

Start with parsing the canonical input lines:

```text
given the canonical input lines
when parse_name_entries is called
then the returned parsed entries preserve the exact source order and exact last and first name values
```

At this point, `parse_name_entries` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
parse_name_entries_preserves_the_canonical_input_order_and_values
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles the canonical parsing case. Do not jump ahead and implement sorting or formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Canonical Sort Order

Write the next failing test:

```text
given the parsed canonical name entries
when sort_name_entries is called
then the returned names are in this order:
Johnson, Jim
Jones, Aaron
Jones, Chris
Ling, Mai
Swift, Geoffrey
Xiong, Fong
Zarnecki, Sabrina
```

Suggested generic test name:

```text
sort_name_entries_returns_the_canonical_last_name_then_first_name_order
```

### 5. Green: Make The Sort Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Non-Mutation Test

Write the next failing test:

```text
given an input list of parsed name entries
when sort_name_entries is called
then the returned list is a new list
and the original input order is unchanged
```

Suggested generic test name:

```text
sort_name_entries_returns_a_new_list_without_mutating_the_input
```

### 8. Green: Make The Non-Mutation Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all parsing and sorting tests green.

### 10. Red: Add The Report Formatter

Write the next failing test:

```text
given the canonical sorted name entries
when format_sorted_name_report is called
then the returned lines are:
Total of 7 names
-----------------
Johnson, Jim
Jones, Aaron
Jones, Chris
Ling, Mai
Swift, Geoffrey
Xiong, Fong
Zarnecki, Sabrina
```

Suggested generic test name:

```text
format_sorted_name_report_returns_the_canonical_output_file_contents
```

### 11. Green: Make The Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- three functions in one module
- one service object with parser, sorter, and formatter methods
- one module plus a small immutable test-fixture helper in the test suite

The important thing is that `Last, First` parsing, alphabetical sorting, and output-file formatting stay small, explicit, and directly testable.

