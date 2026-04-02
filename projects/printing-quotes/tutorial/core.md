<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Printing Quotes](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `printing-quotes` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical example from the project prompt:

```text
given author "Obi-Wan Kenobi"
and quote "These aren't the droids you're looking for."
when format_attributed_quote is called
then the result is "Obi-Wan Kenobi says, \"These aren't the droids you're looking for.\""
```

At this point, `format_attributed_quote` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
format_attributed_quote_formats_author_and_quote
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and generalize further before the next test exists.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Prove That Punctuation Is Not Invented

Write the next failing test:

```text
given author "Ada"
and quote "Hello"
when format_attributed_quote is called
then the result is "Ada says, \"Hello\""
```

Suggested generic test name:

```text
format_attributed_quote_does_not_add_missing_punctuation
```

### 5. Green: Make The Punctuation Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Prove That Apostrophes And Quote Text Are Preserved

Write the next failing test:

```text
given author "Grace Hopper"
and quote "It's easier to ask forgiveness than it is to get permission."
when format_attributed_quote is called
then the result is "Grace Hopper says, \"It's easier to ask forgiveness than it is to get permission.\""
```

Suggested generic test name:

```text
format_attributed_quote_preserves_apostrophes_and_quote_text
```

### 8. Green: Make The Preservation Test Pass

Make it pass without trimming, rewriting, or escaping away the apostrophe in the quote text.

### 9. Refactor

Clean up the implementation while keeping all tests green.

### 10. Red: Prove That Literal Double Quotes Are Always Added

Write the next failing test:

```text
given author "Han Solo"
and quote "Never tell me the odds!"
when format_attributed_quote is called
then the result starts with "Han Solo says, \""
and ends with "\""
```

Suggested generic test name:

```text
format_attributed_quote_wraps_quote_in_literal_double_quotes
```

### 11. Green: Make The Double-Quote Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- one function in one module
- one service object with one method
- one static helper

The important thing is that the formatting rule stays small, explicit, and directly testable.
