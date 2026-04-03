---
partial_kind: core-instructions
project: employee-list-removal
---

# Core Instructions

Project-specific core instruction fragment for the `employee-list-removal` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical employee list:

```text
when default_employee_names is called
then the result is this exact ordered list:
John Smith
Jackie Jackson
Chris Jones
Amanda Cullen
Jeremy Goodwin
```

At this point, `default_employee_names` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
default_employee_names_returns_the_canonical_five_name_list
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement removal or formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Canonical Removal Case

Write the next failing test:

```text
given the canonical employee list
and employee_name_to_remove Chris Jones
when remove_employee_by_exact_name is called
then the result is this exact ordered list:
John Smith
Jackie Jackson
Amanda Cullen
Jeremy Goodwin
```

Suggested generic test name:

```text
remove_employee_by_exact_name_removes_chris_jones_from_the_canonical_list
```

### 5. Green: Make The Removal Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Non-Matching Removal Case

Write the next failing test:

```text
given the canonical employee list
and employee_name_to_remove Chris
when remove_employee_by_exact_name is called
then the result is the unchanged canonical list
```

Suggested generic test name:

```text
remove_employee_by_exact_name_preserves_the_list_for_a_non_matching_name
```

### 8. Green: Make The Non-Matching Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all removal tests green.

### 10. Red: Add The Non-Mutation Test

Write the next failing test:

```text
given an input list of employee names
when remove_employee_by_exact_name is called
then the returned list is a new list
and the original input list is unchanged
```

Suggested generic test name:

```text
remove_employee_by_exact_name_returns_a_new_list_without_mutating_the_input
```

### 11. Green: Make The Non-Mutation Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor

Clean up the implementation while keeping all list-construction and removal tests green.

### 13. Red: Add The List Formatter

Write the next failing test:

```text
given employee_names:
John Smith
Jackie Jackson
Amanda Cullen
Jeremy Goodwin
when format_employee_list is called
then the returned lines are:
There are 4 employees:
John Smith
Jackie Jackson
Amanda Cullen
Jeremy Goodwin
```

Suggested generic test name:

```text
format_employee_list_returns_the_canonical_header_and_ordered_names
```

### 14. Green: Make The Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 15. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- three functions in one module
- one service object with list, removal, and formatter methods
- one module plus a small immutable constant list and one filtering helper

The important thing is that canonical-list creation, exact-match removal, and list formatting stay small, explicit, and directly testable.

