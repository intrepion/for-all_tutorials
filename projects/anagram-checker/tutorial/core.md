<!-- breadcrumbs:start -->
[for-all_tutorials](../../../../README.md) / [Projects](../../../README.md) / [Anagram Checker](../../README.md) / [Tutorial](../README.md) / core.md
<!-- breadcrumbs:end -->

# Core Walkthrough

Project-specific red, green, refactor sequence for the `anagram-checker` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical anagram example:

```text
given first_input "note"
and second_input "tone"
when are_anagrams is called
then the result is true
```

At this point, `are_anagrams` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
are_anagrams_returns_true_for_note_and_tone
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and implement non-anagram or formatter behavior before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Different-Length Failure Case

Write the next failing test:

```text
given first_input "note"
and second_input "tones"
when are_anagrams is called
then the result is false
```

Suggested generic test name:

```text
are_anagrams_returns_false_when_lengths_differ
```

### 5. Green: Make The Length Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Same-Length Non-Anagram Case

Write the next failing test:

```text
given first_input "note"
and second_input "tore"
when are_anagrams is called
then the result is false
```

Suggested generic test name:

```text
are_anagrams_returns_false_when_character_counts_differ
```

### 8. Green: Make The Non-Anagram Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all comparison tests green.

### 10. Red: Prove Repeated Character Counts Matter

Write the next failing test:

```text
given first_input "aab"
and second_input "abb"
when are_anagrams is called
then the result is false
```

Suggested generic test name:

```text
are_anagrams_requires_matching_repeated_character_counts
```

### 11. Green: Make The Repeated-Character Test Pass

Make it pass without collapsing the comparison into a distinct-character set check.

### 12. Refactor

Clean up the implementation while keeping all comparison tests green.

### 13. Red: Prove The Comparison Is Case Sensitive

Write the next failing test:

```text
given first_input "Note"
and second_input "tone"
when are_anagrams is called
then the result is false
```

Suggested generic test name:

```text
are_anagrams_treats_uppercase_and_lowercase_as_different_characters
```

### 14. Green: Make The Case-Sensitivity Test Pass

Make it pass without adding lowercase normalization or other hidden preprocessing.

### 15. Refactor

Clean up the implementation while keeping all comparison tests green.

### 16. Red: Add The Success Formatter

Write the next failing test:

```text
given first_input "note"
and second_input "tone"
and are_anagrams true
when format_anagram_result is called
then the result is:
"note" and "tone" are anagrams.
```

Suggested generic test name:

```text
format_anagram_result_returns_the_canonical_success_sentence
```

### 17. Green: Make The Success Formatter Test Pass

Make it pass.

### 18. Refactor

Clean up the implementation while keeping all tests green.

### 19. Red: Add The Failure Formatter

Write the next failing test:

```text
given first_input "note"
and second_input "tore"
and are_anagrams false
when format_anagram_result is called
then the result is:
"note" and "tore" are not anagrams.
```

Suggested generic test name:

```text
format_anagram_result_returns_the_canonical_failure_sentence
```

### 20. Green: Make The Failure Formatter Test Pass

Make it pass while keeping the earlier tests green.

### 21. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- two functions in one module
- one service object with two methods
- one module plus one helper for character counting
- one static helper

The important thing is that anagram detection and message formatting stay separate, small, and directly testable.
