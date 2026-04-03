<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Who's in Space?](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `whos-in-space` core repo.

### 1. Red: Write The First Failing Test

Start with parsing the canonical sample response:

```text
given the canonical sample response json
when parse_people_in_space_response is called
then the returned report preserves number_of_people and the exact ordered people list
```

At this point, `parse_people_in_space_response` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
parse_people_in_space_response_preserves_the_canonical_people_list
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles the canonical sample parsing case. Do not jump ahead and implement formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Report Formatter

Write the next failing test:

```text
given the canonical parsed people-in-space report
when format_people_in_space_report is called
then the returned lines are:
There are 3 people in space right now:
Name | Craft
--------------------|------
Gennady Padalka | ISS
Mikhail Kornienko | ISS
Scott Kelly | ISS
```

Suggested generic test name:

```text
format_people_in_space_report_returns_the_canonical_table
```

### 5. Green: Make The Formatter Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with parser and formatter methods
- one module plus a small immutable report type

The important thing is that response parsing and table formatting stay small, explicit, and directly testable.
