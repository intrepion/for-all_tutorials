<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Password Generator](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `password-generator` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical generation plan:

```text
given minimum_length 8
and special_character_count 2
and number_count 2
when password_generation_plan is called
then:
total_length is 8
letter_count is 4
special_character_count is 2
number_count is 2
```

At this point, `password_generation_plan` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
password_generation_plan_returns_the_canonical_plan
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement the character pools or formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Length-Growth Case

Write the next failing test:

```text
given minimum_length 3
and special_character_count 2
and number_count 2
when password_generation_plan is called
then:
total_length is 4
letter_count is 0
special_character_count is 2
number_count is 2
```

Suggested generic test name:

```text
password_generation_plan_grows_to_fit_required_non_letter_characters
```

### 5. Green: Make The Length-Growth Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Character Pools

Write failing tests for the three character pools:

```text
when letter_character_pool is called
then the result is the exact ordered lowercase alphabet a through z
```

```text
when digit_character_pool is called
then the result is the exact ordered digits 0 through 9
```

```text
when special_character_pool is called
then the result is the exact ordered characters !, @, #, $, %, ^, &, *, ?
```

Suggested generic test names:

```text
letter_character_pool_returns_the_canonical_lowercase_alphabet
digit_character_pool_returns_the_canonical_digit_set
special_character_pool_returns_the_canonical_special_character_set
```

### 8. Green: Make The Pool Tests Pass

Make them pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all planning and pool tests green.

### 10. Red: Add The Password Formatter

Write the next failing test:

```text
given password aurn2$1s#
when format_generated_password is called
then the returned lines are:
Your password is
aurn2$1s#
```

Suggested generic test name:

```text
format_generated_password_returns_the_canonical_two_line_output
```

### 11. Green: Make The Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- five functions in one module
- one service object with planning, pool, and formatter methods
- one module plus small immutable constant character pools

The important thing is that password planning, canonical pool definition, and output formatting stay small, explicit, and directly testable.
