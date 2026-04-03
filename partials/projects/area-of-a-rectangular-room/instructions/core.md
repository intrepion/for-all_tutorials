---
partial_kind: core-instructions
project: area-of-a-rectangular-room
---

# Core Instructions

Project-specific core instruction fragment for the `area-of-a-rectangular-room` core repo.

### 1. Red: Write The First Failing Test

Start with the square-feet calculation from the canonical example:

```text
given length 15 feet
and width 20 feet
when calculate_room_area is called
then square_feet is 300
```

At this point, `calculate_room_area` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
calculate_room_area_returns_square_feet_for_canonical_example
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement the conversion or report formatting before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Square-Meter Conversion

Write the next failing test:

```text
given length 15 feet
and width 20 feet
when calculate_room_area is called
then square_meters is 27.870912
```

Suggested generic test name:

```text
calculate_room_area_converts_square_feet_to_square_meters_with_the_exact_constant
```

### 5. Green: Make The Conversion Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Prove Another Rectangle Works

Write the next failing test:

```text
given length 10 feet
and width 12 feet
when calculate_room_area is called
then square_feet is 120
and square_meters is 11.1483648
```

Suggested generic test name:

```text
calculate_room_area_handles_a_second_rectangle
```

### 8. Green: Make The Second Rectangle Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all calculation tests green.

### 10. Red: Add The Report Formatter

Write the next failing test:

```text
given length 15 feet
and width 20 feet
and area square_feet 300, square_meters 27.870912
when format_room_area_report is called
then the returned lines are:
You entered dimensions of 15 feet by 20 feet.
The area is
300 square feet
27.871 square meters
```

Suggested generic test name:

```text
format_room_area_report_returns_the_canonical_four_line_report
```

### 11. Green: Make The Formatter Test Pass

Make it pass.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one calculation method and one formatter method
- one module plus a small result record or struct

The important thing is that area calculation, conversion, and report formatting stay small, explicit, and directly testable.

