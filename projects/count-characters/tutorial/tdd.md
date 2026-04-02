<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Counting the Number of Characters](../../README.md) / [Tutorial](../README.md) / tdd.md
<!-- breadcrumbs:end -->

# TDD Walkthrough

Project-specific red, green, refactor sequence for `count-characters`.

## Core Repo

### 1. Red: Write The First Failing Test

Start with the pure counting function and a simple multi-character input:

```text
given input "Homer"
when count_characters is called
then the result is 5
```

At this point, `count_characters` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
count_characters_returns_count_for_basic_input
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first basic case. Do not jump ahead and implement whitespace or empty-input behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Prove That Leading And Trailing Spaces Count

Write the next failing test:

```text
given input "  hi  "
when count_characters is called
then the result is 6
```

Suggested generic test name:

```text
count_characters_preserves_and_counts_leading_and_trailing_spaces
```

### 5. Green: Make The Spaces Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Whitespace-Only Case

Write the next failing test:

```text
given input "   "
when count_characters is called
then the result is 3
```

Suggested generic test name:

```text
count_characters_counts_whitespace_only_input_without_trimming
```

### 8. Green: Make The Whitespace-Only Test Pass

Make it pass without trimming, collapsing, or otherwise normalizing the input.

### 9. Refactor

Clean up the implementation while keeping all tests green.

### 10. Red: Add The Empty-Input Behavior

Write the next failing test:

```text
given input ""
when count_characters is called
then the result is 0
```

Suggested generic test name:

```text
count_characters_returns_zero_for_empty_input
```

### 11. Green: Make The Empty-Input Test Pass

Make it pass.

### 12. Refactor

Clean up the implementation while keeping all tests green.

### 13. Red: Add The Zero-Count Message

Write the next failing test:

```text
given input ""
and count 0
when format_character_count_message is called
then the result is " has no characters."
```

Suggested generic test name:

```text
format_character_count_message_uses_no_characters_for_zero
```

### 14. Green: Make The Zero-Count Message Pass

Make it pass.

### 15. Refactor

Clean up the implementation while keeping all tests green.

### 16. Red: Add The Singular Message

Write the next failing test:

```text
given input "X"
and count 1
when format_character_count_message is called
then the result is "X has 1 character."
```

Suggested generic test name:

```text
format_character_count_message_uses_singular_for_one
```

### 17. Green: Make The Singular Message Pass

Make it pass.

### 18. Refactor

Clean up the implementation while keeping all tests green.

### 19. Red: Add The Plural Message

Write the next failing test:

```text
given input "Homer"
and count 5
when format_character_count_message is called
then the result is "Homer has 5 characters."
```

Suggested generic test name:

```text
format_character_count_message_uses_plural_for_many
```

### 20. Green: Make The Plural Message Pass

Make it pass.

### 21. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with two methods
- one module
- one static helper

The important thing is that the counting and formatting rules stay separate, small, and directly testable.

## Adapter Repo

### 22. Red: Add An Adapter-Level Test

In a separate adapter repo, add a failing test that proves the chosen adapter delegates to the core logic correctly.

### 23. Green: Add The Thin Surface Adapter

Add the thinnest possible adapter for the surface you are building:

- collect one input string from that surface
- pass that input to `count_characters`
- pass the original input plus the returned count to `format_character_count_message`
- return, render, or print the result in the form that surface requires
- keep transport, parsing, and input/output code out of the core counting logic

For prompt-driven adapters, the prompt should be equivalent to:

```text
What is the input string?
```

The adapter must pass the collected input through exactly as entered, without trimming or normalizing whitespace.

### 24. Refactor

Clean up any remaining duplication while keeping the full suite green.
