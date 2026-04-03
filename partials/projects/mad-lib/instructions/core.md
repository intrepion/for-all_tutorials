---
partial_kind: core-instructions
project: mad-lib
---

# Core Instructions

Project-specific core instruction fragment for the `mad-lib` core repo.

### 1. Red: Write The First Failing Test

Start with the canonical example from the project prompt:

```text
given noun "dog"
and verb "walk"
and adjective "blue"
and adverb "quickly"
when format_mad_lib is called
then the result is "Do you walk your blue dog quickly? That's hilarious!"
```

At this point, `format_mad_lib` may not exist yet. That is fine. The first run should fail either at compile time or at test execution time.

Suggested generic test name:

```text
format_mad_lib_formats_the_canonical_example
```

### 2. Green: Make The First Test Pass

Create the smallest production code that makes the first test pass.

At this stage, it is acceptable if the code only handles this first case. Do not jump ahead and generalize further before the next test exists.

### 3. Refactor

Do the smallest cleanup that keeps the first test green.

### 4. Red: Prove The Words Land In The Correct Positions

Write the next failing test:

```text
given noun "cat"
and verb "juggle"
and adjective "tiny"
and adverb "slowly"
when format_mad_lib is called
then the result is "Do you juggle your tiny cat slowly? That's hilarious!"
```

Suggested generic test name:

```text
format_mad_lib_places_each_word_in_the_correct_slot
```

### 5. Green: Make The Position Test Pass

Make the smallest safe change that gets both tests green.

### 6. Refactor

Clean up the implementation while keeping both tests green.

### 7. Red: Prove That Multiword And Punctuated Input Is Preserved

Write the next failing test:

```text
given noun "space hamster"
and verb "admire"
and adjective "neon-green"
and adverb "very carefully"
when format_mad_lib is called
then the result is "Do you admire your neon-green space hamster very carefully? That's hilarious!"
```

Suggested generic test name:

```text
format_mad_lib_preserves_multiword_and_punctuated_input
```

### 8. Green: Make The Preservation Test Pass

Make it pass without trimming, rewriting, or normalizing the words.

### 9. Refactor

Clean up the implementation while keeping all tests green.

### 10. Red: Prove That The Fixed Ending Is Always Present

Write the next failing test:

```text
given noun "robot"
and verb "paint"
and adjective "silver"
and adverb "badly"
when format_mad_lib is called
then the result ends with "? That's hilarious!"
```

Suggested generic test name:

```text
format_mad_lib_preserves_the_fixed_story_ending
```

### 11. Green: Make The Ending Test Pass

Make it pass while keeping the earlier tests green.

### 12. Refactor The Core Logic

Now that the full behavior is covered, refactor toward a small, clear core abstraction.

The exact shape can vary by stack:

- one function in one module
- one service object with one method
- one static helper

The important thing is that the story-template rule stays small, explicit, and directly testable.

