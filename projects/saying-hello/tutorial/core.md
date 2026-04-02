<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Saying Hello](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `saying-hello` core repo.

### 1. Red: Write The First Failing Test

Start with the happiest path:

```text
given name "Ada"
when greet is called
then the result is "Hello, Ada!"
```

At this point, `greet` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
greet_returns_personalized_greeting_for_non_empty_name
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles the first case. Do not jump ahead and implement trimming or empty-input behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

Do not broaden the behavior yet.

### 4. Red: Add The Trimming Behavior

Write the next failing test:

```text
given name "  Ada  "
when greet is called
then the result is "Hello, Ada!"
```

Suggested generic test name:

```text
greet_trims_leading_and_trailing_whitespace
```

### 5. Green: Make The Trimming Test Pass

Make the trimming test pass with the smallest safe change.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Empty-String Behavior

Write the next failing test:

```text
given name ""
when greet is called
then the result is "Hello!"
```

Suggested generic test name:

```text
greet_returns_generic_greeting_for_empty_string
```

### 8. Green: Make The Empty-String Test Pass

Make it pass.

### 9. Refactor

Clean up the implementation while keeping all tests green.

### 10. Red: Add The Whitespace-Only Behavior

Write the next failing test:

```text
given name "   "
when greet is called
then the result is "Hello!"
```

Suggested generic test name:

```text
greet_returns_generic_greeting_for_whitespace_only_input
```

### 11. Green: Make The Whitespace-Only Test Pass

Make it pass.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- one function
- one service object
- one module
- one static helper

The important thing is that the greeting rules live in one small unit that can be tested directly.
