---
partial_kind: core-instructions
project: word-finder
---

# Core Instructions

Project-specific core instruction fragment for the `word-finder` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical two-line example:

```text
given the canonical input text
when replace_exact_word_utilize_with_use is called
then the returned text matches the canonical output text exactly
```

At this point, `replace_exact_word_utilize_with_use` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
replace_exact_word_utilize_with_use_transforms_the_canonical_example
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles the canonical example. Do not jump ahead and implement the boundary cases before those tests exist.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Add The Quoted-Word Case

Write the next failing test:

```text
given "utilize"
when replace_exact_word_utilize_with_use is called
then the result is "use"
```

Suggested generic test name:

```text
replace_exact_word_utilize_with_use_replaces_the_quoted_word_case
```

### 5. Green: Make The Quoted-Word Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Add The Case-Sensitivity Rule

Write the next failing test:

```text
given Utilize utilize
when replace_exact_word_utilize_with_use is called
then the result is Utilize use
```

Suggested generic test name:

```text
replace_exact_word_utilize_with_use_is_case_sensitive
```

### 8. Green: Make The Case-Sensitivity Test Pass

Make it pass while keeping the earlier tests green.

### 9. Refactor

Clean up the implementation while keeping all replacement tests green.

### 10. Red: Add The Whole-Word Boundary Rule

Write the next failing test:

```text
given utilize utilized disutilize
when replace_exact_word_utilize_with_use is called
then the result is use utilized disutilize
```

Suggested generic test name:

```text
replace_exact_word_utilize_with_use_only_replaces_whole_word_occurrences
```

### 11. Green: Make The Whole-Word Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- one function in one module
- one service object with a single replacement method
- one module plus a small helper for word-boundary detection

The important thing is that the exact `utilize -> use` replacement rule stays small, explicit, and directly testable.

