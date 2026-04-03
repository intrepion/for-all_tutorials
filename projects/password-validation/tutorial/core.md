<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Password Validation](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `password-validation` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical invalid example:

```text
given password 12345
when is_known_password is called
then the result is false
```

At this point, `is_known_password` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
is_known_password_returns_false_for_the_canonical_invalid_password
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement formatter behavior before the next tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Canonical Valid Example

Write the next failing test:

```text
given password abc$123
when is_known_password is called
then the result is true
```

Suggested generic test name:

```text
is_known_password_returns_true_for_the_canonical_valid_password
```

### 5. Green: Make The Valid Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Welcome Formatter

Write the next failing test:

```text
given is_valid true
when format_password_validation_message is called
then the returned message is:
Welcome!
```

Suggested generic test name:

```text
format_password_validation_message_returns_the_welcome_sentence
```

### 8. Green: Make The Welcome Formatter Test Pass

Make it pass.

### 9. Refactor

Clean up the implementation while keeping all tests green.

### 10. Red: Add The Rejection Formatter

Write the next failing test:

```text
given is_valid false
when format_password_validation_message is called
then the returned message is:
I don't know you.
```

Suggested generic test name:

```text
format_password_validation_message_returns_the_rejection_sentence
```

### 11. Green: Make The Rejection Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with one validation method and one formatter method
- one module plus a small result record or enum

The important thing is that exact password comparison and message formatting stay small, explicit, and directly testable.
