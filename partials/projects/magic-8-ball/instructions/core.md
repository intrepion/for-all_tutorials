---
partial_kind: core-instructions
project: magic-8-ball
---

# Core Instructions

Project-specific core instruction fragment for the `magic-8-ball` core repo.

### 1. Red: Write The First Failing Test

Start with the response-count contract:

```text
when magic_8_ball_response_count is called
then the result is 4
```

At this point, `magic_8_ball_response_count` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
magic_8_ball_response_count_returns_four
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement response lookup before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The First Two Response Mappings

Write failing tests for the first two choices:

```text
given choice_number 1
when magic_8_ball_response is called
then the result is Yes
```

```text
given choice_number 2
when magic_8_ball_response is called
then the result is No
```

Suggested generic test names:

```text
magic_8_ball_response_returns_yes_for_choice_one
magic_8_ball_response_returns_no_for_choice_two
```

### 5. Green: Make The First Mapping Tests Pass

Make the smallest safe change that gets all existing tests green.

### 6. Refactor

Clean up the implementation while keeping the existing tests green.

### 7. Red: Add The Remaining Response Mappings

Write failing tests for the remaining choices:

```text
given choice_number 3
when magic_8_ball_response is called
then the result is Maybe
```

```text
given choice_number 4
when magic_8_ball_response is called
then the result is Ask again later.
```

Suggested generic test names:

```text
magic_8_ball_response_returns_maybe_for_choice_three
magic_8_ball_response_returns_ask_again_later_for_choice_four
```

### 8. Green: Make The Remaining Mapping Tests Pass

Make them pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all response-mapping tests green.

### 10. Red: Add The Out-Of-Range Case

Write the next failing test:

```text
given choice_number 5
when magic_8_ball_response is called
then the result is null
```

Suggested generic test name:

```text
magic_8_ball_response_returns_null_for_out_of_range_choices
```

### 11. Green: Make The Out-Of-Range Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one count method and one lookup method
- one module plus a small constant array or list of canonical responses

The important thing is that response count and response lookup stay small, explicit, and directly testable.

