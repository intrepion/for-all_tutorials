<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Word Frequency Finder](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `word-frequency-finder` core repo.

### 1. Red: Write The First Failing Test

Start with tokenizing the canonical example text:

```text
given the canonical input text
when split_text_into_words is called
then the returned words preserve the exact source order across spaces and newlines
```

At this point, `split_text_into_words` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
split_text_into_words_preserves_the_canonical_word_order
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles the canonical tokenization case. Do not jump ahead and implement counting or formatting before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Frequency Counter

Write the next failing test:

```text
given the words:
badger
badger
mushroom
snake
badger
when count_word_frequencies is called
then the returned entries are:
badger with count 3
mushroom with count 1
snake with count 1
```

Suggested generic test name:

```text
count_word_frequencies_returns_counts_in_first_appearance_order
```

### 5. Green: Make The Counter Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Histogram Formatter

Write the next failing test:

```text
given the entries:
badger with count 3
snake with count 1
when format_word_frequency_histogram is called
then the returned lines are:
badger: ***
snake: *
```

Suggested generic test name:

```text
format_word_frequency_histogram_returns_the_canonical_line_format
```

### 8. Green: Make The Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all tokenization, counting, and formatting tests green.

### 10. Red: Add The Full Canonical Example

Write the next failing test:

```text
given the canonical input text
when split_text_into_words, count_word_frequencies, and format_word_frequency_histogram are applied in sequence
then the returned lines are:
badger: *******
mushroom: **
snake: *
```

Suggested generic test name:

```text
the_full_canonical_example_produces_the_canonical_histogram
```

### 11. Green: Make The Full Example Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- three functions in one module
- one service object with tokenizer, counter, and formatter methods
- one module plus a small immutable frequency-entry type

The important thing is that tokenization, exact frequency counting, and histogram formatting stay small, explicit, and directly testable.
